# Research Implementation Checklist

## Overview

This checklist maps all 51 research files from `/guide` to specific implementation phases and tasks.

**Legend**:
- 🔴 Critical (Blocks next phase)
- 🟡 High (Important for phase completion)
- 🟢 Medium (Enhances functionality)
- ⚪ Low (Polish/optional)
- ✅ Complete
- 🚧 In Progress
- ❌ Not Started

---

## Phase 4: Frame Pacing & Optimization

### 🚧 Current Tasks

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `synchronization logic 60fps.md` | 🚧 | 🟡 | 30min | Fix delta time calculation in render loop |
| `Why the Old Code Leaked.md` | ✅ | - | - | Reference (issue resolved) |

**Deliverables**:
- [ ] Replace `tick(0.016)` with actual delta time
- [ ] Test with 23.976, 29.97, 60fps videos
- [ ] Add frame drop detection

---

## Phase 4.5: Audio Playback

### ❌ Not Started

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `Missing Pieces.md` (Audio section) | ❌ | 🔴 | 2 days | Implement audio decoding with FFmpeg |
| `synchronization logic 60fps.md` | ❌ | 🔴 | 1 day | Audio as Master Clock strategy |

**Deliverables**:
- [ ] Audio decoding (`cpal` or `rodio`)
- [ ] Audio/Video PTS synchronization
- [ ] Handle buffer underruns
- [ ] Seek without audio pops

**Dependencies**: Phase 4 frame pacing must be complete

---

## Phase 5a: Timeline Backend

### 🔴 Critical Path

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `INSPIRATIONS.md` (Timeline section) | ❌ | 🔴 | 4h | Implement `Timeline` struct |
| `The Uniform Data Structure.md` | ❌ | 🔴 | 6h | Design unified clip/effect data model |
| `Missing Pieces.md` (Project Format) | ❌ | 🔴 | 4h | Design `.kenichi` JSON schema |
| `Recommended Folder Structure.md` | ✅ | - | - | Reference (already followed) |

**Implementation Steps**:

1. **Create Timeline Data Structure** (4 hours)
   ```rust
   // Reference: INSPIRATIONS.md
   pub struct Timeline {
       pub tracks: Vec<Track>,
   }
   
   pub struct Track {
       pub id: Uuid,
       pub clips: Vec<Clip>,
   }
   
   pub struct Clip {
       pub path: String,
       pub start_time: f64,
       pub duration: f64,
       pub offset: f64,
       pub z_index: i32,
   }
   ```

2. **Implement Virtual Time** (2 hours)
   - Timeline time ≠ source file time
   - Handle clip trimming, splitting, gaps

3. **Test with 2 clips programmatically** (2 hours)

**Deliverables**:
- [ ] `src-tauri/src/engine/timeline.rs`
- [ ] Unit tests for timeline operations
- [ ] `.kenichi` project file format

---

## Phase 5b: Multi-Layer WGPU Rendering

### 🔴 Critical Path

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `INSPIRATIONS.md` (Vector-Raster Hybrid) | ❌ | 🔴 | 1 day | Multi-texture shader |
| `Integrating the Magnetic Timeline with the WGPU backend.md` | ❌ | 🔴 | 1 day | `update_composition` command |
| `The Dual-Source Engine Logic.md` | ❌ | 🟡 | 6h | Source/Timeline preview switching |
| `The Core Preview Logic.md` | ❌ | 🟢 | 4h | Optimize YUV to RGBA on GPU |
| `The Two Logic States Fit vs Fill.md` | ❌ | 🟢 | 2h | Video scaling modes |

**Implementation Steps**:

1. **Multi-Texture Shader** (1 day)
   ```wgsl
   // Reference: INSPIRATIONS.md
   // Treat video frames as "Textures"
   // Treat text/shapes as "Geometry"
   ```
   - Update shader to handle multiple textures
   - Implement alpha blending
   - Add Z-index based layer ordering

2. **Backend Sync** (1 day)
   ```rust
   // Reference: Integrating the Magnetic Timeline with the WGPU backend.md
   #[tauri::command]
   pub fn update_composition(new_clips: Vec<ClipData>) {
       engine.composition = new_clips;
       engine.request_frame_update();
   }
   ```

3. **Dual-Source Engine** (6 hours)
   ```rust
   // Reference: The Dual-Source Engine Logic.md
   enum PreviewMode {
       Timeline,
       Source(AssetId),
   }
   ```

**Deliverables**:
- [ ] Multi-layer rendering shader
- [ ] `update_composition` Tauri command
- [ ] Texture pooling for GPU memory
- [ ] Basic cross-fade transition

---

## Phase 5c: Timeline UI (Static)

### 🟡 High Priority

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `Timeline UI.md` | ❌ | 🔴 | 1 day | Build visual timeline |
| `CapCut Desktop.md` (Timeline section) | ❌ | 🟡 | 4h | CapCut-style UI |
| `Font Size Hierarchy.md` | ✅ | - | - | Already in UnoCSS |
| `UI Color Palette (Hex Codes).md` | ✅ | - | - | Already in UnoCSS |
| `The UnoCSS CapCut Theme Configuration.md` | ✅ | - | - | Already configured |

**Implementation Steps**:

1. **Track Lanes Component** (4 hours)
   ```svelte
   <!-- Reference: Timeline UI.md -->
   <TimelineTrack trackId={1}>
     {#each clips as clip}
       <ClipBlock {clip} />
     {/each}
   </TimelineTrack>
   ```

2. **Clip Thumbnails** (4 hours)
   - Use existing `thumbs.rs`
   - Lazy loading with `IntersectionObserver`

3. **Zoom/Pan Controls** (2 hours)

**Deliverables**:
- [ ] Track lane visualization
- [ ] Clip blocks with thumbnails
- [ ] Timecode ruler
- [ ] Zoom/pan controls

---

## Phase 5d: Timeline UI (Interactive)

### 🔴 Critical Path

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `Magnetic Timeline Logic.md` | ❌ | 🔴 | 6h | Implement magnetic snapping |
| `Snap-to-Grid logic.md` | ❌ | 🔴 | 4h | Frame-accurate positioning |
| `Smart Alignment.md` | ❌ | 🟡 | 4h | Safe zone snapping |
| `Svelte Resize Observer.md` | ✅ | - | - | Already implemented |

**Implementation Steps**:

1. **Magnetic Timeline** (6 hours)
   ```typescript
   // Reference: Magnetic Timeline Logic.md (COMPLETE CODE AVAILABLE)
   export function removeClip(id: string) {
     // 1. Remove clip
     // 2. Shift all following clips on same track
     // 3. Animate with Svelte's flip()
   }
   ```

2. **Snap-to-Grid** (4 hours)
   ```typescript
   // Reference: Snap-to-Grid logic.md (COMPLETE CODE AVAILABLE)
   export const snapToGrid = derived([frameWidth, snapEnabled], 
     ([$frameWidth, $enabled]) => {
       return (rawX: number): number => {
         if (!$enabled) return rawX;
         return Math.round(rawX / $frameWidth) * $frameWidth;
       };
     }
   );
   ```

3. **Smart Alignment** (4 hours)
   ```typescript
   // Reference: Smart Alignment.md
   export function calculateSnap(
     currentPos: number,
     targets: number[],
     threshold: number = 10
   )
   ```

**Deliverables**:
- [ ] Drag-and-drop clips
- [ ] Magnetic snapping (ripple delete)
- [ ] Frame-accurate positioning
- [ ] Safe zone alignment guides
- [ ] Multi-select operations

---

## Phase 6: Editing Tools

### 🟡 High Priority

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `Keybindings & Shortcuts.md` | ❌ | 🔴 | 1 day | Full keybinding system |
| `Keybindings Modal.md` | ❌ | 🟡 | 4h | Searchable shortcuts modal |
| `The Global Shortcut Action.md` | ❌ | 🟡 | 4h | Global shortcut registration |
| `HUD (Heads-Up Display).md` | ❌ | 🟢 | 2h | Visual shortcut feedback |
| `INSPIRATIONS.md` (Commands) | ❌ | 🔴 | 1 day | Backend edit commands |

**Implementation Steps**:

1. **Backend Commands** (1 day)
   ```rust
   // Reference: INSPIRATIONS.md
   fn split_clip(clip_id: Uuid, time: f64) -> Result<(Clip, Clip)>
   fn move_clip(clip_id: Uuid, new_position: f64) -> Result<()>
   fn trim_clip(clip_id: Uuid, in_point: f64, out_point: f64) -> Result<()>
   fn delete_clip(clip_id: Uuid, ripple: bool) -> Result<()>
   ```

2. **Keybinding System** (1 day)
   - Customizable keybindings
   - Conflict detection
   - Import/export presets

3. **HUD Integration** (2 hours)
   ```typescript
   // Reference: HUD (Heads-Up Display).md
   showHud('Clip Split', 'i-lucide-scissors');
   ```

**Deliverables**:
- [ ] Razor tool (split at playhead)
- [ ] Selection tool (move/trim)
- [ ] Ripple delete
- [ ] Slip/slide tools
- [ ] Keybinding modal
- [ ] HUD feedback

---

## Phase 6.5: Undo/Redo System

### 🔴 Critical

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `Missing Pieces.md` (Command Pattern) | ❌ | 🔴 | 1 day | Implement Command Pattern |

**Implementation Steps**:

1. **Command Pattern** (1 day)
   ```typescript
   // Reference: Missing Pieces.md (COMPLETE CODE AVAILABLE)
   export const undoStack = writable<Command[]>([]);
   export const redoStack = writable<Command[]>([]);
   
   export function executeCommand(command: Command) {
       command.do();
       undoStack.update(s => [...s, command]);
       redoStack.set([]);
   }
   ```

2. **Command Types**
   - `AddClip`, `RemoveClip`, `MoveClip`
   - `ChangeProperty`, `SplitClip`, `TrimClip`

**Deliverables**:
- [ ] Command Pattern implementation
- [ ] Undo/redo stacks (100 action limit)
- [ ] Keyboard shortcuts (Ctrl+Z, Ctrl+Y)
- [ ] Command serialization for save/load

**Why Critical**: Must be done before Phase 7 to avoid refactoring

---

## Phase 7: Effects & Transitions

### 🟡 High Priority

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `INSPIRATIONS.md` (Tweening Engine) | ❌ | 🔴 | 1 day | Keyframe interpolation |
| `Low Performance Mode.md` | ❌ | 🟡 | 4h | Disable effects on low-end |

**Implementation Steps**:

1. **Interpolation Engine** (1 day)
   ```rust
   // Reference: INSPIRATIONS.md (CODE AVAILABLE)
   pub fn interpolate(start_val: f32, end_val: f32, progress: f32) -> f32 {
       start_val + (end_val - start_val) * progress
   }
   ```

2. **WGPU Shaders**
   - Color correction (easy)
   - Blur/sharpen (medium - multi-pass)
   - Transitions (hard - temporal blending)

3. **Performance Mode** (4 hours)
   ```rust
   // Reference: Low Performance Mode.md
   if is_low_end_system() {
       disable_realtime_effects();
   }
   ```

**Deliverables**:
- [ ] Keyframe animation system
- [ ] Color correction shaders
- [ ] Blur/sharpen effects
- [ ] Dissolve/wipe transitions
- [ ] Effect browser panel
- [ ] Performance mode toggle

---

## Phase 8: Export Pipeline

### 🟡 High Priority

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `The Progress Parser Logic.md` | ❌ | 🔴 | 4h | FFmpeg progress parsing |
| `The Export Modal UI (Svelte & Bits UI).md` | ❌ | 🟡 | 4h | Export modal UI |
| `The High Level Pipeline.md` | ❌ | 🟢 | - | Reference |

**Implementation Steps**:

1. **Progress Parser** (4 hours)
   ```rust
   // Reference: The Progress Parser Logic.md (REGEX AVAILABLE)
   // Parse FFmpeg stderr for progress updates
   // Regex: frame=\s*(\d+)\s*fps=\s*([\d.]+)
   ```

2. **Export Modal** (4 hours)
   ```svelte
   <!-- Reference: The Export Modal UI.md -->
   <ExportModal>
     <PresetSelector />
     <ProgressBar />
     <CancelButton />
   </ExportModal>
   ```

3. **Export Engine** (2 days)
   - Re-encode WGPU output
   - Hardware encoding (NVENC, QuickSync)
   - Audio re-encoding and muxing

**Deliverables**:
- [ ] Export presets (1080p, 4K, web)
- [ ] Progress bar with time estimates
- [ ] Cancel button (graceful termination)
- [ ] Hardware encoding support

---

## Phase 8.5: Project Persistence

### 🔴 Critical

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `Missing Pieces.md` (Project Format) | ❌ | 🔴 | 1 day | Implement save/load |

**Implementation Steps**:

1. **File Format** (4 hours)
   ```json
   // Reference: Missing Pieces.md (SCHEMA AVAILABLE)
   {
     "version": "1.0.0",
     "metadata": { "name": "My Edit", "created": "2026-01-24" },
     "timeline": { "tracks": [...], "clips": [...] }
   }
   ```

2. **Serialization** (4 hours)
   - Serialize Timeline to JSON
   - Handle relative vs absolute paths
   - Auto-save (every 2 minutes)

**Deliverables**:
- [ ] `.kenichi` file format
- [ ] Save/load functionality
- [ ] Auto-save
- [ ] Project templates (16:9, 9:16, 1:1)

---

## Phase 9: Optimization & Polish

### 🟢 Medium Priority

| Research File | Status | Priority | Effort | Task |
|---|---|---|---|---|
| `Low Performance Mode.md` | ❌ | 🟡 | 1 day | Auto-detect low-end systems |
| `Enhanced Engine Sync with Auto-Detection.md` | ❌ | 🟡 | 6h | Auto-throttle to 30fps |
| `is_low_end_system.md` | ❌ | 🟡 | 4h | Hardware detection |
| `The Rust Stress Test Command.md` | ❌ | 🟢 | 2h | Performance benchmarking |
| `The Scrubbable Input Component.md` | ❌ | 🟢 | 4h | Drag-to-change inputs |
| `Safe Area Overlay.md` | ✅ | - | - | Component exists |
| `CapCut Desktop.md` | ❌ | 🟢 | 2 days | UX polish features |
| `The Design Layout - Multi-Device Cohesion.md` | ❌ | ⚪ | - | Future mobile support |
| `The Right-Stack Layout Analysis.md` | ❌ | ⚪ | - | Reference |
| `The Vertical Center Layout.md` | ❌ | ⚪ | - | Reference |
| `The Workspace Toggle (Hybrid Approach).md` | ❌ | 🟢 | 4h | Mode switching |
| `The CapCut Text Tab How it Works.md` | ❌ | ⚪ | - | Future text editing |
| `The Binary Naming Convention.md` | ✅ | - | - | Already followed |

**Implementation Steps**:

1. **Auto-Detection** (1 day)
   ```rust
   // Reference: is_low_end_system.md
   fn is_low_end_system() -> bool {
       // Check GPU, RAM, CPU
   }
   ```

2. **Auto-Throttling** (6 hours)
   ```typescript
   // Reference: Enhanced Engine Sync with Auto-Detection.md (CODE AVAILABLE)
   if (avgDuration > 20ms over 10 frames) {
       performanceMode.set('low'); // 30fps
   }
   ```

3. **Scrubbable Inputs** (4 hours)
   ```svelte
   <!-- Reference: The Scrubbable Input Component.md (CODE AVAILABLE) -->
   <ScrubbableInput 
     label="Scale" 
     bind:value={scale} 
     step={0.5} 
     suffix="%" 
   />
   ```

4. **CapCut Features** (2 days)
   - Custom window frame
   - Draggable panel gutters
   - Hover-to-preview thumbnails
   - Floating toolbars

**Deliverables**:
- [ ] Auto-detect low-end systems
- [ ] Auto-throttle to 30fps
- [ ] Scrubbable inputs
- [ ] Custom window frame
- [ ] Draggable gutters
- [ ] Keyboard shortcuts
- [ ] Themes (dark/light)

---

## Reference Files (No Implementation)

| Research File | Purpose | Notes |
|---|---|---|
| `guide.md` | Overview | Reference |
| `updated-guide.md` | Updated overview | Reference |
| `learning-stack-problems.md` | Historical | Tech stack decisions |
| `long-term.md` | Future planning | Long-term ideas |
| `svelte.typescript-WHY.md` | Historical | TypeScript justification |
| `capcut copy.md` | Unknown | Review and consolidate |
| `capcut copy 7.md` | Unknown | Review and consolidate |
| `capcut copy 8.md` | Unknown | Review and consolidate |

---

## Summary Statistics

### By Phase

| Phase | Total Files | Complete | In Progress | Not Started |
|---|---|---|---|---|
| Phase 4 | 2 | 1 | 1 | 0 |
| Phase 4.5 | 2 | 0 | 0 | 2 |
| Phase 5a | 4 | 1 | 0 | 3 |
| Phase 5b | 5 | 0 | 0 | 5 |
| Phase 5c | 5 | 3 | 0 | 2 |
| Phase 5d | 4 | 1 | 0 | 3 |
| Phase 6 | 5 | 0 | 0 | 5 |
| Phase 6.5 | 1 | 0 | 0 | 1 |
| Phase 7 | 2 | 0 | 0 | 2 |
| Phase 8 | 3 | 0 | 0 | 3 |
| Phase 8.5 | 1 | 0 | 0 | 1 |
| Phase 9 | 13 | 3 | 0 | 10 |
| Reference | 9 | - | - | - |

### By Priority

| Priority | Count | Estimated Effort |
|---|---|---|
| 🔴 Critical | 12 | 12 days |
| 🟡 High | 10 | 8 days |
| 🟢 Medium | 8 | 5 days |
| ⚪ Low | 4 | 2 days |
| ✅ Complete | 8 | - |

**Total Estimated Effort**: ~27 days (excluding completed work)

---

## Quick Reference: Files with Ready-to-Use Code

These files contain complete, production-ready implementations:

1. ✅ **Magnetic Timeline Logic.md** - Complete Svelte store code
2. ✅ **Snap-to-Grid logic.md** - Complete snapping logic
3. ✅ **The Scrubbable Input Component.md** - Complete component
4. ✅ **HUD (Heads-Up Display).md** - Complete HUD system
5. ✅ **Smart Alignment.md** - Complete snapping utility
6. ✅ **Enhanced Engine Sync with Auto-Detection.md** - Complete auto-throttling
7. ✅ **Missing Pieces.md** - Command Pattern, Audio strategy, Project format
8. ✅ **The Progress Parser Logic.md** - FFmpeg regex parser
9. ✅ **INSPIRATIONS.md** - Timeline struct, Interpolation, Commands

**Recommendation**: Prioritize these files - they're ready to copy-paste and integrate.

---

## Next Actions

### Immediate (This Week)
1. [ ] Fix delta time in render loop (Phase 4)
2. [ ] Review and consolidate `capcut copy` files
3. [ ] Plan Phase 4.5 (Audio) implementation

### Short-term (Phase 5)
1. [ ] Implement Timeline struct (5a)
2. [ ] Integrate Magnetic Timeline Logic (5d)
3. [ ] Add Snap-to-Grid (5d)

### Medium-term (Phase 6-8)
1. [ ] Implement Command Pattern (6.5)
2. [ ] Add keybinding system (6)
3. [ ] Build export pipeline (8)

---

**Last Updated**: January 2026  
**Completion**: 18% (9/51 files)  
**Next Milestone**: Phase 5a (Timeline Backend)
