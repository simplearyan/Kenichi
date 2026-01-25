# Development Guide

## Prerequisites

### Required Software
- **Node.js**: v18+ (LTS recommended)
- **pnpm**: v9+ (package manager)
- **Rust**: Latest stable (1.75+)
- **FFmpeg**: 6.0+ (for development testing)

### Platform-Specific Requirements

#### Windows
- Visual Studio Build Tools 2019+
- Windows 10/11

#### macOS
- Xcode Command Line Tools
- macOS 11+

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

---

## Quick Start

### 1. Clone Repository
```bash
git clone https://github.com/simplearyan/kenichi.git
cd kenichi
```

### 2. Install Dependencies
```bash
# Install pnpm globally (if not installed)
npm install -g pnpm

# Install project dependencies
pnpm install
```

### 3. Setup FFmpeg (Development)

The app bundles FFmpeg automatically in release builds, but for development you need FFmpeg in your PATH.

#### Windows
1. Download FFmpeg from [gyan.dev](https://www.gyan.dev/ffmpeg/builds/)
2. Extract to `C:\ffmpeg`
3. Add `C:\ffmpeg\bin` to PATH
4. Verify: `ffmpeg -version`

#### macOS
```bash
brew install ffmpeg
```

#### Linux
```bash
sudo apt-get install ffmpeg
```

### 4. Run Development Server
```bash
pnpm tauri dev
```

The app will open automatically. Hot-reload is enabled for frontend changes.

---

## Project Structure

```
kenichi/
├── src/                    # Frontend (Svelte 5)
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── stores/         # State management
│   │   ├── actions/        # Svelte actions
│   │   └── utils/          # Utilities
│   └── routes/             # SvelteKit routes
├── src-tauri/              # Backend (Rust)
│   ├── src/
│   │   ├── commands/       # Tauri commands
│   │   ├── engine/         # Core engine
│   │   └── lib.rs          # Entry point
│   └── Cargo.toml          # Rust dependencies
├── guide/                  # Research documents
└── scripts/                # Build scripts
```

---

## Development Workflow

### Frontend Development
```bash
# Type checking
pnpm run check

# Watch mode
pnpm run check:watch
```

### Backend Development
```bash
# Lint Rust code
cargo clippy

# Format code
cargo fmt

# Run tests
cargo test
```

### Building for Production
```bash
pnpm tauri build
```

Outputs:
- **Windows**: `src-tauri/target/release/kenichi.exe`
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

---

## Common Tasks

### Adding a New Tauri Command
1. Create function in `src-tauri/src/commands/`
2. Add to `invoke_handler!` in `lib.rs`
3. Call from frontend: `invoke('command_name')`

### Adding a New Svelte Component
1. Create in `src/lib/components/`
2. Import in parent component
3. Use UnoCSS shortcuts for styling

### Debugging

#### Frontend
- Open DevTools: `Ctrl+Shift+I` (Windows/Linux) or `Cmd+Option+I` (macOS)
- Console logs appear in DevTools

#### Backend
- Rust logs appear in terminal
- Use `println!` or `eprintln!` (will migrate to `tracing` crate)

---

## Troubleshooting

### "FFmpeg not found"
**Solution**: Ensure FFmpeg is in PATH. Run `ffmpeg -version` to verify.

### "WGPU initialization failed"
**Solution**: Update GPU drivers. WGPU requires Vulkan/Metal/DX12.

### "pnpm install fails"
**Solution**: Clear cache: `pnpm store prune` then retry.

### "Cargo build fails on Windows"
**Solution**: Install Visual Studio Build Tools with C++ workload.

---

## IDE Setup (VSCode)

### Recommended Extensions
- **Svelte for VS Code** (`svelte.svelte-vscode`)
- **rust-analyzer** (`rust-lang.rust-analyzer`)
- **Tauri** (`tauri-apps.tauri-vscode`)
- **UnoCSS** (`antfu.unocss`)

### Settings
```json
{
  "editor.formatOnSave": true,
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

---

## Performance Testing

### Target Hardware
- **GPU**: NVIDIA GT 740 (low-end)
- **RAM**: 4GB minimum
- **CPU**: Dual-core 2.0GHz+

### Benchmarking
```bash
# Profile Rust code
cargo install flamegraph
cargo flamegraph --bin kenichi
```

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/simplearyan/kenichi/issues)
- **Discussions**: [GitHub Discussions](https://github.com/simplearyan/kenichi/discussions)
- **Roadmap**: See [ROADMAP.md](./ROADMAP.md)
