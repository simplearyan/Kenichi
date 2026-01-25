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
│  Top Bar (40px)                                              │
├──────────┬───────────────────────────┬──────────────────────┤
│          │                           │                      │
│  Media   │    Preview (9:16)         │   Properties         │
│  Library │    Centered, Tall         │   (Wide Panel)       │
│          │                           │                      │
│  250px   │      flex-1               │      400px           │
│          │                           │                      │
│          │   ┌─────────────┐         │                      │
│          │   │             │         │   - Text Styling     │
│          │   │   9:16      │         │   - Animation        │
│          │   │   Video     │         │   - Filters          │
│          │   │             │         │                      │
│          │   │             │         │                      │
│          │   └─────────────┘         │                      │
│          │                           │                      │
├──────────┴───────────────────────────┴──────────────────────┤
│  Timeline (Short - 150px)                                    │
│  Mini-timeline for short-form content                        │
└──────────────────────────────────────────────────────────────┘
```

### Key Differences from Standard Mode

1. **Preview Aspect Ratio**: 9:16 (vertical)
2. **Properties Panel**: Wider (400px) - more room for text controls
3. **Timeline Height**: Shorter (150px) - videos are typically 15-60s
4. **Default Zoom**: Higher zoom level for short content
5. **Safe Zone**: TikTok/Instagram UI overlays

### Implementation

```svelte
<div class="grid grid-cols-[250px_1fr_400px] h-[calc(100vh-40px)]">
  <!-- Media Library -->
  <section class="bg-kenichi-panel border-r border-kenichi-border">
    <MediaLibrary density="compact" />
  </section>

  <!-- Preview (Centered) -->
  <section class="flex flex-col items-center justify-center bg-black p-4">
    <div class="aspect-[9/16] h-full max-h-full bg-gray-900 shadow-2xl relative">
      <WGPUPreview />
      <SafeZoneOverlay platform="tiktok" />
    </div>
  </section>

  <!-- Properties (Wide) -->
  <section class="bg-kenichi-panel border-l border-kenichi-border p-4">
    <Accordion.Root>
      <TextStyling />
      <AnimationPresets />
      <Filters />
    </Accordion.Root>
  </section>
</div>

<!-- Mini Timeline (Fixed Bottom) -->
<footer class="fixed bottom-0 left-0 right-0 h-150px bg-kenichi-panel/90 backdrop-blur-md border-t border-kenichi-border">
  <Timeline mode="portrait" defaultZoom={200} />
</footer>
```

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
