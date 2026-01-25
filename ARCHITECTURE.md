# Architecture

## Overview

Kenichi is a **desktop video editor** built with a hybrid architecture:
- **Frontend**: Svelte 5 (reactive UI)
- **Backend**: Rust (performance-critical operations)
- **Renderer**: WGPU (GPU-accelerated video display)
- **Decoder**: FFmpeg (video/audio decoding)

**Philosophy**: "Engine First, UI Second" - Performance is prioritized over aesthetics.

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         User Interface                       │
│                      (Svelte 5 + UnoCSS)                     │
└───────────────────────┬─────────────────────────────────────┘
                        │ Tauri IPC (invoke)
┌───────────────────────▼─────────────────────────────────────┐
│                      Tauri Backend                           │
│                    (Rust + Tokio Async)                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Commands   │  │    Engine    │  │ ProxyManager │      │
│  └──────────────┘  └──────┬───────┘  └──────────────┘      │
└────────────────────────────┼──────────────────────────────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
┌───────▼────────┐  ┌────────▼────────┐  ┌───────▼────────┐
│  VideoDecoder  │  │  WGPU Renderer  │  │  Export Utils  │
│   (FFmpeg)     │  │   (GPU Accel)   │  │   (FFmpeg)     │
└────────────────┘  └─────────────────┘  └────────────────┘
```

---

## Core Components

### 1. Frontend (Svelte 5)

**Location**: `src/`

**Responsibilities**:
- User interface rendering
- User input handling
- State management (Svelte stores)
- Timeline visualization

**Key Patterns**:
- **Runes**: Svelte 5's reactivity system (`$state`, `$derived`)
- **Stores**: Global state (`playback.ts`, `timeline.ts`)
- **Actions**: Reusable behaviors (`smartDrag.ts`, `shortcuts.ts`)

**Why Svelte 5?**
- Compiled (no virtual DOM overhead)
- Smallest bundle size (~3KB runtime)
- Native reactivity (no useState/useEffect)

---

### 2. Backend (Rust + Tauri)

**Location**: `src-tauri/src/`

**Responsibilities**:
- Video decoding (FFmpeg wrapper)
- GPU rendering (WGPU)
- File I/O and proxy generation
- Playback state management

**Key Modules**:

#### `engine/model.rs` - KinetixEngine
Core engine managing:
- WGPU surface and device
- Video decoder instance
- Playback state (playing, paused, current time)
- Render loop (60 FPS)

#### `engine/decoding.rs` - VideoDecoder
FFmpeg wrapper providing:
- Frame decoding (`decode_next_frame`)
- Seeking (`seek` with keyframe + roll-forward)
- FPS extraction

#### `engine/renderer.rs` - WGPU Rendering
GPU rendering pipeline:
- Texture upload
- Shader execution
- Frame presentation

#### `commands/` - Tauri Commands
Thin wrappers exposing engine to frontend:
- `attach_wgpu_renderer`
- `play`, `pause`, `seek`
- `get_playback_state`

**Why Rust?**
- Memory safety without garbage collection
- Zero-cost abstractions
- Excellent FFmpeg bindings

---

### 3. WGPU Renderer

**Why WGPU over Canvas/WebGL?**

| Feature | WGPU | Canvas | WebGL |
|---|---|---|---|
| **Performance** | Native GPU | CPU-bound | GPU |
| **Memory** | Direct texture upload | Copy to JS | Copy to JS |
| **Cross-platform** | Vulkan/Metal/DX12 | Browser-only | Browser-only |
| **Low-end GPUs** | Optimized | Slow | Moderate |

**Rendering Pipeline**:
```
Video Frame (FFmpeg)
    ↓
RGBA Conversion (Rust)
    ↓
Texture Upload (WGPU)
    ↓
Shader (video.wgsl)
    ↓
Screen (60 FPS)
```

---

### 4. FFmpeg Integration

**Proxy Methodology**:
```
User imports 4K video
    ↓
ProxyManager generates 360p proxy (background)
    ↓
User edits with proxy (smooth)
    ↓
Export uses original 4K (high quality)
```

**Why Proxies?**
- 4K decoding is CPU-intensive (30-50% CPU on GT 740)
- 360p decoding is fast (<10% CPU)
- Enables smooth editing on low-end hardware

---

## Data Flow

### Playback Flow
```
1. User clicks "Play" (Svelte)
2. invoke('play') → Tauri
3. engine.play() sets is_playing = true
4. 60 FPS loop calls tick()
5. tick() decodes next frame
6. Frame uploaded to WGPU texture
7. WGPU renders to screen
8. Frontend polls get_playback_state()
9. Timeline updates playhead position
```

### Seek Flow
```
1. User clicks timeline (Svelte)
2. invoke('seek', { time: 5.0 })
3. decoder.seek(5.0)
   a. Seek to nearest keyframe (backward)
   b. Decode frames until PTS >= 5.0
4. Update texture with target frame
5. Update playback_state.current_time
6. Frontend updates playhead
```

---

## Threading Model

### Main Thread (Tauri)
- Handles UI events
- Processes Tauri commands
- Manages WGPU surface

### Render Loop Thread (Tokio)
```rust
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_millis(16));
    loop {
        interval.tick().await;
        engine.tick(0.016); // 60 FPS
    }
});
```

**Why Separate Thread?**
- Decouples rendering from UI events
- Prevents frame drops during heavy UI operations
- Enables smooth 60 FPS playback

---

## Memory Management

### Video Frames
- **Decoded**: ~8MB per 1080p RGBA frame
- **Texture**: GPU memory (shared with system on integrated GPUs)
- **Strategy**: Single frame buffer (no frame queue)

### Proxy Files
- **Storage**: `~/.kenichi/cache/`
- **Cleanup**: Manual (future: LRU cache)

---

## State Management

### Frontend State (Svelte Stores)
```typescript
// playback.ts
export const isPlaying = writable(false);
export const currentTime = writable(0.0);

// timeline.ts
export const clips = writable([]);
export const tracks = writable([]);
```

### Backend State (Rust)
```rust
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_time: f64,
    pub duration: f64,
    pub last_frame_time: Option<Instant>,
}
```

**Sync Strategy**:
- Backend is source of truth
- Frontend polls `get_playback_state()` at 10 FPS
- User actions immediately update backend

---

## Design Decisions

### Why Tauri over Electron?
| Metric | Tauri | Electron |
|---|---|---|
| **Binary Size** | 3-5 MB | 120+ MB |
| **Memory** | 50-100 MB | 200-500 MB |
| **Startup** | <1s | 2-5s |
| **Security** | Rust sandboxing | Node.js risks |

### Why SvelteKit SPA Mode?
- Desktop apps don't need SSR
- Smaller bundle size
- Faster startup

### Why UnoCSS over Tailwind?
- Faster build times (instant vs 500ms)
- Smaller bundle (only used classes)
- Attributify mode (cleaner HTML)

---

## Performance Optimizations

### 1. Proxy Files
Reduces 4K decode overhead by 80%

### 2. GPU Rendering
Offloads video display to GPU (10x faster than Canvas)

### 3. Frame Pacing
Wall-clock sync prevents "fast-forward" playback

### 4. Lazy Loading
Timeline thumbnails load on-demand (IntersectionObserver)

---

## Security Considerations

### Tauri Security
- Commands require explicit `#[tauri::command]` macro
- No `eval()` or `dangerouslySetInnerHTML`
- CSP enabled by default

### FFmpeg Binaries
- Downloaded from trusted sources
- Bundled in release builds
- Checksums verified (future)

---

## Future Architecture Changes

### Phase 5: Multi-Track
- Add `Timeline` struct (tracks, clips)
- Multi-texture rendering (layer compositing)

### Phase 6.5: Undo/Redo
- Command Pattern for all operations
- Undo/redo stacks in Rust

### Phase 8: Export
- Background export thread
- Progress streaming via Tauri events

---

## References

- [WGPU Documentation](https://wgpu.rs/)
- [FFmpeg Documentation](https://ffmpeg.org/documentation.html)
- [Tauri Documentation](https://tauri.app/)
- [Svelte 5 Documentation](https://svelte-5-preview.vercel.app/)

---

**Last Updated**: January 2026  
**Architecture Version**: 1.0 (Phases 1-4)
