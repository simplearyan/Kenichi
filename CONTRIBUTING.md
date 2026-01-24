# Contributing to Kenichi (剣一)

First off, thank you for considering contributing to Kenichi! It’s people like you who will make this the most accessible and powerful open-source video editor in the world.

As a GPLv3 project, we value transparency, community ownership, and ethical development. This document outlines the standards we use to keep the **Kinetix** engine fast and the UI responsive.

---

## 🏗️ Our Philosophy

1.  **Performance is a Feature:** Kenichi must run on low-end hardware (target: GT 740). Avoid heavy dependencies and expensive CPU operations in the render loop.
2.  **Native over Web:** If a task can be done in the Rust core (decoding, mixing, effects), it belongs in the Kinetix engine, not the Svelte frontend.
3.  **Atomic Design:** Keep Svelte components small, reusable, and styled with UnoCSS shortcuts.

---

## 🛠️ Getting Started

1.  **Fork and Clone:** Fork the repository and clone it to your local machine.
2.  **Environment:** Ensure you have the latest stable Rust, Node.js (v20+), and `pnpm` installed.
3.  **Setup:** Run `pnpm install` in the root and ensure your FFmpeg sidecars are in `src-tauri/bin`.
4.  **Branching:** Create a branch for your feature or fix: `git checkout -b feat/your-feature-name`.

---

## 💻 Code Standards

### Rust (The Kinetix Engine)
* **Formatting:** Always run `cargo fmt` before committing.
* **Linting:** We use Clippy. Ensure your code passes `cargo clippy`.
* **Safety:** Avoid `unsafe` blocks unless absolutely necessary for WGPU/Native handles. Document all `unsafe` code with a `// SAFETY:` comment.
* **Async:** Use `tokio` for non-blocking I/O and sidecar management.

### Svelte & TypeScript (The UI)
* **Reactivity:** Use Svelte 5 runes (`$state`, `$derived`, `$effect`) for state management.
* **Styling:** Use UnoCSS classes. Avoid writing custom `<style>` blocks unless necessary for complex animations.
* **Types:** No `any`. Explicitly define interfaces for all component props and store objects.

---

## 📝 Commit Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

* `feat:` A new feature for the user.
* `fix:` A bug fix.
* `docs:` Documentation changes only.
* `perf:` A code change that improves performance.
* `refactor:` A code change that neither fixes a bug nor adds a feature.
* `chore:` Updating build tasks, package manager configs, etc.

**Example:** `feat(timeline): add smart-snapping to safe zone boundaries`

---

## 🧪 Testing & Performance

Before submitting a Pull Request:
1.  **Stress Test:** Run the Kenichi Performance Diagnostic (within the app) to ensure your changes don't drop the UI below 30 FPS on your hardware.
2.  **Build:** Ensure the project builds successfully with `pnpm tauri build`.
3.  **Rust Tests:** Run `cargo test` to verify engine logic.

---

## 🚀 Pull Request Process

1.  Update the `README.md` if your change introduces new functionality or configuration.
2.  Include screenshots or screen recordings if you are changing the UI.
3.  The PR will be reviewed by maintainers. We may ask for changes to ensure alignment with the GT 740 performance target.
4.  Once approved, your code will be merged into `main`.

---

## ⚖️ License

By contributing to Kenichi, you agree that your contributions will be licensed under the **GNU General Public License v3 (GPLv3)**.

---
*Thank you for helping us build the future of open-source video editing!*