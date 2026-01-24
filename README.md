<div align="center">

  # ⚔️ Kenichi Video Editor
  **The High-Performance Editor for Every Machine.**

  [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE)
  [![Tauri](https://img.shields.io/badge/Built_with-Tauri-orange?logo=tauri)](https://tauri.app)
  [![Rust](https://img.shields.io/badge/Backend-Rust-black?logo=rust)](https://www.rust-lang.org)
  [![Svelte](https://img.shields.io/badge/Frontend-Svelte_5-red?logo=svelte)](https://svelte.dev)
  [![Sponsor](https://img.shields.io/badge/♥-Sponsor_Us-pink?logo=githubsponsors)](https://github.com/sponsors/YOUR_USERNAME)

  <br />
  
  <img src="docs/screenshot_hero.png" alt="Kenichi Interface" width="800" style="border-radius: 10px; box-shadow: 0 10px 30px rgba(0,0,0,0.5);">

  <br />
  <br />

  [**Download Alpha**](https://github.com/YOUR_USERNAME/kenichi/releases) • [**Discord Community**](https://discord.gg/YOUR_INVITE) • [**Roadmap**](https://github.com/users/YOUR_USERNAME/projects/1)

</div>

---

## 🎬 Kenichi (剣一)

**The high-performance, open-source video editing suite for everyone.**

Built with **Rust**, **Tauri**, and **Svelte**, Kenichi is designed to be a fast, modern, and ethical alternative to bloated professional NLEs. Powered by the custom **Kinetix** engine, it brings 4K 60fps editing to the masses—even on low-end hardware like the GT 740.



---

## 🚀 Key Features

* **⚡ Kinetix Engine:** A low-level WGPU-based rendering engine written in Rust for maximum hardware efficiency.
* **🏗️ Trinity Layout:** A high-density, professional workspace featuring Media, Preview, Inspector, and Timeline zones.
* **🧲 Magnetic Timeline:** Non-destructive editing with ripple-delete and smart-snapping logic.
* **📱 Content Creator Toolkit:** Built-in Safe Zone overlays for YouTube Shorts, TikTok, and Instagram Reels.
* **🛠️ Performance First:** Intelligent IPC throttling and "Stress Test" diagnostics to ensure stability on older PCs.
* **📦 Open Architecture:** Built on open standards like FFmpeg and WGPU.

---

## 🛠️ Tech Stack

* **Core Engine:** Rust + WGPU (The Kinetix Engine)
* **Desktop Wrapper:** Tauri v2
* **Frontend UI:** Svelte 5 + TypeScript
* **Styling:** UnoCSS (High-density design system)
* **Components:** Bits UI (Accessible primitive components)
* **Processing:** FFmpeg (Sidecar integration)



---

## 📦 Getting Started

### Prerequisites
* [Rust](https://www.rust-lang.org/tools/install) (MSRV 1.75+)
* [Node.js](https://nodejs.org/) (v20+)
* [pnpm](https://pnpm.io/installation)
* FFmpeg binaries in `src-tauri/bin` (named correctly for your target triple)

### Installation
1. Clone the repository:
   ```bash
   git clone [https://github.com/simplearyan/kenichi.git](https://github.com/simplearyan/kenichi.git)
   cd kenichi