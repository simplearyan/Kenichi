# Kenichi Project Roadmap: From Player to Editor

> **Philosophy**: "Engine First, UI Second" - A beautiful UI is useless if the engine can't play 4K video smoothly.

## 🎯 Current Status: Phase 4 (Frame Pacing) 🚧

We have successfully built a **Hardware-Accelerated Video Player** with:
- ✅ WGPU rendering pipeline
- ✅ FFmpeg decoding (hardware-accelerated path ready)
- ✅ Frame-accurate seeking with keyframe roll-forward
- ✅ Timeline sync (Backend ↔ Frontend)
- 🚧 Frame Pacing (wall-clock sync implemented)

---

## ⚠️ Technical Debt

**Current Debt Ratio**: 16-20% (Acceptable for MVP)

### 🔴 Critical (Must Fix Before Phase 5)
1. **Timeline Store** - Only 6 lines (stub). Complete code available in `guide/Magnetic Timeline Logic.md`
2. **Zero Tests** - No unit/integration tests. High regression risk.
3. **No CI Checks** - No automated linting/type-checking on PRs.

**Remediation Time**: 1 day  
**Impact**: Unblocks Phase 5, prevents 10+ days of rework

### 🟡 Moderate (Fix During Phase 5-6)
1. **Fixed Delta Time** - Render loop uses hardcoded 16ms (causes audio desync)
2. **println! Logging** - 50+ instances, should use `tracing` crate
3. **Unsafe Send** - Needs better documentation of safety invariants

**Remediation Time**: 2.5 days  
**Impact**: Reduces bugs, improves debugging

### 🟢 Low Priority (Phase 7+)
1. **Magic Numbers** - Hardcoded values need constants
2. **No API Docs** - Missing rustdoc/TSDoc

**Remediation Time**: 7 hours

**Full Analysis**: See `technical_debt_analysis.md` for detailed breakdown and cost-benefit analysis.

**Recommendation**: Address critical debt (1 day) before starting Phase 5.

---

## 📅 Development Phases

### ✅ Phase 1-3: Foundation (Completed)
**Goal**: Build a working video player

- [x] WGPU rendering pipeline setup
- [x] FFmpeg integration for video decoding
- [x] Basic playback controls (Play/Pause)
- [x] Timeline UI with playhead tracking
- [x] Frame-accurate seeking with keyframe roll-forward
- [x] Backend ↔ Frontend state synchronization

### 🚧 Phase 4: Frame Pacing & Optimization
**Goal**: Ensure video plays at native speed (e.g., 24fps, 30fps)

**Tasks**:
- [x] Extract FPS from video stream
- [x] Implement wall-clock synchronization
- [ ] Test with various framerates (23.976, 29.97, 60fps)
- [ ] Add frame drop detection for performance monitoring

**Implementation Strategy** (from research):
- Use `std::time::Instant` for wall-clock timing
- Implement frame skipping when decoder falls behind
- Reference: `guide/synchronization logic 60fps.md`

**Why Critical**: Without precise timing, audio will desync immediately.

---

### 🎵 Phase 4.5: Audio Playback
**Goal**: Add audio decoding and synchronization

**Backend**:
- Decode audio streams with FFmpeg (`AAC`, `MP3`, `PCM`)
- Use `cpal` or `rodio` for audio output
- Implement **Audio as Master Clock** strategy
- Handle audio buffer underruns gracefully

**Implementation Strategy** (from research):
```rust
// Audio Clock becomes the "Master"
// Every audio callback reports its timestamp
// WGPU renders the corresponding video frame
// Reference: guide/Missing Pieces.md - "Audio Mixing"
```

**Challenges**:
- Audio requires ±1ms precision (video can skip frames, audio cannot)
- Seeking must be instant to avoid audio pops
- Need to handle variable audio formats

**Verification**:
- Play video with audio in sync
- Seek without audio glitches
- Pause/resume without pops/clicks

---

### 🎬 Phase 5: Multi-Track Composition
**Goal**: Transition from "player" to "editor"

This is the most complex phase. Breaking it into sub-phases:

#### Phase 5a: Timeline Backend
**Goal**: Support multiple clips without UI

**Data Structure** (from research):
```rust
// Reference: guide/INSPIRATIONS.md - Timeline Data Architecture
pub struct Timeline {
    pub tracks: Vec<Track>,
}

pub struct Track {
    pub id: uuid::Uuid,
    pub name: String,
    pub clips: Vec<Clip>,
}

pub struct Clip {
    pub path: String,
    pub start_time: f64,  // Seconds in timeline
    pub duration: f64,
    pub offset: f64,      // Start point within source file
    pub z_index: i32,
}
```

**Tasks**:
- Implement `Timeline` struct with tracks and clips
- Handle "virtual time" (timeline time ≠ source file time)
- Support clip trimming, splitting, gaps
- Test with 2 clips on 1 track programmatically

**Reference**: `guide/The Uniform Data Structure.md`

#### Phase 5b: Multi-Layer WGPU Rendering
**Goal**: Composite multiple video sources

**Implementation Strategy** (from research):
- Update shader to handle multiple textures
- Implement alpha blending and layer ordering
- Add basic transitions (cut, fade)
- Use texture pooling for GPU memory optimization

**Shader Logic** (from research):
```wgsl
// Treat video frames as "Textures"
// Treat text/shapes as "Geometry"
// Reference: guide/INSPIRATIONS.md - Vector-Raster Hybrid Pipeline
```

**Tasks**:
- Create `TextureBindGroup` for each video layer
- Implement Z-index based rendering order
- Add basic cross-fade transition
- Optimize GPU memory usage (texture pooling)

#### Phase 5c: Timeline UI (Static)
**Goal**: Visual representation of clips

**UI Components** (from research):
- Track lanes with clip blocks
- Clip thumbnails (use existing `thumbs.rs`)
- Clip boundaries and duration labels
- Zoom/pan controls

**Implementation Strategy**:
```svelte
<!-- Reference: guide/Timeline UI.md -->
<TimelineTrack trackId={1}>
  {#each clips as clip}
    <ClipBlock {clip} />
  {/each}
</TimelineTrack>
```

**Tasks**:
- Render tracks and clips in Timeline panel
- Display clip thumbnails using `IntersectionObserver` (lazy loading)
- Show clip boundaries and duration
- Implement zoom/pan on timeline

**Reference**: `guide/Timeline UI.md`, `guide/CapCut Desktop.md`

#### Phase 5d: Timeline UI (Interactive)
**Goal**: Allow user to manipulate clips

**Magnetic Timeline** (from research):
```typescript
// Reference: guide/Magnetic Timeline Logic.md
export function removeClip(id: string) {
  // 1. Remove clip
  // 2. Shift all following clips on same track
  // 3. Animate with Svelte's flip()
}
```

**Tasks**:
- Drag-and-drop clips between tracks
- **Magnetic snapping** to other clips and playhead
- Resize clips (trim in/out points)
- Multi-select and group operations
- Ripple delete with smooth animation

**References**: 
- `guide/Magnetic Timeline Logic.md` (complete implementation)
- `guide/Smart Alignment.md`
- `guide/Snap-to-Grid logic.md`

---

### ✂️ Phase 6: Editing Tools
**Goal**: Provide professional editing capabilities

**UI Tools** (from research):
- **Razor Tool**: Split clips at playhead
- **Selection Tool**: Move and trim clips
- **Ripple Delete**: Remove clip and close gap (magnetic mode)
- **Slip/Slide**: Adjust clip content without moving position

**Backend Commands**:
```rust
// Reference: guide/INSPIRATIONS.md
fn split_clip(clip_id: Uuid, time: f64) -> Result<(Clip, Clip)>
fn move_clip(clip_id: Uuid, new_position: f64) -> Result<()>
fn trim_clip(clip_id: Uuid, in_point: f64, out_point: f64) -> Result<()>
fn delete_clip(clip_id: Uuid, ripple: bool) -> Result<()>
```

**UX Details** (from research):
- Context menus on right-click (not browser default)
- Tooltips showing keyboard shortcuts (e.g., "Split (B)")
- Active state highlighting when dragging
- Floating toolbar at top of timeline

**References**: 
- `guide/Keybindings & Shortcuts.md`
- `guide/The Global Shortcut Action.md`

---

### ↩️ Phase 6.5: Undo/Redo System
**Goal**: Allow users to revert mistakes

**Implementation** (from research):
```typescript
// Reference: guide/Missing Pieces.md - Command Pattern
export const undoStack = writable<Command[]>([]);
export const redoStack = writable<Command[]>([]);

export function executeCommand(command: Command) {
    command.do();
    undoStack.update(s => [...s, command]);
    redoStack.set([]); // Clear redo on new action
}
```

**Command Types**:
- `AddClip`, `RemoveClip`, `MoveClip`
- `ChangeProperty` (scale, position, opacity)
- `SplitClip`, `TrimClip`

**Tasks**:
- Implement Command Pattern for all edit operations
- Maintain undo/redo stacks (limit: 100 actions)
- Serialize commands for project save/load
- Add keyboard shortcuts (Ctrl+Z, Ctrl+Y)

**Why Critical**: Must be implemented before Phase 7 to avoid refactoring pain.

**Reference**: `guide/Missing Pieces.md` (complete implementation)

---

### 🎨 Phase 7: Effects & Transitions
**Goal**: Add visual polish and creativity

**WGPU Shaders** (from research):
- **Color Correction**: Brightness, contrast, saturation (easy - fragment shader math)
- **Blur/Sharpen**: Gaussian blur, unsharp mask (medium - requires multi-pass)
- **Transitions**: Dissolve, wipe, zoom (hard - temporal blending)
- **Keyframe Animation**: Animate effect parameters over time

**Interpolation Engine** (from research):
```rust
// Reference: guide/INSPIRATIONS.md - Tweening Engine
pub fn interpolate(start_val: f32, end_val: f32, progress: f32) -> f32 {
    start_val + (end_val - start_val) * progress
}
```

**UI Components**:
- Effect browser panel
- Drag-and-drop effects onto clips
- Keyframe editor for animations
- Real-time preview (with performance mode toggle)

**Performance Optimization** (from research):
- Disable real-time effects on low-end systems
- Use proxy clips for effect preview
- Reference: `guide/Low Performance Mode.md`

---

### 💾 Phase 8: Export Pipeline
**Goal**: Render timeline to video file

**Export Strategy** (from research):
```rust
// Re-encode WGPU output using FFmpeg
// Reference: guide/The High Level Pipeline.md
// Use hardware encoding: h264_nvenc, vaapi, QuickSync
```

**Tasks**:
- Re-encode WGPU output using FFmpeg's `libx264`/`libx265`
- Support multiple export presets (1080p, 4K, web-optimized)
- Show export progress with time estimates
- Handle audio re-encoding and muxing

**Progress Parsing** (from research):
```rust
// Reference: guide/The Progress Parser Logic.md
// Parse FFmpeg stderr for progress updates
// Regex: frame=\s*(\d+)\s*fps=\s*([\d.]+)
```

**UI Components** (from research):
- Export modal with preset selection
- Progress bar with time remaining
- Cancel button (graceful FFmpeg termination)
- Reference: `guide/The Export Modal UI (Svelte & Bits UI).md`

**Challenges**:
- Maintaining quality while keeping file size reasonable
- Handling long exports without UI freezing
- Supporting hardware encoding (NVENC, QuickSync)

---

### 💾 Phase 8.5: Project Persistence
**Goal**: Save and load editing projects

**File Format** (from research):
```json
// Reference: guide/Missing Pieces.md - Project File Format
{
  "version": "1.0.0",
  "metadata": { "name": "My Epic Edit", "created": "2026-01-24" },
  "settings": { "width": 1920, "height": 1080, "fps": 30 },
  "assets": [
    { "id": "uuid-1", "path": "C:/Videos/shot1.mp4", "proxyPath": "cache/uuid-1.mp4" }
  ],
  "timeline": {
    "tracks": [
      {
        "id": "v1",
        "type": "video",
        "clips": [
          { "assetId": "uuid-1", "start": 0, "duration": 150, "offset": 10 }
        ]
      }
    ]
  }
}
```

**Tasks**:
- Serialize Timeline to `.kenichi` JSON format
- Handle relative vs absolute file paths
- Auto-save functionality (every 2 minutes)
- Project templates (16:9, 9:16, 1:1)

**Reference**: `guide/Missing Pieces.md`

---

### ⚡ Phase 9: Optimization & Polish
**Goal**: Production-ready performance

**Performance** (from research):
- Multi-threaded decoding (decode ahead of playhead)
- GPU memory management (texture pooling, LRU cache)
- **Proxy generation for 4K+ files** (already implemented)
- Scrubbing optimization (decode only keyframes)

**Low-End PC Optimizations** (from research):
```rust
// Reference: guide/Low Performance Mode.md
// Auto-detect system capabilities
fn is_low_end_system() -> bool {
    // Check GPU, RAM, CPU
    // Reference: guide/is_low_end_system.md
}

// Adaptive quality:
// - Reduce timeline thumbnail resolution
// - Disable real-time effects preview
// - Use 30fps instead of 60fps
```

**UX Polish** (from research):
- **Keyboard shortcuts** (comprehensive keybinding system)
- **Customizable workspace layouts**
- **Dark/light themes** (UnoCSS theme switching)
- **Scrubbable inputs** (drag on numbers to change values)
- **Custom window frame** (chromeless, CapCut-style)
- **Safe zone overlays** (TikTok, Instagram aspect ratios)

**CapCut-Inspired Features** (from research):
- Hover-to-preview thumbnails in Media panel
- Draggable panel gutters for resizing
- Floating toolbars that stay visible when scrolling
- Active state highlighting during drag operations

**References**:
- `guide/CapCut Desktop.md` (complete UX teardown)
- `guide/The Scrubbable Input Component.md`
- `guide/Keybindings & Shortcuts.md`
- `guide/Safe Area Overlay.md`
- `guide/HUD (Heads-Up Display).md`

---

## 🏗️ Architectural Strategy: "Vertical Features"

We build **complete features end-to-end**, not layers:

❌ **Wrong**: Build all backend → Build all WGPU → Build all UI  
✅ **Right**: Build Playback (Backend + WGPU + UI) → Build Timeline (Backend + WGPU + UI)

**Benefits**:
- Always have a working demo
- Test real-world performance at each stage
- Avoid "integration hell" at the end
- Can pivot based on user feedback

---

## 🎯 Success Metrics

### Phase 4-5 (Player → Editor)
- [ ] Play 4K video at 60fps without frame drops
- [ ] Seek to any timestamp in <100ms
- [ ] Audio/video sync within ±10ms
- [ ] Support 10+ clips on timeline without lag

### Phase 6-9 (Editor → Production Tool)
- [ ] Export 1080p video faster than real-time
- [ ] Undo/redo 100 operations without memory issues
- [ ] Apply 5+ effects in real-time
- [ ] Handle 1-hour timeline without crashes

---

## 🤝 Contributing

This roadmap is a living document. If you're interested in contributing:

1. Check the current phase in [Issues](../../issues)
2. Look for `good-first-issue` or `help-wanted` tags
3. Read [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines
4. Join discussions in [Discussions](../../discussions)

---

## 📚 Technical Stack

- **Backend**: Rust, FFmpeg, Tauri
- **Renderer**: WGPU (WebGPU)
- **Frontend**: Svelte 5, TypeScript
- **Audio**: cpal/rodio (TBD)
- **Build**: pnpm, Cargo

---

**Last Updated**: January 2026  
**Current Phase**: 4 (Frame Pacing)  
**Next Milestone**: Phase 4.5 (Audio Playback)
