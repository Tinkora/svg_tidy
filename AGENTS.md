# Repository Guide for AI Agents

## Project Overview

svg_tidy is a browser-native SVG optimization and cleanup tool powered by Rust/WASM. Minify SVG (remove comments, whitespace, metadata), optimize paths (precision reduction), auto-fix viewBox, extract/embed styles, convert to data URI. All processing happens in the browser — zero uploads, zero server.

## Architecture

```
svg_tidy/
├── crates/
│   ├── svg_tidy_core/        # SVG minification, path optimization, info extraction
│   └── svg_tidy_web/         # WASM bridge + HTML editor UI
├── docs/                      # Product specification
├── skills/                    # Agent Skill definitions (MCP tools)
└── index.html                 # Product landing page
```

## Key Files for AI Context

| File | Purpose |
|------|---------|
| `crates/svg_tidy_core/src/optimize.rs` | Core minification and optimization logic |
| `crates/svg_tidy_core/src/info.rs` | SVG metadata analysis |
| `crates/svg_tidy_core/src/error.rs` | Error types and error codes |
| `crates/svg_tidy_core/src/wasm.rs` | WASM bindings exposed to JS |
| `crates/svg_tidy_web/src/lib.rs` | Web crate re-exporting WASM functions |
| `crates/svg_tidy_web/static/index.html` | Full-featured SVG optimization UI |
| `skills/svg_tidy.md` | Agent usage workflow |
| `skills/mcp-tools.json` | MCP tool definitions |

## Build & Test Commands

```bash
# Run all tests
cargo test --workspace

# Format check
cargo fmt --all -- --check

# Lint (strict)
cargo clippy --workspace --all-targets -- -D warnings

# WASM compilation check
cargo check -p svg_tidy_web --target wasm32-unknown-unknown

# Build Web WASM for deployment
wasm-pack build --target web crates/svg_tidy_web
```

## Design Principles

1. **Browser-first**: All SVG processing happens in-browser via WASM. No SVG content ever leaves the user's machine.
2. **Regex-based parsing**: SVG optimization uses smart regex/string manipulation rather than a full XML parser, keeping the WASM binary small and fast.
3. **Composable operations**: Each optimization step is an independent function that can be called individually or composed via `full_optimize`.
4. **Precision control**: Path coordinate precision is user-configurable (0-6 decimal places), balancing size vs. fidelity.
5. **Lossless by default**: Operations like comment removal and metadata stripping are semantic no-ops for rendering; path rounding is the only lossy operation and is opt-in.

## Operation Pipeline (full_optimize)

```
Input SVG
  → remove_comments
  → remove_metadata
  → remove_editor_data
  → collapse_whitespace
  → remove_empty_groups
  → optimize_paths (with precision)
  → fix_viewbox
  → normalize
  → compute stats
  → Output (optimized SVG, SvgStats)
```

## Error Codes (Stable Machine-Readable)

| Code | Meaning |
|------|---------|
| `EMPTY_INPUT` | Input SVG string is empty |
| `PARSE_ERROR` | SVG content could not be processed |
| `INVALID_PRECISION` | Precision value outside 0..6 range |
| `DATA_URI_ENCODE` | Failed to encode SVG as data URI |
| `INTERNAL_ERROR` | Unexpected internal processing error |

## SVG Processing Notes

- Comments are matched with `<!-- -->` regex (not nested — SVG comments don't nest in practice)
- Metadata elements: `<metadata>`, `<rdf:RDF>`, `<cc:Work>`, `<dc:format>`, etc.
- Editor data: `sodipodi:*`, `inkscape:*`, `xmlns:sodipodi`, `xmlns:inkscape` attributes
- Path precision: round all numeric values in `d="..."` attributes to N decimal places
- Empty groups: `<g>` elements with no children (recursive removal)
- viewBox fix: if viewBox is missing but width/height exist, synthesize `viewBox="0 0 width height"`

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths, including console, keyboard, accessibility, and overflow checks.
