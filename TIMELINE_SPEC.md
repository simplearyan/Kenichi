# Timeline Specification

## Overview

This document provides comprehensive specifications for the Kenichi timeline, including UI aesthetics, editing tools, magnetic logic, snap-to-grid, and performance optimizations.

**Reference Files**:
- `guide/Timeline UI.md`
- `guide/Magnetic Timeline Logic.md`
- `guide/Snap-to-Grid logic.md`

---

## 1. Timeline UI Aesthetics & Layout

### 1.1 Visual Design

**Color Palette**:
- **Background**: Deep charcoal `#121212`
- **Track Background**: Lighter gray `#1E1E1E`
- **Playhead**: Cyan `#00C4CC` (high visibility)
- **Selected Clip Border**: Cyan `#00C4CC`
- **Clip Background**: `#2A2A2A` with `4px` rounded corners

**Typography**:
- **Timestamps**: `10px` monospace (prevents number jumping)
- **Clip Labels**: `11px` sans-serif
- **Track Names**: `11px` bold uppercase

**Visual Density**:
- Clips have `4px` rounded corners
- Thin `1px` borders
- Selected clips: cyan border + trim handles
- Negative space creates visual hierarchy

### 1.2 Layout Structure

```
┌─────────────────────────────────────────────────────────────┐
│  Timeline Toolbar (32px height)                              │
│  ┌──────────────────┐              ┌──────────────────┐     │
│  │ Left: Actions    │              │ Right: View      │     │
│  │ Undo/Redo/Split  │              │ Magnet/Snap/Zoom │     │
│  └──────────────────┘              └──────────────────┘     │
├─────────────────────────────────────────────────────────────┤
│  Timecode Ruler (24px height)                                │
│  00:00  00:05  00:10  00:15  00:20  00:25  00:30            │
├──────┬──────────────────────────────────────────────────────┤
│Track │                                                       │
│Names │  Track 1 (Main)    [Clip 1] [Clip 2]    [Clip 3]    │
│(80px)│                                                       │
│      │  Track 2 (Overlay)      [Clip 4]                     │
│      │                                                       │
│      │  Track 3 (Text)              [Clip 5]                │
│      │                                                       │
└──────┴──────────────────────────────────────────────────────┘
```

### 1.3 Layering Logic

**Top-Down Priority**:
- Top tracks render **over** bottom tracks
- Track 1 (Main): Ripple deletes affect entire timeline
- Track 2+ (Overlays): Independent unless linked

---

## 2. Timeline Toolbars

### 2.1 Left Toolbar (Action Zone)

**Destructive & Utility Tools**:

```svelte
<div class="toolbar-left flex items-center gap-2">
  <!-- Undo/Redo -->
  <button class="nle-icon-btn" on:click={undo} disabled={!canUndo}>
    <i class="i-lucide-undo" />
  </button>
  <button class="nle-icon-btn" on:click={redo} disabled={!canRedo}>
    <i class="i-lucide-redo" />
  </button>
  
  <div class="w-1px h-20px bg-kenichi-border" />
  
  <!-- Split (B) -->
  <button class="nle-icon-btn" on:click={splitAtPlayhead} title="Split (B)">
    <i class="i-lucide-scissors" />
  </button>
  
  <!-- Delete -->
  <button class="nle-icon-btn" on:click={deleteSelected} title="Delete">
    <i class="i-lucide-trash-2" />
  </button>
  
  <!-- Freeze/Reverse -->
  <button class="nle-icon-btn" on:click={freezeFrame} title="Freeze Frame">
    <i class="i-lucide-pause-circle" />
  </button>
  <button class="nle-icon-btn" on:click={reverseClip} title="Reverse">
    <i class="i-lucide-rewind" />
  </button>
</div>
```

### 2.2 Right Toolbar (View Zone)

**Perception Controls**:

```svelte
<div class="toolbar-right flex items-center gap-2">
  <!-- Magnet Toggle -->
  <button 
    class="nle-icon-btn" 
    class:active={$magneticMode}
    on:click={() => magneticMode.update(v => !v)}
    title="Magnetic Timeline (M)"
  >
    <i class="i-lucide-magnet" />
  </button>
  
  <!-- Snap Toggle -->
  <button 
    class="nle-icon-btn"
    class:active={$snapEnabled}
    on:click={() => snapEnabled.update(v => !v)}
    title="Snap to Grid (S)"
  >
    <i class="i-lucide-grid-3x3" />
  </button>
  
  <div class="w-1px h-20px bg-kenichi-border" />
  
  <!-- Zoom Slider -->
  <div class="flex items-center gap-2 w-120px">
    <i class="i-lucide-zoom-out text-xs" />
    <Slider.Root 
      bind:value={$zoomLevel} 
      min={10} 
      max={500} 
      step={10}
      class="flex-1"
    />
    <i class="i-lucide-zoom-in text-xs" />
  </div>
</div>
```

---

## 3. Timeline Zoom Logic

### 3.1 Zoom Mechanics

**Interaction Methods**:
- Slider in toolbar
- `Ctrl + Scroll` (Desktop)
- `Pinch` gesture (Touch)

**Anchor Point**: **Playhead-Centric**
- Timeline expands outward from playhead
- Current frame stays centered
- Prevents disorientation

**Mathematics**:
```typescript
// src/lib/stores/timeline.ts
export const zoomLevel = writable(100); // 100px per second

// Low Zoom: 1 second = 10px (overview)
// Medium Zoom: 1 second = 100px (editing)
// High Zoom: 1 second = 500px (frame-accurate)

// Calculate clip width
$: clipWidth = clip.duration * $zoomLevel;
```

**Visual Feedback**:
- Ruler increments change with zoom
- Low zoom: 10s intervals
- Medium zoom: 1s intervals
- High zoom: Frame numbers (`01:15f`)

### 3.2 Implementation

```svelte
<script>
  import { zoomLevel, playheadTime } from '$lib/stores/timeline';
  
  let timelineContainer: HTMLElement;
  
  function handleZoom(delta: number) {
    zoomLevel.update(z => {
      const newZoom = Math.max(10, Math.min(500, z + delta));
      
      // Keep playhead centered
      if (timelineContainer) {
        const centerOffset = $playheadTime * newZoom;
        timelineContainer.scrollLeft = centerOffset - timelineContainer.clientWidth / 2;
      }
      
      return newZoom;
    });
  }
  
  function handleWheel(e: WheelEvent) {
    if (e.ctrlKey) {
      e.preventDefault();
      handleZoom(-e.deltaY * 0.1);
    }
  }
</script>

<div 
  bind:this={timelineContainer}
  class="timeline-container"
  on:wheel={handleWheel}
>
  <!-- Timeline content -->
</div>
```

**Performance Optimization**:
```svelte
<!-- DON'T: Recalculate positions -->
{#each clips as clip}
  <div style="left: {calculatePosition(clip)}px" />
{/each}

<!-- DO: Use CSS transform -->
<div 
  class="timeline-content"
  style="transform: scaleX({$zoomLevel / 100})"
>
  {#each clips as clip}
    <div style="left: {clip.start * 100}px; width: {clip.duration * 100}px" />
  {/each}
</div>
```

---

## 4. Magnetic Timeline Logic

### 4.1 Core Concept

**Ripple Delete**: When a clip is removed, all following clips on the same track shift left to close the gap.

**Benefits**:
- No accidental gaps (prevents black frames)
- Maintains story flow
- Mobile-first UX (CapCut standard)

### 4.2 Implementation

**Store** (`src/lib/stores/timeline.ts`):
```typescript
import { writable, get } from 'svelte/store';

export const clips = writable<Clip[]>([]);
export const magneticMode = writable(true);

export function removeClip(id: string) {
  clips.update($clips => {
    const index = $clips.findIndex(c => c.id === id);
    if (index === -1) return $clips;
    
    const deletedClip = $clips[index];
    const isMagnetic = get(magneticMode);
    
    // 1. Remove clip
    let nextClips = $clips.filter(c => c.id !== id);
    
    // 2. Ripple: Shift following clips
    if (isMagnetic) {
      nextClips = nextClips.map(clip => {
        // Only affect clips on same track that start AFTER deleted clip
        if (clip.trackId === deletedClip.trackId && clip.start > deletedClip.start) {
          return {
            ...clip,
            start: clip.start - deletedClip.duration
          };
        }
        return clip;
      });
    }
    
    return nextClips;
  });
}
```

### 4.3 Visual Animation

**Smooth Slide Effect**:
```svelte
<script>
  import { flip } from 'svelte/animate';
  import { clips, removeClip, frameWidth } from '$lib/stores/timeline';
  
  export let trackId: number;
  
  $: trackClips = $clips.filter(c => c.trackId === trackId);
</script>

<div class="track-lane">
  {#each trackClips as clip (clip.id)}
    <div 
      animate:flip={{ duration: 300 }}
      class="clip-block"
      style="left: {clip.start * $frameWidth}px; width: {clip.duration * $frameWidth}px"
      on:contextmenu|preventDefault={() => removeClip(clip.id)}
    >
      <span class="clip-label">{clip.name}</span>
    </div>
  {/each}
</div>
```

**Why `flip` Animation**:
- GPU-accelerated (CSS transforms)
- Smooth on GT 740
- Prevents disorienting "teleport"

### 4.4 Gap Closing

**Manual Gap Removal**:
```typescript
export function closeGap(trackId: number, startTime: number, gapDuration: number) {
  clips.update($clips => $clips.map(clip => {
    if (clip.trackId === trackId && clip.start > startTime) {
      return { ...clip, start: clip.start - gapDuration };
    }
    return clip;
  }));
}
```

---

## 5. Snap-to-Grid Logic

### 5.1 Core Concept

**Frame-Accurate Positioning**: Clips snap to frame boundaries, preventing "between frames" placement.

**Benefits**:
- No black frames
- Temporal precision
- Professional workflow

### 5.2 Implementation

**Store** (`src/lib/stores/timeline.ts`):
```typescript
import { writable, derived } from 'svelte/store';

export const zoomLevel = writable(100); // px per second
export const snapEnabled = writable(true);
export const fps = writable(30);

// Calculate pixel width of single frame
export const frameWidth = derived([zoomLevel, fps], ([$zoom, $fps]) => {
  return $zoom / $fps;
});

// Snapping function
export const snapToGrid = derived([frameWidth, snapEnabled], ([$frameWidth, $enabled]) => {
  return (rawX: number): number => {
    if (!$enabled) return rawX;
    
    // Snap to nearest frame boundary
    return Math.round(rawX / $frameWidth) * $frameWidth;
  };
});
```

### 5.3 Visual Feedback

**Snap Guide Line**:
```svelte
<script>
  import { snapToGrid } from '$lib/stores/timeline';
  export let rawX: number;
  
  $: snappedX = $snapToGrid(rawX);
  $: isSnapping = Math.abs(snappedX - rawX) > 0.1;
</script>

{#if isSnapping}
  <div 
    class="snap-guide"
    style="left: {snappedX}px"
  />
{/if}

<style>
  .snap-guide {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 1px;
    background: var(--brand-accent);
    box-shadow: 0 0 8px rgba(0, 196, 204, 0.8);
    z-index: 50;
    pointer-events: none;
  }
</style>
```

### 5.4 Draggable Clips

```svelte
<script>
  import { snapToGrid } from '$lib/stores/timeline';
  
  let clipX = 0;
  let isDragging = false;
  
  function handleDrag(e: MouseEvent) {
    if (!isDragging) return;
    
    const rawX = e.clientX - timelineOffset;
    clipX = $snapToGrid(rawX); // Snap to frame boundary
  }
</script>

<div 
  class="clip-block"
  style="transform: translateX({clipX}px)"
  on:mousedown={() => isDragging = true}
  on:mousemove={handleDrag}
  on:mouseup={() => isDragging = false}
>
  <span>{clip.name}</span>
</div>
```

### 5.5 Advanced: Snap to Multiple Targets

```typescript
export function snapToTargets(
  rawX: number,
  targets: number[],
  threshold: number = 10
): number {
  if (!get(snapEnabled)) return rawX;
  
  // Check if close to any target
  for (const target of targets) {
    if (Math.abs(rawX - target) <= threshold) {
      return target;
    }
  }
  
  // Otherwise snap to grid
  return get(snapToGrid)(rawX);
}

// Usage: Snap to playhead or other clips
const targets = [
  $playheadTime * $frameWidth,
  ...otherClips.map(c => c.start * $frameWidth),
  ...otherClips.map(c => (c.start + c.duration) * $frameWidth)
];

const snappedX = snapToTargets(rawX, targets);
```

---

## 6. Track Management

### 6.1 Dynamic Track Allocation

**CapCut-Style**: No "Add Track" button

**Logic**:
- Drag clip to empty space above/below tracks
- Timeline automatically creates new track
- "Ghost box" shows where track will appear

**Implementation**:
```svelte
<script>
  let tracks = writable([{ id: 1, name: 'Main' }]);
  let dragOverY = -1;
  
  function handleDragOver(e: DragEvent) {
    const trackHeight = 80;
    const trackIndex = Math.floor(e.offsetY / trackHeight);
    
    if (trackIndex >= $tracks.length) {
      dragOverY = $tracks.length * trackHeight;
    } else {
      dragOverY = -1;
    }
  }
  
  function handleDrop(e: DragEvent) {
    const trackHeight = 80;
    const trackIndex = Math.floor(e.offsetY / trackHeight);
    
    if (trackIndex >= $tracks.length) {
      // Create new track
      tracks.update(t => [...t, { 
        id: t.length + 1, 
        name: `Track ${t.length + 1}` 
      }]);
    }
    
    // Add clip to track
    addClipToTrack(trackIndex, clipData);
    dragOverY = -1;
  }
</script>

<div 
  class="timeline-tracks"
  on:dragover|preventDefault={handleDragOver}
  on:drop={handleDrop}
>
  {#each $tracks as track}
    <TrackLane {track} />
  {/each}
  
  {#if dragOverY >= 0}
    <div 
      class="ghost-track"
      style="top: {dragOverY}px"
    />
  {/if}
</div>
```

### 6.2 Track Types

**Main Track (Track 1)**:
- Ripple deletes affect entire timeline
- Primary story backbone
- Cannot be deleted

**Overlay Tracks (Track 2+)**:
- Independent unless linked
- Picture-in-picture, text, effects
- Can be deleted

**Track Linking**:
```typescript
export const linkedTracks = writable<Set<number>>(new Set());

export function toggleTrackLink(trackId: number) {
  linkedTracks.update(set => {
    if (set.has(trackId)) {
      set.delete(trackId);
    } else {
      set.add(trackId);
    }
    return new Set(set);
  });
}
```

---

## 7. Timecode Ruler

### 7.1 Dynamic Increments

**Zoom-Responsive**:
- Low zoom (10-50px/s): 10s intervals
- Medium zoom (50-200px/s): 1s intervals
- High zoom (200-500px/s): Frame numbers

**Implementation**:
```svelte
<script>
  import { zoomLevel, fps } from '$lib/stores/timeline';
  
  $: increment = $zoomLevel < 50 ? 10 : $zoomLevel < 200 ? 1 : 1 / $fps;
  $: markers = generateMarkers(duration, increment);
  
  function generateMarkers(duration: number, increment: number) {
    const markers = [];
    for (let time = 0; time <= duration; time += increment) {
      markers.push({
        time,
        position: time * $zoomLevel,
        label: formatTimecode(time, increment < 1),
        isMajor: time % (increment * 5) === 0
      });
    }
    return markers;
  }
  
  function formatTimecode(time: number, showFrames: boolean) {
    const minutes = Math.floor(time / 60);
    const seconds = Math.floor(time % 60);
    const frames = Math.floor((time % 1) * $fps);
    
    if (showFrames) {
      return `${pad(minutes)}:${pad(seconds)}f${pad(frames)}`;
    } else {
      return `${pad(minutes)}:${pad(seconds)}`;
    }
  }
  
  function pad(num: number) {
    return num.toString().padStart(2, '0');
  }
</script>

<div class="timecode-ruler">
  {#each markers as marker}
    <div 
      class="time-marker"
      class:major={marker.isMajor}
      style="left: {marker.position}px"
    >
      {#if marker.isMajor}
        <span class="time-label">{marker.label}</span>
      {/if}
      <div class="tick" class:major={marker.isMajor} />
    </div>
  {/each}
</div>

<style>
  .timecode-ruler {
    position: relative;
    height: 24px;
    background: var(--kenichi-panel);
    border-bottom: 1px solid var(--kenichi-border);
  }
  
  .time-marker {
    position: absolute;
    top: 0;
    height: 100%;
  }
  
  .time-label {
    font-size: 10px;
    font-family: 'JetBrains Mono', monospace;
    color: var(--kenichi-muted);
    position: absolute;
    top: 2px;
    left: 4px;
  }
  
  .tick {
    position: absolute;
    bottom: 0;
    width: 1px;
    height: 4px;
    background: var(--kenichi-border);
  }
  
  .tick.major {
    height: 8px;
    background: var(--kenichi-muted);
  }
</style>
```

---

## 8. Playhead

### 8.1 Visual Design

**Components**:
- **Head**: 12px circle (draggable)
- **Line**: 2px vertical line (spans all tracks)
- **Color**: Cyan `#00C4CC`
- **Shadow**: Glow effect for visibility

**Implementation**:
```svelte
<script>
  import { playheadTime, zoomLevel } from '$lib/stores/timeline';
  
  let isDragging = false;
  
  function handleDrag(e: MouseEvent) {
    if (!isDragging) return;
    
    const rect = timelineContainer.getBoundingClientRect();
    const x = e.clientX - rect.left + timelineContainer.scrollLeft;
    const time = x / $zoomLevel;
    
    playheadTime.set(Math.max(0, Math.min(duration, time)));
    invoke('seek', { time: $playheadTime });
  }
</script>

<div 
  class="playhead"
  style="left: {$playheadTime * $zoomLevel}px"
>
  <div 
    class="playhead-head"
    on:mousedown={() => isDragging = true}
  />
  <div class="playhead-line" />
</div>

<svelte:window 
  on:mousemove={handleDrag}
  on:mouseup={() => isDragging = false}
/>

<style>
  .playhead {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px;
    pointer-events: none;
    z-index: 100;
  }
  
  .playhead-head {
    width: 12px;
    height: 12px;
    background: var(--brand-accent);
    border-radius: 50%;
    margin-left: -5px;
    pointer-events: all;
    cursor: ew-resize;
    box-shadow: 0 0 4px rgba(0, 196, 204, 0.8);
  }
  
  .playhead-line {
    width: 2px;
    height: 100%;
    background: var(--brand-accent);
    box-shadow: 0 0 4px rgba(0, 196, 204, 0.5);
  }
</style>
```

---

## 9. Clip Components

### 9.1 Clip Block

**Features**:
- Thumbnail preview
- Duration label
- Trim handles (left/right)
- Selection state
- Context menu

**Implementation**:
```svelte
<script>
  import { selectedClip, frameWidth } from '$lib/stores/timeline';
  
  export let clip: Clip;
  
  $: isSelected = $selectedClip === clip.id;
  
  function handleSelect() {
    selectedClip.set(clip.id);
  }
</script>

<div 
  class="clip-block"
  class:selected={isSelected}
  style="left: {clip.start * $frameWidth}px; width: {clip.duration * $frameWidth}px"
  on:click={handleSelect}
  on:contextmenu|preventDefault={handleContextMenu}
>
  <!-- Thumbnail -->
  {#if clip.thumbnail}
    <img src={clip.thumbnail} alt="" class="clip-thumbnail" />
  {/if}
  
  <!-- Label -->
  <span class="clip-label">{clip.name}</span>
  
  <!-- Trim Handles (only when selected) -->
  {#if isSelected}
    <div class="trim-handle trim-left" use:trimHandle={{ side: 'left', clip }} />
    <div class="trim-handle trim-right" use:trimHandle={{ side: 'right', clip }} />
  {/if}
</div>

<style>
  .clip-block {
    position: absolute;
    height: 100%;
    background: var(--kenichi-surface);
    border: 1px solid var(--kenichi-border);
    border-left: 2px solid var(--brand-accent);
    border-radius: 4px;
    overflow: hidden;
    cursor: grab;
    transition: border-color 0.2s;
  }
  
  .clip-block.selected {
    border-color: var(--brand-accent);
    box-shadow: 0 0 0 2px var(--brand-accent);
  }
  
  .clip-thumbnail {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    opacity: 0.3;
  }
  
  .clip-label {
    position: relative;
    font-size: 11px;
    padding: 4px;
    color: white;
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }
  
  .trim-handle {
    position: absolute;
    width: 8px;
    height: 100%;
    background: var(--brand-accent);
    cursor: ew-resize;
    opacity: 0;
    transition: opacity 0.2s;
  }
  
  .clip-block:hover .trim-handle {
    opacity: 1;
  }
  
  .trim-left {
    left: 0;
  }
  
  .trim-right {
    right: 0;
  }
</style>
```

### 9.2 Trim Handle Action

```typescript
// src/lib/actions/trimHandle.ts
export function trimHandle(node: HTMLElement, params: { side: 'left' | 'right', clip: Clip }) {
  let isDragging = false;
  let startX = 0;
  let startValue = 0;
  
  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    startX = e.clientX;
    startValue = params.side === 'left' ? params.clip.offset : params.clip.duration;
    
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
  }
  
  function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;
    
    const deltaX = e.clientX - startX;
    const deltaTime = deltaX / get(zoomLevel);
    
    if (params.side === 'left') {
      // Trim in-point
      const newOffset = Math.max(0, startValue + deltaTime);
      updateClip(params.clip.id, { offset: newOffset });
    } else {
      // Trim out-point
      const newDuration = Math.max(0.1, startValue + deltaTime);
      updateClip(params.clip.id, { duration: newDuration });
    }
  }
  
  function handleMouseUp() {
    isDragging = false;
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);
  }
  
  node.addEventListener('mousedown', handleMouseDown);
  
  return {
    destroy() {
      node.removeEventListener('mousedown', handleMouseDown);
    }
  };
}
```

---

## 10. Performance Optimizations

### 10.1 Virtual Scrolling

**Problem**: 100+ clips cause lag

**Solution**: Only render visible clips

```svelte
<script>
  import { onMount } from 'svelte';
  
  let visibleClips = [];
  let scrollLeft = 0;
  let containerWidth = 0;
  
  $: {
    const startTime = scrollLeft / $zoomLevel;
    const endTime = (scrollLeft + containerWidth) / $zoomLevel;
    
    visibleClips = $clips.filter(clip => {
      const clipEnd = clip.start + clip.duration;
      return clipEnd >= startTime && clip.start <= endTime;
    });
  }
</script>

<div 
  class="timeline-container"
  on:scroll={e => scrollLeft = e.target.scrollLeft}
  bind:clientWidth={containerWidth}
>
  {#each visibleClips as clip (clip.id)}
    <ClipBlock {clip} />
  {/each}
</div>
```

### 10.2 Debounced Updates

**Problem**: Dragging sends 60+ updates/second

**Solution**: Debounce backend sync

```typescript
import { debounce } from '$lib/utils/debounce';

const syncToBackend = debounce((clips: Clip[]) => {
  invoke('update_composition', { clips });
}, 100);

clips.subscribe(syncToBackend);
```

### 10.3 CSS Transform Zoom

**Problem**: Recalculating positions is expensive

**Solution**: Use CSS `scaleX`

```svelte
<div 
  class="timeline-content"
  style="transform: scaleX({$zoomLevel / 100}); transform-origin: 0 0"
>
  <!-- Clips with fixed positions at 100px/s -->
</div>
```

---

## 11. Implementation Checklist

### Phase 5c (Timeline UI - Static)
- [ ] Create timeline container with toolbar
- [ ] Implement timecode ruler with dynamic increments
- [ ] Create track lane components
- [ ] Add playhead with visual design
- [ ] Implement clip blocks with thumbnails

### Phase 5d (Timeline UI - Interactive)
- [ ] Implement magnetic timeline logic
- [ ] Add snap-to-grid functionality
- [ ] Create draggable clips
- [ ] Add trim handles
- [ ] Implement zoom controls
- [ ] Add dynamic track allocation

### Phase 6 (Editing Tools)
- [ ] Implement split tool
- [ ] Add delete with ripple
- [ ] Create selection tool
- [ ] Add multi-select
- [ ] Implement context menus

---

## References

- `guide/Timeline UI.md` - Aesthetics and layout
- `guide/Magnetic Timeline Logic.md` - Ripple delete implementation
- `guide/Snap-to-Grid logic.md` - Frame-accurate positioning
- `guide/CapCut Desktop.md` - Professional NLE patterns

---

**Last Updated**: January 2026  
**Status**: Specification Complete  
**Next**: Implement in Phase 5c-5d
