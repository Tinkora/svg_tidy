# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x (current) | ✅ |

## Reporting a Vulnerability

If you discover a security vulnerability, please **do not** open a public issue.

Instead, email the project maintainer directly. You should receive a response
within 48 hours. We will work with you to understand the scope and coordinate
a fix and disclosure timeline.

### Scope

The following areas are within scope:

- SVG input validation bypasses (script injection via SVG content)
- WASM sandbox escapes
- Potential XSS vectors in the HTML editor UI
- Data URI encoding edge cases

### Out of Scope

- Issues already documented as known limitations
- Theoretical attacks requiring physical access
- Issues in dependencies (please report upstream)

## Security Model

The svg_tidy project follows these security principles:

1. **Browser-local by default**: All SVG processing happens in-browser via WASM. No SVG content ever touches a server.

2. **No script execution from SVG**: The tool strips `<script>` elements and event handler attributes by default. The `has_scripts` flag in `SvgInfo` lets users audit before processing.

3. **CSP-ready**: The static editor renders SVG previews via `<img>` with data URIs, avoiding inline SVG injection surfaces.

4. **No external resource loading**: External references (`xlink:href`, `url()`) are detected but not resolved; `has_external_refs` flag warns users.
