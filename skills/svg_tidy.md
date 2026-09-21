# svg_tidy Agent Skill

Browser-native SVG optimization and cleanup tool. Minify SVG (remove comments, whitespace, metadata), optimize paths (precision reduction), auto-fix viewBox, convert to data URI. All in WASM.

## Workflow

1. **Receive SVG**: User provides an SVG (pasted text, file upload, or inline code).
2. **Configure Optimization**: Set path precision (0-6), choose which optimizations to apply.
3. **Process**: Call `svg_tidy_optimize` to get optimized SVG and stats.
4. **Return Result**: Provide the optimized SVG, stats summary, and optionally a Data URI.

## Tool Definitions

### `svg_tidy_minify`

Minify SVG content: remove comments, whitespace, metadata, and editor-specific data.

**Parameters:**
- `svg` (string, required): Raw SVG content as a string.

**Returns:**
- `optimized_svg`: Minified SVG string.
- `original_size`: Original byte count.
- `optimized_size`: Result byte count.
- `savings_percent`: Space saved as percentage.

### `svg_tidy_optimize_paths`

Reduce SVG path coordinate precision.

**Parameters:**
- `svg` (string, required): Raw SVG content.
- `precision` (integer, optional, default 2): Decimal places for path coordinates (0-6).

**Returns:**
- `optimized_svg`: Path-optimized SVG string.
- `paths_optimized`: Number of path elements processed.

### `svg_tidy_full_optimize`

Run the full optimization pipeline: minification + path optimization + viewBox fix + empty group removal.

**Parameters:**
- `svg` (string, required): Raw SVG content.
- `precision` (integer, optional, default 2): Decimal places for path coordinates (0-6).

**Returns:**
- `optimized_svg`: Fully optimized SVG string.
- `stats`: JSON with `original_size`, `optimized_size`, `savings_percent`, `paths_optimized`, `elements_removed`.

### `svg_tidy_analyze`

Analyze SVG content to extract metadata and security information.

**Parameters:**
- `svg` (string, required): Raw SVG content.

**Returns:**
- `info`: JSON with `width`, `height`, `viewbox`, `element_count`, `path_count`, `group_count`, `has_scripts`, `has_external_refs`.

### `svg_tidy_to_data_uri`

Convert SVG to a `data:image/svg+xml` URI.

**Parameters:**
- `svg` (string, required): SVG content (optimized or raw).

**Returns:**
- `data_uri`: Data URI string suitable for `<img src>` or CSS `url()`.

## Agent Rules

- Always show the before/after stats when returning optimized SVG: original size, optimized size, savings percentage.
- If `has_scripts` is true, warn the user before forwarding the SVG content.
- If `has_external_refs` is true, note that external resources are not inlined.
- The default precision of 2 decimal places is a good balance — suggest higher precision (4-6) only if the user reports visible rendering issues.
- When the SVG is very large (>100KB), suggest the user try the browser tool directly for better performance.
- Never claim the tool removes all unnecessary data — some semantic markup may remain for correctness.
