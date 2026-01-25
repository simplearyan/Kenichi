# Changelog

All notable changes to Kenichi will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Frame pacing with wall-clock synchronization
- FPS extraction from video streams
- Render loop delta time tracking
- Technical debt tracking in ROADMAP.md
- Comprehensive documentation (DEVELOPMENT.md, ARCHITECTURE.md)

### Changed
- Updated PlaybackState to include `last_frame_time` for frame pacing
- Improved seek accuracy with keyframe + roll-forward strategy

### Fixed
- Seek no longer jumps to 0 seconds
- Frame pacing prevents "fast-forward" playback

## [0.1.0] - 2026-01-25

### Added
- **WGPU Rendering Pipeline**
  - GPU-accelerated video display
  - Custom shader (`video.wgsl`) for full-screen rendering
  - Texture upload and bind group management

- **FFmpeg Video Decoding**
  - VideoDecoder wrapper for FFmpeg
  - Frame-accurate seeking (keyframe + roll-forward)
  - RGBA frame conversion
  - FPS and time base extraction

- **Proxy Methodology**
  - Automatic proxy generation for 4K+ videos
  - Background FFmpeg processes
  - Proxy file caching

- **Playback Controls**
  - Play/Pause functionality
  - Timeline scrubbing
  - Playback state synchronization (Backend ↔ Frontend)
  - 60 FPS render loop

- **UI Components**
  - Trinity Layout (Media | Preview | Inspector)
  - Timeline panel with playhead
  - Video viewport with WGPU canvas
  - HUD overlay with playback controls
  - Safe zone overlays (aspect ratio guides)
  - Export modal
  - Keybindings modal

- **State Management**
  - Svelte stores for playback, timeline, performance
  - Backend PlaybackState struct
  - Engine sync store for Backend ↔ Frontend communication

- **Developer Experience**
  - UnoCSS configuration with custom theme
  - Svelte 5 with Runes
  - TypeScript support
  - Cross-platform CI/CD (GitHub Actions)
  - FFmpeg bundling for Windows, macOS, Linux

### Technical Details
- **Frontend**: Svelte 5 + SvelteKit + UnoCSS
- **Backend**: Rust + Tauri 2 + WGPU 28.0 + FFmpeg 6.0
- **Build**: pnpm + Vite 6 + Cargo

### Known Issues
- Timeline store is minimal (stub implementation)
- No unit tests or integration tests
- No CI checks for PRs (only release workflow)
- Fixed delta time in render loop (should use actual delta)

---

## Release Notes

### v0.1.0 - Initial Release

This is the first public release of Kenichi, a desktop video editor built for performance on low-end hardware.

**Highlights**:
- ✅ Hardware-accelerated video playback (WGPU)
- ✅ Frame-accurate seeking
- ✅ Proxy generation for smooth 4K editing
- ✅ Cross-platform (Windows, macOS, Linux)

**What's Next**:
- Phase 4.5: Audio playback and synchronization
- Phase 5: Multi-track timeline composition
- Phase 6: Editing tools (razor, trim, ripple delete)

See [ROADMAP.md](./ROADMAP.md) for the full development plan.

---

**Note**: This project is in active development. Breaking changes may occur between minor versions until v1.0.0.
