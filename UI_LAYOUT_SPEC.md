# UI Layout Specifications

## Overview

This document provides comprehensive UI layout specifications for Kenichi, based on research from CapCut Desktop and professional NLE workflows.

---

## Layout Modes

Kenichi supports **3 layout modes** optimized for different workflows:

### 1. Standard Edit Mode (16:9)
### 2. Vertical Edit Mode (9:16)  
### 3. Code/Scripting Mode

---

## 1. Standard Edit Mode (16:9 / Landscape)

**Purpose**: Traditional video editing workflow  
**Reference**: `guide/The Design Layout - Multi-Device Cohesion.md`

### Layout Structure

```
┌─────────────────────────────────────────────────────────────┐
│  Top Bar (40px) - Logo | Workspace Toggle | Export          │
├──────────────┬────────────────────────────┬─────────────────┤
│              │                            │                 │
│   Media      │      Preview (WGPU)        │   Inspector     │
│   Library    │      (Center Canvas)       │   (Properties)  │
│   (Left)     │                            │   (Right)       │
│              │                            │                 │
│   250px      │        flex-1              │     350px       │
│              │                            │                 │
├──────────────┴────────────────────────────┴─────────────────┤
│                                                              │
│                  Timeline (Bottom)                           │
│                  Multi-track Editor                          │
│                  Height: 200-400px                           │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Component Breakdown

#### Top Bar (40px)
```svelte
<div class="h-40px bg-kenichi-panel flex items-center px-4 border-b border-kenichi-border justify-between">
  <!-- Left: Logo -->
  <div class="text-brand-accent font-bold text-sm">KENICHI</div>
  
  <!-- Center: Workspace Toggle -->
  <ToggleGroup.Root bind:value={workspace} type="single">
    <ToggleGroup.Item value="edit">Edit</ToggleGroup.Item>
    <ToggleGroup.Item value="vertical">Vertical</ToggleGroup.Item>
    <ToggleGroup.Item value="code">Code</ToggleGroup.Item>
  </ToggleGroup.Root>
  
  <!-- Right: Export Button -->
  <button class="nle-button-primary">Export</button>
</div>
```

#### Media Library (Left - 250px)
- **Asset tabs**: Media, Audio, Text, Effects
- **Grid view** with thumbnails
- **Search/filter** bar
- **Drag-and-drop** to timeline

#### Preview (Center - flex-1)
- **WGPU Canvas** (full viewport)
- **Playback controls** (bottom overlay)
- **Safe zone overlays** (toggleable)
- **Aspect ratio**: Letterboxed to fit

#### Inspector (Right - 350px)
- **Accordion panels**: Transform, Effects, Audio
- **Scrubbable inputs** for properties
- **Keyframe editor**
- **Context-sensitive** (shows selected clip properties)

#### Timeline (Bottom - 200-400px)
- **Multi-track** lanes
- **Clip thumbnails**
- **Timecode ruler**
- **Zoom/pan** controls
- **Magnetic snapping** (toggleable)

---

## 2. Vertical Edit Mode (9:16 / Portrait)

**Purpose**: TikTok, Reels, Shorts editing  
**Reference**: `guide/The Vertical Center Layout.md`

### Layout Structure

```
┌─────────────────────────────────────────────────────────────┐
│  Top Bar (40px) - Workspace Toggle | Panel Controls         │
├──────────────────────────┬──────────────────────────────────┤
│                          │  ┌────────────┬────────────────┐ │
│                          │  │   Media    │  Properties    │ │
│                          │  │  Library   │   (Wide)       │ │
│   Preview (9:16)         │  │            │                │ │
│   Full Height            │  │  Toggle    │   Toggle       │ │
│   Left Panel             │  │  [Hide]    │   [Hide]       │ │
│                          │  │            │                │ │
│   ┌────────────┐         │  │  250px     │    400px       │ │
│   │            │         │  │            │                │ │
│   │   9:16     │         │  ├────────────┴────────────────┤ │
│   │   Video    │         │  │                             │ │
│   │            │         │  │   Timeline (Resizable)      │ │
│   │            │         │  │   Min: 150px, Max: 400px    │ │
│   └────────────┘         │  │                             │ │
│                          │  │   Drag handle to resize ↕   │ │
│   flex-1                 │  │                             │ │
│                          │  └─────────────────────────────┘ │
└──────────────────────────┴──────────────────────────────────┘
         ↔ Draggable              ↔ Draggable
```

### Key Features

1. **Left Preview Panel**: Full height 9:16 preview (centered, tall)
2. **Right Split Layout**:
   - **Top Half**: Media Library + Properties (side-by-side)
   - **Bottom Half**: Timeline (full width)
3. **Toggleable Panels**: Hide Media/Properties for more timeline space
4. **Draggable Dividers**: Resize all panels horizontally and vertically
5. **Adaptive Layout**: Panels collapse/expand based on priority

### Panel Specifications

#### Preview Panel (Left)
- **Width**: Flexible (min: 300px, default: flex-1)
- **Height**: Full viewport height
- **Draggable**: Right edge (horizontal resize)
- **Content**: 9:16 video centered with safe zones

#### Media Library (Right-Top-Left)
- **Width**: 250px (min: 150px, max: 400px)
- **Height**: Shares top half with Properties
- **Toggle**: Hide button to give space to Properties
- **Draggable**: Right edge (resize vs Properties)

#### Properties Panel (Right-Top-Right)
- **Width**: 400px (min: 250px, max: 600px)
- **Height**: Shares top half with Media
- **Toggle**: Hide button to give space to Timeline
- **Content**: Text styling, animation, filters

#### Timeline (Right-Bottom)
- **Width**: Full right panel width
- **Height**: Flexible (min: 150px, default: 200px, max: 400px)
- **Draggable**: Top edge (vertical resize vs top panels)
- **Zoom**: Higher default for short-form content

### Implementation

```svelte
<script>
  import { writable } from 'svelte/store';
  import Resizable from '$lib/components/ui/Resizable.svelte';
  
  // Panel visibility
  const showMedia = writable(true);
  const showProperties = writable(true);
  
  // Panel sizes
  const previewWidth = writable(500);
  const mediaWidth = writable(250);
  const propertiesWidth = writable(400);
  const timelineHeight = writable(200);
  
  // Calculate right panel top height
  $: rightTopHeight = `calc(100% - ${$timelineHeight}px)`;
</script>

<div class="vertical-edit-layout h-[calc(100vh-40px)] flex">
  <!-- Left: Preview Panel (Draggable) -->
  <Resizable
    direction="horizontal"
    bind:size={$previewWidth}
    minSize={300}
    maxSize={800}
    class="preview-panel"
  >
    <section class="h-full flex flex-col items-center justify-center bg-black p-4">
      <div class="aspect-[9/16] h-full max-h-full bg-gray-900 shadow-2xl relative">
        <WGPUPreview />
        <SafeZoneOverlay platform="tiktok" />
      </div>
    </section>
  </Resizable>

  <!-- Right: Split Panel (Media/Properties + Timeline) -->
  <div class="flex-1 flex flex-col">
    <!-- Top Half: Media + Properties -->
    <div class="flex" style="height: {rightTopHeight}">
      {#if $showMedia}
        <Resizable
          direction="horizontal"
          bind:size={$mediaWidth}
          minSize={150}
          maxSize={400}
          class="media-panel"
        >
          <section class="h-full bg-kenichi-panel border-l border-kenichi-border">
            <div class="panel-header flex justify-between items-center">
              <span>Media Library</span>
              <button 
                class="nle-icon-btn"
                on:click={() => showMedia.set(false)}
                title="Hide Media Library"
              >
                <i class="i-lucide-panel-left-close" />
              </button>
            </div>
            <MediaLibrary density="compact" />
          </section>
        </Resizable>
      {:else}
        <!-- Collapsed Media - Show button -->
        <button 
          class="collapsed-panel-btn"
          on:click={() => showMedia.set(true)}
          title="Show Media Library"
        >
          <i class="i-lucide-panel-left-open" />
        </button>
      {/if}

      {#if $showProperties}
        <section class="flex-1 bg-kenichi-panel border-l border-kenichi-border">
          <div class="panel-header flex justify-between items-center">
            <span>Properties</span>
            <button 
              class="nle-icon-btn"
              on:click={() => showProperties.set(false)}
              title="Hide Properties"
            >
              <i class="i-lucide-panel-right-close" />
            </button>
          </div>
          <div class="p-4 overflow-y-auto">
            <Accordion.Root>
              <TextStyling />
              <AnimationPresets />
              <Filters />
            </Accordion.Root>
          </div>
        </section>
      {:else}
        <!-- Collapsed Properties - Show button -->
        <button 
          class="collapsed-panel-btn"
          on:click={() => showProperties.set(true)}
          title="Show Properties"
        >
          <i class="i-lucide-panel-right-open" />
        </button>
      {/if}
    </div>

    <!-- Bottom Half: Timeline (Draggable) -->
    <Resizable
      direction="vertical"
      bind:size={$timelineHeight}
      minSize={150}
      maxSize={400}
      class="timeline-panel"
    >
      <section class="h-full bg-kenichi-panel border-t border-kenichi-border">
        <div class="panel-header">Timeline</div>
        <Timeline mode="portrait" defaultZoom={200} />
      </section>
    </Resizable>
  </div>
</div>

<style>
  .collapsed-panel-btn {
    width: 32px;
    background: var(--kenichi-panel);
    border-left: 1px solid var(--kenichi-border);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background 0.2s;
  }
  
  .collapsed-panel-btn:hover {
    background: var(--kenichi-surface);
  }
  
  .panel-header {
    padding: 8px 12px;
    background: var(--kenichi-surface);
    border-bottom: 1px solid var(--kenichi-border);
    font-size: 11px;
    font-weight: bold;
    text-transform: uppercase;
  }
</style>
```

### Draggable Divider Component

```svelte
<!-- src/lib/components/ui/Resizable.svelte -->
<script lang="ts">
  export let direction: 'horizontal' | 'vertical' = 'horizontal';
  export let size: number = 300;
  export let minSize: number = 100;
  export let maxSize: number = 800;
  
  let isDragging = false;
  let startPos = 0;
  let startSize = 0;
  
  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    startPos = direction === 'horizontal' ? e.clientX : e.clientY;
    startSize = size;
    
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
    document.body.style.cursor = direction === 'horizontal' ? 'ew-resize' : 'ns-resize';
  }
  
  function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;
    
    const currentPos = direction === 'horizontal' ? e.clientX : e.clientY;
    const delta = currentPos - startPos;
    const newSize = Math.max(minSize, Math.min(maxSize, startSize + delta));
    
    size = newSize;
  }
  
  function handleMouseUp() {
    isDragging = false;
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);
    document.body.style.cursor = 'default';
  }
</script>

<div class="resizable-container {direction}" style="{direction === 'horizontal' ? 'width' : 'height'}: {size}px">
  <slot />
  
  <div 
    class="resize-handle {direction}"
    on:mousedown={handleMouseDown}
  />
</div>

<style>
  .resizable-container {
    position: relative;
    flex-shrink: 0;
  }
  
  .resize-handle {
    position: absolute;
    background: transparent;
    z-index: 10;
  }
  
  .resize-handle.horizontal {
    right: 0;
    top: 0;
    bottom: 0;
    width: 4px;
    cursor: ew-resize;
  }
  
  .resize-handle.vertical {
    left: 0;
    right: 0;
    top: 0;
    height: 4px;
    cursor: ns-resize;
  }
  
  .resize-handle:hover {
    background: var(--brand-accent);
  }
  
  .resize-handle.horizontal:hover {
    box-shadow: 0 0 8px rgba(0, 196, 204, 0.5);
  }
  
  .resize-handle.vertical:hover {
    box-shadow: 0 0 8px rgba(0, 196, 204, 0.5);
  }
</style>
```

### Panel Toggle Shortcuts

```typescript
// src/lib/stores/panels.ts
import { writable } from 'svelte/store';

export const panelVisibility = writable({
  media: true,
  properties: true,
  timeline: true
});

// Keyboard shortcuts
export function registerPanelShortcuts() {
  window.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === '1') {
      panelVisibility.update(p => ({ ...p, media: !p.media }));
    }
    if (e.ctrlKey && e.key === '2') {
      panelVisibility.update(p => ({ ...p, properties: !p.properties }));
    }
    if (e.ctrlKey && e.key === '3') {
      panelVisibility.update(p => ({ ...p, timeline: !p.timeline }));
    }
  });
}
```

### Use Cases

**Scenario 1: Text-Heavy Editing**
- Hide Media Library (more space for Properties)
- Expand Properties panel (wider text controls)
- Minimize Timeline (short clips)

**Scenario 2: Timeline-Focused**
- Hide both Media + Properties
- Maximize Timeline height (400px)
- Full right panel for timeline editing

**Scenario 3: Balanced Workflow**
- All panels visible
- Default sizes (250px Media, 400px Properties, 200px Timeline)
- Quick toggle as needed

---

## 3. Code/Scripting Mode

**Purpose**: Programmatic animation (Manim/Motion Canvas style)  
**Reference**: `guide/The Workspace Toggle (Hybrid Approach).md`

### Layout Structure

```
┌─────────────────────────────────────────────────────────────┐
│  Top Bar (40px) - Workspace: CODE MODE                      │
├───────────────────────────────┬─────────────────────────────┤
│                               │                             │
│   Code Editor                 │   Preview (WGPU)            │
│   (Monaco/CodeMirror)         │   Live Render               │
│                               │                             │
│   50% width                   │   50% width                 │
│                               │                             │
│   - Syntax highlighting       │   - Real-time updates       │
│   - Autocomplete              │   - Math animations         │
│   - Error markers             │   - Procedural graphics     │
│                               │                             │
├───────────────────────────────┴─────────────────────────────┤
│  Console / Output (100px)                                    │
│  Debugging and error messages                                │
└──────────────────────────────────────────────────────────────┘
```

### Implementation

```svelte
{#if workspace === 'code'}
  <div class="flex flex-1 overflow-hidden">
    <!-- Code Editor (Left 50%) -->
    <div class="w-1/2 border-r border-kenichi-border">
      <CodeEditor 
        language="javascript"
        theme="kenichi-dark"
        on:change={handleCodeChange}
      />
    </div>

    <!-- Preview (Right 50%) -->
    <div class="w-1/2 flex items-center justify-center bg-black">
      <WGPUPreview source="code" />
    </div>
  </div>

  <!-- Console (Bottom) -->
  <footer class="h-100px bg-kenichi-panel border-t border-kenichi-border">
    <Console logs={consoleLogs} />
  </footer>
{/if}
```

---

## Alternative Layout: Right-Stack

**Purpose**: IDE-style workflow (code-heavy)  
**Reference**: `guide/The Right-Stack Layout Analysis.md`

### Layout Structure

```
┌─────────────────────────────────────────────────────────────┐
│  Top Bar (40px)                                              │
├─────────────────────────────────┬───────────────────────────┤
│                                 │  Inspector (Top)          │
│                                 │  Height: 33%              │
│   Code Editor / Asset Grid      ├───────────────────────────┤
│   (Main Workspace)              │  Preview (Middle)         │
│                                 │  Height: 33%              │
│   70% width                     │  9:16 optimized           │
│                                 ├───────────────────────────┤
│                                 │  Timeline (Bottom)        │
│                                 │  Height: 33%              │
│                                 │  Narrow, scrollable       │
└─────────────────────────────────┴───────────────────────────┘
```

**Pros**:
- Excellent for 9:16 content (vertical preview fits perfectly)
- Coding efficiency (large left pane)
- Tight feedback loop (preview above timeline)

**Cons**:
- Timeline is narrow (needs horizontal scroll or mini-map)
- 16:9 videos appear small in right column

---

## Responsive Breakpoints

### Desktop (1920x1080+)
- Use Standard Edit Mode
- All panels visible
- Timeline: 300-400px height

### Laptop (1366x768)
- Reduce panel widths (Media: 200px, Inspector: 300px)
- Timeline: 200px height
- Enable panel collapse

### Tablet (1024x768)
- Single-panel mode (tabs to switch)
- Fullscreen preview option
- Simplified timeline

---

## Panel Resizing

All panels support **draggable resizing** with these constraints:

### Media Library
- Min width: 150px
- Max width: 400px
- Collapsible to icon-only mode

### Inspector
- Min width: 250px
- Max width: 500px
- Collapsible

### Timeline
- Min height: 100px
- Max height: 600px
- Collapsible to mini-timeline (50px)

### Implementation

```svelte
<script>
  import { Resizable } from '$lib/components/ui/Resizable.svelte';
</script>

<Resizable
  direction="horizontal"
  minSize={250}
  maxSize={500}
  defaultSize={350}
>
  <Inspector />
</Resizable>
```

---

## Workspace Switching Logic

**Reference**: `guide/The Workspace Toggle (Hybrid Approach).md`

### State Management

```typescript
// src/lib/stores/workspace.ts
import { writable } from 'svelte/store';

export type WorkspaceMode = 'edit' | 'vertical' | 'code';
export const workspace = writable<WorkspaceMode>('edit');

// Persist to localStorage
workspace.subscribe(value => {
  localStorage.setItem('kenichi-workspace', value);
});
```

### Layout Switching

**DO NOT** use routing or conditional rendering (`{#if}`) - this destroys state.

**DO** use CSS visibility to keep components "warm":

```svelte
<main class="flex-1 flex overflow-hidden">
  <!-- Edit Mode -->
  <div class={$workspace === 'edit' ? 'contents' : 'hidden'}>
    <MediaLibrary />
    <Preview />
    <Inspector />
    <Timeline />
  </div>

  <!-- Vertical Mode -->
  <div class={$workspace === 'vertical' ? 'contents' : 'hidden'}>
    <MediaLibrary />
    <PreviewVertical />
    <InspectorWide />
    <MiniTimeline />
  </div>

  <!-- Code Mode -->
  <div class={$workspace === 'code' ? 'flex flex-1' : 'hidden'}>
    <CodeEditor />
    <PreviewWGPU />
    <Console />
  </div>
</main>
```

**Why**: Keeps WGPU instance and video assets loaded in memory. No re-initialization.

---

## UX Principles

### 1. Mobile-First DNA
- Prioritize visibility and thumb-reach (even on desktop)
- Large touch targets (min 40x40px)
- Clear visual hierarchy

### 2. Magnetic Timeline
- Default ON (prevents gaps)
- Toggleable with magnet icon
- Visual feedback when snapping

### 3. Contextual Surfacing
- Show relevant tools based on selection
- Hide complexity until needed
- Smart defaults

### 4. Safe Zones
- YouTube Shorts: 15% top, 35% bottom, 18% right
- TikTok: 12% top, 25% bottom
- Instagram: Similar to TikTok

### 5. Keyboard-First
- Every action has a shortcut
- Tooltips show shortcuts
- Customizable keybindings

---

## Performance Considerations

### Low-End PC Optimizations

1. **Panel Visibility**: Use `display: none` instead of unmounting
2. **Lazy Loading**: Timeline thumbnails load on-demand
3. **Debounced Resize**: Update WGPU viewport only on resize end
4. **CSS Transforms**: Use GPU-accelerated transitions

### Memory Management

- **WGPU Instance**: Shared across all workspaces
- **Video Decoders**: Kept "warm" when switching modes
- **Texture Pooling**: Reuse GPU memory

---

## Implementation Checklist

### Phase 5c (Timeline UI - Static)
- [ ] Implement Standard Edit Mode layout
- [ ] Add panel structure (Media, Preview, Inspector, Timeline)
- [ ] Create resizable panel components
- [ ] Add workspace toggle in top bar

### Phase 5d (Timeline UI - Interactive)
- [ ] Implement Vertical Edit Mode
- [ ] Add workspace switching logic
- [ ] Create safe zone overlays
- [ ] Add panel resize functionality

### Phase 7+ (Code Mode)
- [ ] Integrate Monaco/CodeMirror
- [ ] Implement Code/Scripting Mode layout
- [ ] Add console component
- [ ] Create Right-Stack alternative layout

---

## References

- `guide/The Design Layout - Multi-Device Cohesion.md`
- `guide/The Vertical Center Layout.md`
- `guide/The Workspace Toggle (Hybrid Approach).md`
- `guide/The Right-Stack Layout Analysis.md`
- `guide/CapCut Desktop.md`

---

**Last Updated**: January 2026  
**Status**: Specification Complete  
**Next**: Implement in Phase 5c
