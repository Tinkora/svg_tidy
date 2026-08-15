# svg_tidy

[![CI](https://github.com/Tinkora/svg_tidy/actions/workflows/test.yml/badge.svg)](https://github.com/Tinkora/svg_tidy/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

A browser-native SVG optimization and cleanup tool — minify, optimize paths, fix viewBox, and convert to data URI. All powered by Rust/WASM, all in your browser.

## ✨ Features

- 🧹 **Smart Minification** — Strip comments, whitespace, metadata, and editor-specific cruft
- 📐 **Path Optimization** — Reduce coordinate precision with configurable decimal places (0-6)
- 🖼️ **Auto-Fix viewBox** — Detect missing/broken viewBox and repair from width/height
- 📦 **Data URI Export** — One-click conversion to `data:image/svg+xml` URI
- 🔍 **SVG Analysis** — Extract dimensions, element counts, and security info (scripts, external refs)
- 🔒 **100% Browser-Local** — Your SVG never leaves your machine; all processing in WASM
- 📊 **Stats & Savings** — See exactly how much you saved (bytes, percentages, elements removed)

## 🚀 Quick Start

```bash
# Clone
git clone https://github.com/Tinkora/svg_tidy.git
cd svg_tidy

# Build Web WASM
wasm-pack build --target web crates/svg_tidy_web

# Launch
cp crates/svg_tidy_web/pkg/* crates/svg_tidy_web/static/pkg/
cd crates/svg_tidy_web/static && python3 -m http.server 8080
```

Open `http://localhost:8080` in your browser.

## 📂 Project Structure

| Component | Description | Status |
|-----------|-------------|--------|
| `svg_tidy_core` | SVG minification, path optimization, info extraction | ✅ |
| `svg_tidy_web` | WASM bridge + HTML editor UI | ✅ |
| `skills/` | Agent Skill definition (MCP tools) | ✅ |

## 🔧 Development

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p svg_tidy_web --target wasm32-unknown-unknown
```

## 📄 Docs

- [Product Spec (zh-CN)](docs/product_spec.zh-CN.md)

## 🤝 Community

- [Contributing](./CONTRIBUTING.md)
- [Code of Conduct](./CODE_OF_CONDUCT.md)
- [Security](./SECURITY.md)
- [Changelog](./CHANGELOG.md)

## Support

If svg_tidy saves you time, support Tinkora on [Ko-fi](https://ko-fi.com/tinkora).
Support is optional and never affects access or issue priority.

See [SUPPORT.md](./SUPPORT.md) for questions, bug reports, and security reports.

## 📜 License

MIT © [Tinkora](https://github.com/Tinkora)
