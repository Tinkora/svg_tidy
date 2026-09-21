use crate::error::CoreError;
use regex::Regex;
use serde::Serialize;

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("invalid regex")
}

/// Metadata about an SVG document.
#[derive(Clone, Debug, Serialize)]
pub struct SvgInfo {
    /// The `width` attribute value in user units, if present.
    pub width: Option<f64>,
    /// The `height` attribute value in user units, if present.
    pub height: Option<f64>,
    /// The `viewBox` attribute value as a string, if present.
    pub viewbox: Option<String>,
    /// Approximate total count of XML elements.
    pub element_count: usize,
    /// Number of `<path>` elements.
    pub path_count: usize,
    /// Number of `<g>` (group) elements.
    pub group_count: usize,
    /// Whether the SVG contains `<script>` elements.
    pub has_scripts: bool,
    /// Whether the SVG has external references (`xlink:href`, `url()`, `<use href>` external).
    pub has_external_refs: bool,
}

/// Analyze an SVG string and extract metadata.
pub fn analyze_svg(svg: &str) -> Result<SvgInfo, CoreError> {
    if svg.trim().is_empty() {
        return Err(CoreError::EmptyInput);
    }

    // Extract width
    let width_re = re(r#"\bwidth\s*=\s*"(\d+(?:\.\d+)?)""#);
    let width = width_re
        .captures(svg)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok());

    // Extract height
    let height_re = re(r#"\bheight\s*=\s*"(\d+(?:\.\d+)?)""#);
    let height = height_re
        .captures(svg)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok());

    // Extract viewBox
    let viewbox_re = re(r#"\bviewBox\s*=\s*"([^"]*)""#);
    let viewbox = viewbox_re
        .captures(svg)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    // Count elements (approximate via opening tag pattern)
    let element_re = re(r"<\s*(\w+)[\s/>]");
    let element_count = element_re.find_iter(svg).count();

    // Count path elements
    let path_re = re(r"<\s*path\b");
    let path_count = path_re.find_iter(svg).count();

    // Count group elements
    let group_re = re(r"<\s*g\b");
    let group_count = group_re.find_iter(svg).count();

    // Check for scripts
    let script_re = re(r"<\s*script\b");
    let has_scripts = script_re.is_match(svg);

    // Check for external references
    let has_external_refs = has_external_references(svg);

    Ok(SvgInfo {
        width,
        height,
        viewbox,
        element_count,
        path_count,
        group_count,
        has_scripts,
        has_external_refs,
    })
}

/// Check if the SVG contains external references that could trigger network requests.
fn has_external_references(svg: &str) -> bool {
    // xlink:href pointing to external URLs (not just fragment #ids)
    let xlink_re = re(r#"xlink:href\s*=\s*"https?://"#);
    if xlink_re.is_match(svg) {
        return true;
    }

    // href pointing to external URLs (SVG 2)
    let href_re = re(r#"\bhref\s*=\s*"https?://"#);
    if href_re.is_match(svg) {
        return true;
    }

    // url() references to external resources (in style or fill)
    let url_re = re(r#"url\(\s*"https?://"#);
    if url_re.is_match(svg) {
        return true;
    }

    // <image href="http..."> or <image xlink:href="http...">
    let image_re = re(r#"<\s*image\b[^>]*\b(?:href|xlink:href)\s*=\s*"https?://"#);
    if image_re.is_match(svg) {
        return true;
    }

    // <use href="http...">
    let use_re = re(r#"<\s*use\b[^>]*\bhref\s*=\s*"https?://"#);
    if use_re.is_match(svg) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_basic() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="200" viewBox="0 0 100 200"><path d="M0,0"/><path d="M10,10"/><g><path d="M20,20"/></g></svg>"#;
        let info = analyze_svg(svg).unwrap();
        assert_eq!(info.width, Some(100.0));
        assert_eq!(info.height, Some(200.0));
        assert_eq!(info.viewbox, Some("0 0 100 200".to_string()));
        assert_eq!(info.path_count, 3);
        assert_eq!(info.group_count, 1);
        assert!(!info.has_scripts);
        assert!(!info.has_external_refs);
    }

    #[test]
    fn test_analyze_no_dimensions() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M0,0"/></svg>"#;
        let info = analyze_svg(svg).unwrap();
        assert_eq!(info.width, None);
        assert_eq!(info.height, None);
        assert_eq!(info.viewbox, None);
    }

    #[test]
    fn test_analyze_detects_scripts() {
        let svg = r#"<svg><script>alert(1)</script><path d="M0,0"/></svg>"#;
        let info = analyze_svg(svg).unwrap();
        assert!(info.has_scripts);
    }

    #[test]
    fn test_analyze_detects_external_refs() {
        let svg =
            r#"<svg><image href="https://example.com/img.png" width="100" height="100"/></svg>"#;
        let info = analyze_svg(svg).unwrap();
        assert!(info.has_external_refs);
    }

    #[test]
    fn test_analyze_no_external_refs_local() {
        let svg = r#"<svg><use href="#icon1"/></svg>"#;
        let info = analyze_svg(svg).unwrap();
        assert!(!info.has_external_refs);
    }

    #[test]
    fn test_analyze_empty() {
        let result = analyze_svg("");
        assert!(result.is_err());
    }
}
