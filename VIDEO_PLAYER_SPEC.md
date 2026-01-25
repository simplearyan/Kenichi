# Video Player & Preview Architecture

## Overview

This document specifies the video player architecture for Kenichi, including dual-view logic (Source vs Timeline preview), aspect ratio handling, and performance optimizations.

**Reference Files**:
- `guide/The Core Preview Logic.md`
- `guide/The Dual-Source Engine Logic.md`
- `guide/The Two Logic States Fit vs Fill.md`

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Svelte Frontend                           │
│  ┌──────────────┐           ┌──────────────┐               │
│  │ Source View  │           │ Timeline View│               │
│  │  (Media Lib) │           │  (Editing)   │               │
│  └──────┬───────┘           └──────┬───────┘               │
│         │                          │                        │
│         └──────────┬───────────────┘                        │
│                    │ invoke('switch_context')               │
└────────────────────┼──────────────────────────────────────────┘
                     │
┌────────────────────▼──────────────────────────────────────────┐
│                 Rust Backend (Tauri)                          │
│  ┌──────────────────────────────────────────────────────┐   │
│  │           KinetixEngine (State Manager)              │   │
│  │  ┌────────────────┐    ┌────────────────┐           │   │
│  │  │ PreviewMode    │    │ Decoder Pool   │           │   │
│  │  │ - Timeline     │    │ HashMap<AssetId│           │   │
│  │  │ - Source(id)   │    │  FFmpegDecoder>│           │   │
│  │  └────────┬───────┘    └────────┬───────┘           │   │
│  │           │                     │                    │   │
│  │           └──────┬──────────────┘                    │   │
│  │                  │ get_next_frame()                  │   │
│  └──────────────────┼───────────────────────────────────┘   │
│                     │                                        │
│  ┌──────────────────▼───────────────────────────────────┐   │
│  │              WGPU Renderer                           │   │
│  │  - Single shared texture                             │   │
│  │  - Letterbox/Fill shader                             │   │
│  │  - 60 FPS render loop                                │   │
│  └──────────────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────────────────┘
```

---

## 1. Dual-View System

### 1.1 Source Preview vs Timeline Preview

**Source Preview**:
- **Purpose**: Preview individual clips from media library
- **Behavior**: Isolated, doesn't affect timeline
- **Use Case**: Scrubbing through raw footage to find good moments

**Timeline Preview**:
- **Purpose**: Preview the edited sequence
- **Behavior**: Renders composition of all clips, effects, transitions
- **Use Case**: See the final result while editing

### 1.2 Switching Logic

**❌ BAD: Page/Route Switch**
```svelte
<!-- DON'T DO THIS -->
{#if mode === 'source'}
  <SourcePreview />
{:else}
  <TimelinePreview />
{/if}
```
**Problems**: Destroys WGPU instance, 200-500ms delay, flickers

**✅ GOOD: Engine Context Swap**
```rust
// src-tauri/src/engine/mod.rs
pub enum PreviewMode {
    Timeline,
    Source(AssetId),
}

pub struct KinetixEngine {
    current_mode: PreviewMode,
    decoders: HashMap<AssetId, FFmpegDecoder>,
    // Single WGPU instance shared by both modes
}

impl KinetixEngine {
    fn get_next_frame(&mut self) -> RawPixels {
        match self.current_mode {
            PreviewMode::Timeline => self.resolve_timeline_frame(),
            PreviewMode::Source(id) => {
                self.decoders
                    .get_mut(&id)
                    .unwrap()
                    .next_frame()
            }
        }
    }
}
```

**Frontend**:
```typescript
// src/lib/stores/preview.ts
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export type PreviewMode = 'timeline' | 'source';
export const previewMode = writable<PreviewMode>('timeline');

export async function switchToSource(assetId: string) {
    await invoke('switch_context', { mode: 'source', assetId });
    previewMode.set('source');
}

export async function switchToTimeline() {
    await invoke('switch_context', { mode: 'timeline' });
    previewMode.set('timeline');
}
```

### 1.3 Warm Decoder Pool

**Strategy**: Keep FFmpeg decoders "warm" (paused but in memory)

**Benefits**:
- Instant switching (no 200-500ms file open delay)
- Shared GPU texture (no VRAM reallocation)
- Smooth user experience

**Implementation**:
```rust
// Keep recently accessed decoders in memory
const MAX_WARM_DECODERS: usize = 5;

impl KinetixEngine {
    fn get_or_create_decoder(&mut self, asset_id: AssetId) -> &mut FFmpegDecoder {
        if !self.decoders.contains_key(&asset_id) {
            // Evict oldest if pool is full
            if self.decoders.len() >= MAX_WARM_DECODERS {
                let oldest = self.find_oldest_decoder();
                self.decoders.remove(&oldest);
            }
            
            let decoder = FFmpegDecoder::new(&asset_id.path).unwrap();
            self.decoders.insert(asset_id, decoder);
        }
        
        self.decoders.get_mut(&asset_id).unwrap()
    }
}
```

---

## 2. Asset Type Handling

### 2.1 Video Clips

**Preview Logic**: Frame seeking

**Backend Processing**:
1. FFmpeg seeks to nearest keyframe
2. Decodes forward to exact timestamp
3. Converts to RGBA
4. Uploads to WGPU texture

**Frontend Request**:
```typescript
await invoke('preview_seek', { assetId: 'video_01', time: 12.5 });
```

### 2.2 Audio Clips

**Preview Logic**: Waveform rendering

**Backend Processing**:
1. Analyze audio file
2. Generate 1D array of peaks
3. Send to frontend

**Frontend Rendering**:
```svelte
<canvas bind:this={waveformCanvas} />

<script>
  function renderWaveform(peaks: number[]) {
    const ctx = waveformCanvas.getContext('2d');
    peaks.forEach((peak, i) => {
      const x = (i / peaks.length) * width;
      const h = peak * height;
      ctx.fillRect(x, height/2 - h/2, 2, h);
    });
  }
</script>
```

### 2.3 Text Animations

**Preview Logic**: Procedural generation

**Backend Processing**:
1. Calculate animation state at current time
2. Tessellate glyphs to triangles
3. Render via WGPU

**Math Example**:
```rust
// Fade In over 1.0s
let alpha = if time < 1.0 {
    time / 1.0  // 0.0 to 1.0
} else {
    1.0
};

// Send alpha as uniform to shader
```

### 2.4 Effect Presets

**Preview Logic**: Shader simulation

**Backend Processing**:
1. Apply preset's `.wgsl` shader
2. Use static sample image
3. Render result

---

## 3. Aspect Ratio Handling

### 3.1 Fit vs Fill Logic

**Reference**: `guide/The Two Logic States Fit vs Fill.md`

| Mode | Visual Result | Math Logic |
|---|---|---|
| **Fit (Letterbox)** | Whole video visible with black bars | Scale until width matches |
| **Fill (Zoom)** | Video fills screen, sides cropped | Scale until height matches |

### 3.2 Scaling Math

**Example**: 16:9 video → 9:16 canvas

```
Source Aspect: 16/9 = 1.778
Target Aspect: 9/16 = 0.5625

Zoom Factor = Source / Target = 1.778 / 0.5625 = 3.16x
```

**WGPU Shader**:
```wgsl
struct Transform {
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
}

@group(0) @binding(1) var<uniform> u_transform: Transform;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply zoom and offset
    let uv = (in.tex_coords - 0.5) * u_transform.zoom + 0.5;
    let uv_offset = uv + vec2(u_transform.offset_x, u_transform.offset_y);
    
    // Sample texture
    return textureSample(t_video, s_video, uv_offset);
}
```

### 3.3 Manual Reframe (Pan)

**User Interaction**:
1. User drags video in preview
2. Svelte calculates X/Y offset (-0.2 to +0.2)
3. Send to Rust backend
4. Update `u_transform` uniform
5. Shader shifts texture lookup

**Implementation**:
```svelte
<script>
  let offsetX = 0;
  let offsetY = 0;
  
  function handleDrag(e: MouseEvent) {
    const deltaX = e.movementX / canvasWidth;
    const deltaY = e.movementY / canvasHeight;
    
    offsetX = Math.max(-0.5, Math.min(0.5, offsetX + deltaX));
    offsetY = Math.max(-0.5, Math.min(0.5, offsetY + deltaY));
    
    invoke('update_transform', { offsetX, offsetY });
  }
</script>

<div 
  class="video-preview"
  on:mousedown={startDrag}
  on:mousemove={handleDrag}
/>
```

### 3.4 Aspect Ratio Switching

**Smooth Transition**:
```svelte
<script>
  let aspectRatio = '16:9';
  
  async function changeAspectRatio(newRatio: string) {
    aspectRatio = newRatio;
    await invoke('set_aspect_ratio', { ratio: newRatio });
  }
</script>

<div 
  class="preview-container"
  style="aspect-ratio: {aspectRatio}; transition: aspect-ratio 0.3s ease"
>
  <canvas id="wgpu-canvas" />
</div>
```

**Backend**:
```rust
#[tauri::command]
fn set_aspect_ratio(ratio: String) -> Result<()> {
    let (width, height) = parse_aspect_ratio(&ratio)?;
    
    // Update projection matrix
    engine.update_projection(width, height);
    
    // Don't resize WGPU surface (expensive)
    // Instead, update viewport/scissor rect
    Ok(())
}
```

---

## 4. Performance Optimizations

### 4.1 Thumbnail Cache

**Strategy**: Pre-generate JPG thumbnails (1 per second)

**Benefits**:
- Fast scrubbing (show JPG instead of decoding)
- Only use real decoder when user stops

**Implementation**:
```rust
// On import
fn generate_thumbnails(video_path: &str) -> Result<()> {
    let cache_dir = get_cache_dir(video_path);
    
    for second in 0..duration_seconds {
        let frame = decoder.seek_and_decode(second as f64)?;
        let jpg = encode_jpeg(&frame, 50); // 50% quality
        save_to_cache(&cache_dir, second, &jpg)?;
    }
    
    Ok(())
}
```

**Frontend**:
```typescript
let isScrubbing = false;
let scrubTimeout: number;

function handleScrub(time: number) {
    isScrubbing = true;
    clearTimeout(scrubTimeout);
    
    // Show thumbnail while scrubbing
    showThumbnail(Math.floor(time));
    
    // Decode real frame after 200ms of no movement
    scrubTimeout = setTimeout(() => {
        isScrubbing = false;
        invoke('preview_seek', { time });
    }, 200);
}
```

### 4.2 YUV to RGBA on GPU

**Problem**: CPU conversion is slow (30-50ms on GT 740)

**Solution**: Send raw YUV planes to GPU, convert in shader

**Shader**:
```wgsl
@group(0) @binding(0) var t_y: texture_2d<f32>;
@group(0) @binding(1) var t_u: texture_2d<f32>;
@group(0) @binding(2) var t_v: texture_2d<f32>;
@group(0) @binding(3) var s_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let y = textureSample(t_y, s_sampler, in.tex_coords).r;
    let u = textureSample(t_u, s_sampler, in.tex_coords).r - 0.5;
    let v = textureSample(t_v, s_sampler, in.tex_coords).r - 0.5;
    
    // YUV to RGB conversion (BT.709)
    let r = y + 1.5748 * v;
    let g = y - 0.1873 * u - 0.4681 * v;
    let b = y + 1.8556 * u;
    
    return vec4(r, g, b, 1.0);
}
```

**Performance**: 3-5x faster than CPU conversion

### 4.3 Shared Texture Memory

**Strategy**: Use same GPU memory for both Source and Timeline views

**Benefits**:
- No VRAM reallocation
- Instant switching
- Lower memory usage

**Implementation**:
```rust
pub struct KinetixEngine {
    // Single texture shared by both modes
    video_texture: Option<wgpu::Texture>,
}

impl KinetixEngine {
    fn update_texture(&mut self, frame: &[u8]) {
        // Reuse existing texture
        self.queue.write_texture(
            self.video_texture.as_ref().unwrap().as_image_copy(),
            frame,
            wgpu::ImageDataLayout { /* ... */ },
            texture_size,
        );
    }
}
```

### 4.4 Resolution Toggle

**UI Control**:
```svelte
<select bind:value={previewQuality}>
  <option value="full">Full (1080p)</option>
  <option value="half">Half (540p)</option>
  <option value="quarter">Quarter (270p)</option>
</select>
```

**Backend**:
```rust
#[tauri::command]
fn set_preview_quality(quality: String) -> Result<()> {
    let scale = match quality.as_str() {
        "full" => 1.0,
        "half" => 0.5,
        "quarter" => 0.25,
        _ => 1.0,
    };
    
    engine.set_decode_scale(scale);
    Ok(())
}
```

---

## 5. Export Pipeline

### 5.1 Crop Command (FFmpeg)

**16:9 to 9:16 Conversion**:
```bash
# crop=width:height:x_offset:y_offset
ffmpeg -i input.mp4 \
  -vf "crop=ih*(9/16):ih:(iw-ow)/2:0" \
  output_short.mp4
```

**With Manual Offset**:
```bash
# User dragged video 20% to the left
ffmpeg -i input.mp4 \
  -vf "crop=ih*(9/16):ih:(iw-ow)/2+iw*0.2:0" \
  output_short.mp4
```

### 5.2 Export from Preview State

**DON'T**: Export WGPU pixels (might be lower resolution)

**DO**: Send transform data to FFmpeg

```rust
#[tauri::command]
fn export_video(
    input_path: String,
    output_path: String,
    transform: Transform,
) -> Result<()> {
    let crop_filter = format!(
        "crop={}:{}:{}:{}",
        transform.crop_width,
        transform.crop_height,
        transform.offset_x,
        transform.offset_y,
    );
    
    Command::new("ffmpeg")
        .args(["-i", &input_path])
        .args(["-vf", &crop_filter])
        .arg(&output_path)
        .spawn()?;
    
    Ok(())
}
```

---

## 6. Implementation Checklist

### Phase 4.5 (Audio)
- [ ] Implement audio waveform generation
- [ ] Add audio preview to Source view
- [ ] Sync audio with video playback

### Phase 5a (Timeline Backend)
- [ ] Implement `PreviewMode` enum
- [ ] Create decoder pool (HashMap)
- [ ] Add `switch_context` command

### Phase 5b (Multi-Layer Rendering)
- [ ] Implement `resolve_timeline_frame()`
- [ ] Add multi-clip composition
- [ ] Handle transitions between clips

### Phase 5c (Timeline UI)
- [ ] Add Source/Timeline toggle button
- [ ] Show current mode in UI
- [ ] Implement smooth switching animation

### Phase 7 (Effects)
- [ ] Add Fit/Fill toggle
- [ ] Implement manual reframe (drag to pan)
- [ ] Add aspect ratio selector
- [ ] Create letterbox shader

### Phase 9 (Optimization)
- [ ] Implement thumbnail cache
- [ ] Add YUV to RGBA shader
- [ ] Create resolution quality toggle
- [ ] Optimize decoder pool eviction

---

## 7. API Reference

### Tauri Commands

```rust
#[tauri::command]
fn switch_context(mode: String, asset_id: Option<String>) -> Result<()>

#[tauri::command]
fn preview_seek(asset_id: String, time: f64) -> Result<()>

#[tauri::command]
fn set_aspect_ratio(ratio: String) -> Result<()>

#[tauri::command]
fn update_transform(offset_x: f32, offset_y: f32, zoom: f32) -> Result<()>

#[tauri::command]
fn set_preview_quality(quality: String) -> Result<()>
```

### Svelte Stores

```typescript
// src/lib/stores/preview.ts
export const previewMode: Writable<'timeline' | 'source'>
export const aspectRatio: Writable<string>  // '16:9', '9:16', '1:1'
export const previewQuality: Writable<'full' | 'half' | 'quarter'>
export const transform: Writable<{ zoom: number, offsetX: number, offsetY: number }>
```

---

## References

- `guide/The Core Preview Logic.md` - Asset type handling
- `guide/The Dual-Source Engine Logic.md` - Source/Timeline switching
- `guide/The Two Logic States Fit vs Fill.md` - Aspect ratio math
- `guide/Integrating the Magnetic Timeline with the WGPU backend.md` - Timeline composition

---

**Last Updated**: January 2026  
**Status**: Specification Complete  
**Next**: Implement in Phase 5a-5b
