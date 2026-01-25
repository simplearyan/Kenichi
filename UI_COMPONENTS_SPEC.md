# UI Components Specification

## Overview

This document specifies all UI components for Kenichi, based on research from CapCut Desktop and professional NLE workflows.

---

## Component Categories

1. **Input Components** - Scrubbable inputs, sliders, dropdowns
2. **Timeline Components** - Clips, tracks, rulers, playhead
3. **Preview Components** - WGPU viewport, overlays, controls
4. **Panel Components** - Media library, inspector, timeline
5. **Modal Components** - Export, keybindings, settings
6. **Feedback Components** - HUD, notifications, tooltips

---

## 1. Input Components

### Scrubbable Input

**Purpose**: Drag-to-change number inputs (CapCut-style)  
**Reference**: `guide/The Scrubbable Input Component.md`

**Features**:
- Horizontal drag to change value
- Shift key: 5x speed (fast scrubbing)
- Alt key: 0.1x speed (fine-tuning)
- Double-click: Direct text input
- Visual feedback (border highlight when dragging)

**Implementation**:
```svelte
<ScrubbableInput 
  label="Scale" 
  bind:value={scale} 
  min={0} 
  max={1000} 
  step={0.5} 
  suffix="%" 
/>
```

**Props**:
- `value: number` - Current value
- `label: string` - Label text
- `step: number` - Increment per pixel (default: 1)
- `min: number` - Minimum value (default: -Infinity)
- `max: number` - Maximum value (default: Infinity)
- `suffix: string` - Unit suffix (%, °, px, etc.)

**Events**:
- `on:change` - Fires when value changes

**Styling**:
- Uses `nle-input` UnoCSS shortcut
- Border changes to `brand-accent` when dragging
- Cursor changes to `ew-resize` during drag

---

### Slider

**Purpose**: Visual range selection

**Implementation**:
```svelte
<Slider.Root 
  bind:value={opacity} 
  min={0} 
  max={100} 
  step={1}
  class="w-full"
>
  <Slider.Track class="bg-kenichi-border h-2px">
    <Slider.Range class="bg-brand-accent" />
  </Slider.Track>
  <Slider.Thumb class="w-12px h-12px bg-white rounded-full" />
</Slider.Root>
```

---

### Dropdown

**Purpose**: Select from predefined options

**Implementation**:
```svelte
<Select.Root bind:value={blendMode}>
  <Select.Trigger class="nle-input">
    <Select.Value placeholder="Blend Mode" />
  </Select.Trigger>
  <Select.Content class="bg-kenichi-panel border border-kenichi-border">
    <Select.Item value="normal">Normal</Select.Item>
    <Select.Item value="multiply">Multiply</Select.Item>
    <Select.Item value="screen">Screen</Select.Item>
  </Select.Content>
</Select.Root>
```

---

## 2. Timeline Components

### Clip Block

**Purpose**: Visual representation of a clip on the timeline

**Features**:
- Thumbnail preview
- Duration label
- Trim handles (left/right)
- Drag-and-drop
- Selection state
- Magnetic snapping

**Implementation**:
```svelte
<div 
  class="clip-block"
  style="left: {clip.start * pixelsPerSecond}px; width: {clip.duration * pixelsPerSecond}px"
  class:selected={$selectedClip === clip.id}
  use:draggable
  use:snapToGrid
>
  <img src={clip.thumbnail} alt="" class="clip-thumbnail" />
  <span class="clip-label">{clip.name}</span>
  <div class="trim-handle trim-left" use:trimHandle={{ side: 'left' }} />
  <div class="trim-handle trim-right" use:trimHandle={{ side: 'right' }} />
</div>
```

**Styling**:
```css
.clip-block {
  position: absolute;
  height: 40px;
  background: var(--kenichi-surface);
  border: 1px solid var(--kenichi-border);
  border-left: 2px solid var(--brand-accent);
  border-radius: 2px;
  overflow: hidden;
  cursor: grab;
}

.clip-block.selected {
  border-color: var(--brand-accent);
  box-shadow: 0 0 0 2px var(--brand-accent);
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
```

---

### Track Lane

**Purpose**: Container for clips on a single track

**Implementation**:
```svelte
<div class="track-lane" data-track-id={track.id}>
  <div class="track-header">
    <span class="track-name">{track.name}</span>
    <button class="track-mute" class:active={track.muted}>
      <i class="i-lucide-volume-x" />
    </button>
  </div>
  <div class="track-content" use:dropzone>
    {#each track.clips as clip (clip.id)}
      <ClipBlock {clip} />
    {/each}
  </div>
</div>
```

---

### Timecode Ruler

**Purpose**: Show time markers and current playhead position

**Features**:
- Major ticks (seconds)
- Minor ticks (frames)
- Timecode labels
- Click to seek

**Implementation**:
```svelte
<div class="timecode-ruler" on:click={handleSeek}>
  {#each timeMarkers as marker}
    <div 
      class="time-marker" 
      style="left: {marker.position}px"
      class:major={marker.isMajor}
    >
      {#if marker.isMajor}
        <span class="time-label">{marker.label}</span>
      {/if}
    </div>
  {/each}
</div>
```

---

### Playhead

**Purpose**: Visual indicator of current playback position

**Features**:
- Vertical line spanning all tracks
- Draggable for scrubbing
- Snaps to frames

**Implementation**:
```svelte
<div 
  class="playhead" 
  style="left: {$currentTime * pixelsPerSecond}px"
  use:draggable={{ axis: 'x', onDrag: handleScrub }}
>
  <div class="playhead-head" />
  <div class="playhead-line" />
</div>
```

**Styling**:
```css
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
}

.playhead-line {
  width: 2px;
  height: 100%;
  background: var(--brand-accent);
  box-shadow: 0 0 4px rgba(0, 196, 204, 0.5);
}
```

---

## 3. Preview Components

### Safe Zone Overlay

**Purpose**: Show platform-specific safe zones  
**Reference**: `guide/Safe Area Overlay.md`

**Platforms**:
- YouTube Shorts: 15% top, 35% bottom, 18% right
- TikTok: 12% top, 25% bottom
- Instagram: Similar to TikTok

**Implementation**:
```svelte
<script>
  import { activeSafeZone } from '$lib/stores/safeArea';
</script>

{#if $activeSafeZone !== 'none'}
  <div class="safe-zone-overlay" transition:fade>
    {#if $activeSafeZone === 'shorts'}
      <!-- Top UI (Title/Search) -->
      <div class="safe-zone-top" style="height: 15%">
        <span class="zone-label">Top UI (Title/Search)</span>
      </div>
      
      <!-- Bottom UI (Channel/Audio) -->
      <div class="safe-zone-bottom" style="height: 35%">
        <span class="zone-label">Bottom UI (Channel/Audio)</span>
      </div>
      
      <!-- Right Buttons (Like/Comment) -->
      <div class="safe-zone-right" style="width: 18%">
        <span class="zone-label">Buttons</span>
      </div>
    {/if}
  </div>
{/if}
```

**Styling**:
```css
.safe-zone-overlay {
  position: absolute;
  inset: 0;
  pointer-events: none;
  z-index: 20;
}

.safe-zone-top,
.safe-zone-bottom {
  position: absolute;
  left: 0;
  right: 0;
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(1px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.safe-zone-top {
  top: 0;
}

.safe-zone-bottom {
  bottom: 0;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.safe-zone-right {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  height: 40%;
  background: rgba(0, 0, 0, 0.3);
  border-left: 1px solid rgba(255, 255, 255, 0.1);
}

.zone-label {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.4);
  text-transform: uppercase;
  padding: 4px;
}
```

---

### Playback Controls

**Purpose**: Play, pause, seek controls

**Implementation**:
```svelte
<div class="playback-controls">
  <button class="control-btn" on:click={handlePlayPause}>
    <i class={$isPlaying ? 'i-lucide-pause' : 'i-lucide-play'} />
  </button>
  
  <span class="timecode">{formatTimecode($currentTime)}</span>
  <span class="timecode-separator">/</span>
  <span class="timecode">{formatTimecode($duration)}</span>
  
  <div class="spacer" />
  
  <button class="control-btn" on:click={toggleSafeZone}>
    <i class="i-lucide-smartphone" />
  </button>
  
  <button class="control-btn" on:click={toggleFullscreen}>
    <i class="i-lucide-maximize" />
  </button>
</div>
```

---

## 4. Panel Components

### Media Library

**Purpose**: Browse and import media assets

**Features**:
- Grid/list view toggle
- Search/filter
- Drag-and-drop to timeline
- Thumbnail generation
- File metadata display

**Implementation**:
```svelte
<div class="media-library">
  <div class="media-header">
    <input 
      type="search" 
      placeholder="Search media..." 
      class="media-search"
    />
    <button class="view-toggle" on:click={toggleView}>
      <i class={viewMode === 'grid' ? 'i-lucide-grid' : 'i-lucide-list'} />
    </button>
  </div>
  
  <div class="media-grid" class:list-view={viewMode === 'list'}>
    {#each assets as asset (asset.id)}
      <div 
        class="media-item" 
        draggable="true"
        on:dragstart={e => handleDragStart(e, asset)}
      >
        <img src={asset.thumbnail} alt={asset.name} />
        <span class="media-name">{asset.name}</span>
        <span class="media-duration">{formatDuration(asset.duration)}</span>
      </div>
    {/each}
  </div>
</div>
```

---

### Inspector Panel

**Purpose**: Edit properties of selected clip

**Features**:
- Accordion sections (Transform, Effects, Audio)
- Scrubbable inputs
- Keyframe editor
- Context-sensitive (shows relevant properties)

**Implementation**:
```svelte
<Accordion.Root class="inspector-panel">
  <Accordion.Item value="transform">
    <Accordion.Header>
      <Accordion.Trigger class="accordion-trigger">
        Transform
      </Accordion.Trigger>
    </Accordion.Header>
    <Accordion.Content class="accordion-content">
      <ScrubbableInput label="Scale" bind:value={scale} suffix="%" />
      <div class="input-group">
        <ScrubbableInput label="X" bind:value={positionX} />
        <ScrubbableInput label="Y" bind:value={positionY} />
      </div>
      <ScrubbableInput label="Rotate" bind:value={rotation} suffix="°" />
      <ScrubbableInput label="Opacity" bind:value={opacity} suffix="%" />
    </Accordion.Content>
  </Accordion.Item>
</Accordion.Root>
```

---

## 5. Modal Components

### Export Modal

**Purpose**: Configure and start export  
**Reference**: `guide/The Export Modal UI (Svelte & Bits UI).md`

**Features**:
- Preset selection (1080p, 4K, web)
- Custom settings
- Progress bar
- Cancel button

**Implementation**:
```svelte
<Dialog.Root bind:open={exportModalOpen}>
  <Dialog.Trigger class="nle-button-primary">Export</Dialog.Trigger>
  
  <Dialog.Content class="export-modal">
    <Dialog.Title>Export Video</Dialog.Title>
    
    <div class="preset-selector">
      <button class="preset" on:click={() => selectPreset('1080p')}>
        1080p (Full HD)
      </button>
      <button class="preset" on:click={() => selectPreset('4k')}>
        4K (Ultra HD)
      </button>
      <button class="preset" on:click={() => selectPreset('web')}>
        Web Optimized
      </button>
    </div>
    
    {#if exporting}
      <div class="export-progress">
        <progress value={exportProgress} max={100} />
        <span>{exportProgress}%</span>
        <span class="time-remaining">{timeRemaining} remaining</span>
      </div>
      <button class="nle-button-primary" on:click={cancelExport}>
        Cancel
      </button>
    {:else}
      <button class="nle-button-primary" on:click={startExport}>
        Start Export
      </button>
    {/if}
  </Dialog.Content>
</Dialog.Root>
```

---

### Keybindings Modal

**Purpose**: Show all keyboard shortcuts  
**Reference**: `guide/Keybindings Modal.md`

**Features**:
- Searchable
- Category grouping
- Visual keyboard layout

**Implementation**:
```svelte
<Dialog.Root bind:open={keybindingsModalOpen}>
  <Dialog.Content class="keybindings-modal">
    <Dialog.Title>Keyboard Shortcuts</Dialog.Title>
    
    <input 
      type="search" 
      placeholder="Search shortcuts..." 
      bind:value={searchQuery}
    />
    
    <div class="shortcuts-list">
      {#each filteredShortcuts as category}
        <div class="shortcut-category">
          <h3>{category.name}</h3>
          {#each category.shortcuts as shortcut}
            <div class="shortcut-item">
              <span class="shortcut-action">{shortcut.action}</span>
              <kbd class="shortcut-key">{shortcut.key}</kbd>
            </div>
          {/each}
        </div>
      {/each}
    </div>
  </Dialog.Content>
</Dialog.Root>
```

---

## 6. Feedback Components

### HUD (Heads-Up Display)

**Purpose**: Visual feedback for keyboard shortcuts  
**Reference**: `guide/HUD (Heads-Up Display).md`

**Features**:
- Appears in center of preview
- Auto-dismisses after 1.5s
- Shows icon + text
- Non-blocking (pointer-events: none)

**Implementation**:
```svelte
<script>
  import { hudMessage } from '$lib/stores/hud';
  import { fly, fade } from 'svelte/transition';
</script>

{#if $hudMessage}
  {#key $hudMessage.id}
    <div 
      class="hud-overlay"
      in:fly={{ y: 10, duration: 200 }}
      out:fade={{ duration: 150 }}
    >
      <div class="hud-content">
        <i class={$hudMessage.icon} />
        <span>{$hudMessage.text}</span>
      </div>
    </div>
  {/key}
{/if}
```

**Styling**:
```css
.hud-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;
  z-index: 50;
}

.hud-content {
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 12px 24px;
  border-radius: 9999px;
  display: flex;
  align-items: center;
  gap: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
}

.hud-content i {
  color: var(--brand-accent);
  font-size: 20px;
}

.hud-content span {
  font-size: 13px;
  font-weight: bold;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: rgba(255, 255, 255, 0.9);
}
```

**Usage**:
```typescript
import { showHud } from '$lib/stores/hud';

// Show HUD when action is performed
showHud('Clip Split', 'i-lucide-scissors');
showHud('Magnet ON', 'i-lucide-magnet');
showHud('Stress Test Running...', 'i-lucide-zap');
```

---

### Tooltip

**Purpose**: Show keyboard shortcuts on hover

**Implementation**:
```svelte
<button 
  class="nle-icon-btn"
  use:tooltip={{ text: 'Split Clip (B)' }}
>
  <i class="i-lucide-scissors" />
</button>
```

---

## Component Library Structure

```
src/lib/components/
├── ui/
│   ├── ScrubbableInput.svelte
│   ├── Slider.svelte
│   ├── Dropdown.svelte
│   ├── Tooltip.svelte
│   └── Resizable.svelte
├── timeline/
│   ├── ClipBlock.svelte
│   ├── TrackLane.svelte
│   ├── TimecodeRuler.svelte
│   ├── Playhead.svelte
│   └── Timeline.svelte
├── preview/
│   ├── VideoViewport.svelte
│   ├── SafeZoneOverlay.svelte
│   ├── PlaybackControls.svelte
│   └── HudOverlay.svelte
├── panels/
│   ├── MediaPanel.svelte
│   ├── Inspector.svelte
│   └── Timeline.svelte
├── modals/
│   ├── ExportModal.svelte
│   ├── KeybindingsModal.svelte
│   └── SettingsModal.svelte
└── layout/
    └── TrinityLayout.svelte
```

---

## Implementation Checklist

### Phase 5c (Timeline UI - Static)
- [ ] Create ClipBlock component
- [ ] Create TrackLane component
- [ ] Create TimecodeRuler component
- [ ] Create Playhead component
- [ ] Implement MediaPanel grid view

### Phase 5d (Timeline UI - Interactive)
- [ ] Add drag-and-drop to ClipBlock
- [ ] Implement ScrubbableInput component
- [ ] Add SafeZoneOverlay component
- [ ] Implement HudOverlay component

### Phase 6 (Editing Tools)
- [ ] Add trim handles to ClipBlock
- [ ] Implement multi-select
- [ ] Add context menus
- [ ] Integrate HUD with shortcuts

### Phase 8 (Export)
- [ ] Create ExportModal component
- [ ] Add progress bar
- [ ] Implement preset selection

---

## References

- `guide/The Scrubbable Input Component.md`
- `guide/Safe Area Overlay.md`
- `guide/HUD (Heads-Up Display).md`
- `guide/The Export Modal UI (Svelte & Bits UI).md`
- `guide/Keybindings Modal.md`
- `guide/CapCut Desktop.md`

---

**Last Updated**: January 2026  
**Status**: Specification Complete  
**Next**: Implement in Phase 5c-5d
