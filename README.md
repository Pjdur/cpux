# Minimal GUI Framework 🖼️

A lightweight, modular GUI framework built in Rust using `pixels`, `winit`, `rusttype`, and `imageproc`. This project is designed for simplicity, extensibility, and full control over rendering and input — ideal for learning, prototyping, or building custom tools.

## ✨ Features

- 🖋️ Text rendering with `rusttype`
- 🖱️ Interactive buttons with click detection
- 🎨 Pixel-based drawing via `pixels`
- 🧱 Modular widget system (`Text`, `Button`)
- 🖼️ Real-time rendering loop with `winit`

## 📦 Dependencies

- [`pixels`](https://crates.io/crates/pixels) — GPU-backed pixel buffer
- [`winit`](https://crates.io/crates/winit) — cross-platform window and event handling
- [`rusttype`](https://crates.io/crates/rusttype) — font rendering
- [`image`](https://crates.io/crates/image) — image buffer manipulation
- [`imageproc`](https://crates.io/crates/imageproc) — drawing primitives

## 🚀 Getting Started

```bash
git clone https://github.com/your-username/gui_framework.git
cd gui_framework
cargo run
```

Make sure you have a valid font file path in `main.rs` (e.g. `C:/Windows/Fonts/arial.ttf`).

## 🧱 Project Structure

```
src/
├── app.rs          # Core app engine and event loop
├── font.rs         # Font loading utility
├── widget/         # Modular widgets
│   ├── mod.rs
│   ├── text.rs     # Text widget
│   └── button.rs   # Button widget with click detection
├── main.rs         # Demo app
└── render.rs       # (Reserved for future drawing utilities)
```

## 🧠 Planned Features

- Layout system (`Row`, `Column`, `Stack`)
- Hover effects and visual feedback
- Shared state/context for dynamic widgets
- More widgets: `Checkbox`, `Slider`, `Label`, `TextInput`
- Style system for reusable themes

## 🛠️ Development Notes

This project is in early development. Expect rapid iteration and architectural changes as features are added. Contributions are welcome once the repo is public — for now, it's a personal playground for GUI experimentation.

## 📜 License

Private project. License will be defined when made public.

---

Made with ❤️ in Rust.
