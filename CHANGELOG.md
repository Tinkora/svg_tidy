# Changelog

## [0.1.0] - 2026-08-07

### Added
- `svg_tidy_core`: SVG minification, path optimization, viewBox repair, info extraction
- `svg_tidy_web`: WASM bridge, HTML editor with dark theme and Chinese labels
- Agent Skill definition (`skills/`)
- Product landing page (Chinese)
- CI workflow (native test, clippy, WASM check, wasm-pack build)
- Documentation: product spec

### Features
- Remove SVG comments, metadata, and editor-specific data (`sodipodi:`, `inkscape:`)
- Collapse whitespace while preserving content in `<text>`, `<tspan>`, and `<style>` elements
- Path coordinate precision reduction (0-6 decimal places)
- Remove empty `<g>` groups with recursive cleanup
- Auto-fix missing or broken viewBox from width/height attributes
- Convert SVG to `data:image/svg+xml` URI
- Analyze SVG: dimensions, element counts, script detection, external reference detection
- Full optimization pipeline with stats (original/optimized size, savings %, paths/elements counts)
- Modern dark theme UI with split-panel (original + optimized views)
- Live SVG preview rendering
- One-click copy/download optimized SVG and data URI
