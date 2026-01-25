# Kenichi Project Roadmap: From Player to Editor

> **Philosophy**: "Engine First, UI Second" - A beautiful UI is useless if the engine can't play 4K video smoothly.

## 🎯 Current Status: Phase 4 (Frame Pacing)

We have successfully built a **Hardware-Accelerated Video Player** with:
- ✅ **Backend (Rust)**: FFmpeg video decoding, memory management, playback loops
- ✅ **Renderer (WGPU)**: GPU-accelerated frame display
- ✅ **UI (Svelte)**: Play/Pause/Seek controls with timeline synchronization
- ✅ **Seeking**: Frame-accurate keyframe + roll-forward seeking
- 🚧 **Frame Pacing**: Wall-clock synchronized playback (in progress)

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

**Why Critical**: Without precise timing, audio will desync immediately.

---

### 🎵 Phase 4.5: Audio Playback
**Goal**: Add audio decoding and synchronization

**Backend**:
- Decode audio streams with FFmpeg
- Use `cpal` or `rodio` for audio output
- Implement Audio/Video PTS synchronization
- Handle audio buffer underruns gracefully

**Challenges**:
- Audio requires ±1ms precision (video can skip frames, audio cannot)
- Need to handle variable audio formats (AAC, MP3, PCM)

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

**Tasks**:
- Create `Timeline` struct (clips, tracks, layers)
- Implement "virtual time" (timeline time ≠ source file time)
- Handle clip trimming, splitting, gaps
- Test with 2 clips on 1 track programmatically

#### Phase 5b: Multi-Layer WGPU Rendering
**Goal**: Composite multiple video sources

**Tasks**:
- Update shader to handle multiple textures
- Implement alpha blending and layer ordering
- Add basic transitions (cut, fade)
- Optimize GPU memory usage (texture pooling)

#### Phase 5c: Timeline UI (Static)
**Goal**: Visual representation of clips

**Tasks**:
- Render tracks and clips in Timeline panel
- Display clip thumbnails
- Show clip boundaries and duration
- Implement zoom/pan on timeline

#### Phase 5d: Timeline UI (Interactive)
**Goal**: Allow user to manipulate clips

**Tasks**:
- Drag-and-drop clips between tracks
- Snapping to other clips and playhead
- Resize clips (trim in/out points)
- Multi-select and group operations

---

### ✂️ Phase 6: Editing Tools
**Goal**: Provide professional editing capabilities

**UI Tools**:
- **Razor Tool**: Split clips at playhead
- **Selection Tool**: Move and trim clips
- **Ripple Delete**: Remove clip and close gap
- **Slip/Slide**: Adjust clip content without moving position

**Backend Commands**:
- `split_clip(clip_id, time)`
- `move_clip(clip_id, new_position)`
- `trim_clip(clip_id, in_point, out_point)`
- `delete_clip(clip_id, ripple: bool)`

---

### ↩️ Phase 6.5: Undo/Redo System
**Goal**: Allow users to revert mistakes

**Implementation**:
- Command Pattern for all edit operations
- Maintain undo/redo stacks
- Serialize commands for project save/load
- Set reasonable history limit (e.g., 100 actions)

**Critical**: Must be implemented before Phase 7 to avoid refactoring pain.

---

### 🎨 Phase 7: Effects & Transitions
**Goal**: Add visual polish and creativity

**WGPU Shaders**:
- **Color Correction**: Brightness, contrast, saturation (easy)
- **Blur/Sharpen**: Gaussian blur, unsharp mask (medium - requires multi-pass)
- **Transitions**: Dissolve, wipe, zoom (hard - temporal blending)
- **Keyframe Animation**: Animate effect parameters over time

**UI**:
- Effect browser panel
- Drag-and-drop effects onto clips
- Keyframe editor for animations
- Real-time preview

---

### 💾 Phase 8: Export Pipeline
**Goal**: Render timeline to video file

**Tasks**:
- Re-encode WGPU output using FFmpeg's `libx264`/`libx265`
- Support multiple export presets (1080p, 4K, web-optimized)
- Show export progress with time estimates
- Handle audio re-encoding and muxing

**Challenges**:
- Maintaining quality while keeping file size reasonable
- Handling long exports without UI freezing
- Supporting hardware encoding (NVENC, QuickSync)

---

### 💾 Phase 8.5: Project Persistence
**Goal**: Save and load editing projects

**Tasks**:
- Serialize Timeline to JSON/Binary format
- Handle relative vs absolute file paths
- Auto-save functionality
- Project templates

**File Format**:
```json
{
  "version": "1.0",
  "timeline": {
    "tracks": [...],
    "clips": [...],
    "effects": [...]
  },
  "settings": {...}
}
```

---

### ⚡ Phase 9: Optimization & Polish
**Goal**: Production-ready performance

**Performance**:
- Multi-threaded decoding (decode ahead of playhead)
- GPU memory management (texture pooling, LRU cache)
- Proxy generation for 4K+ files
- Scrubbing optimization (decode only keyframes)

**UX Polish**:
- Keyboard shortcuts
- Customizable workspace layouts
- Dark/light themes
- Accessibility features

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
