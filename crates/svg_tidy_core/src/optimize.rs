use crate::error::CoreError;
use regex::Regex;
use serde::Serialize;

/// Statistics about SVG optimization.
#[derive(Clone, Debug, Serialize)]
pub struct SvgStats {
    /// Original SVG size in bytes.
    pub original_size: usize,
    /// Optimized SVG size in bytes.
    pub optimized_size: usize,
    /// Space saved as percentage (0.0–100.0).
    pub savings_percent: f32,
    /// Number of `<path>` elements whose coordinates were reduced.
    pub paths_optimized: usize,
    /// Number of elements removed (empty groups, metadata, etc.).
    pub elements_removed: usize,
}

// ── internal helpers ────────────────────────────────────────────────

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("invalid regex")
}

/// Round a numeric string to `precision` decimal places.
fn round_number_str(num_str: &str, precision: u32) -> String {
    if precision >= 6 {
        // Near-lossless — still normalize representation
        if let Ok(v) = num_str.parse::<f64>() {
            return format!("{v}");
        }
        return num_str.to_string();
    }
    if let Ok(v) = num_str.parse::<f64>() {
        // Check if it's an integer already
        if v.fract() == 0.0 && !num_str.contains('.') {
            return num_str.to_string();
        }
        if precision == 0 {
            return format!("{}", v.round() as i64);
        }
        return format!("{v:.precision$}");
    }
    num_str.to_string()
}

// ── public API ──────────────────────────────────────────────────────

/// Remove XML/SVG comments (`<!-- ... -->`).
pub fn remove_comments(svg: &str) -> String {
    let comment_re = re(r"<!--.*?-->");
    comment_re.replace_all(svg, "").to_string()
}

/// Remove metadata elements and their contents.
/// Targets `<metadata>`, `<rdf:RDF>`, and Dublin Core elements.
pub fn remove_metadata(svg: &str) -> String {
    let re_metadata = re(r"(?s)<metadata\b[^>]*>.*?</metadata>");
    let re_rdf = re(r"(?s)<rdf:RDF\b[^>]*>.*?</rdf:RDF>");
    let re_dc = re(r"(?s)<(dc|cc):\w+\b[^>]*>.*?</(dc|cc):\w+>");

    let result = re_metadata.replace_all(svg, "").to_string();
    let result = re_rdf.replace_all(&result, "").to_string();
    re_dc.replace_all(&result, "").to_string()
}

/// Remove editor-specific namespaces and attributes from Illustrator, Inkscape, Sketch, etc.
pub fn remove_editor_data(svg: &str) -> String {
    // Remove namespace declarations
    let re_ns_sodipodi = re(r#"\s*xmlns:sodipodi\s*=\s*"[^"]*""#);
    let re_ns_inkscape = re(r#"\s*xmlns:inkscape\s*=\s*"[^"]*""#);
    let re_ns_ai = re(r#"\s*xmlns:ai\s*=\s*"[^"]*""#);
    let re_ns_graph = re(r#"\s*xmlns:graph\s*=\s*"[^"]*""#);
    let re_ns_serif = re(r#"\s*xmlns:serif\s*=\s*"[^"]*""#);

    // Remove attributes with editor prefixes
    let re_attr_sodipodi = re(r#"\s*sodipodi:\w+\s*=\s*"[^"]*""#);
    let re_attr_inkscape = re(r#"\s*inkscape:\w+\s*=\s*"[^"]*""#);
    let re_attr_ai = re(r#"\s*ai:\w+\s*=\s*"[^"]*""#);
    let re_attr_serif = re(r#"\s*serif:\w+\s*=\s*"[^"]*""#);

    let result = re_ns_sodipodi.replace_all(svg, "").to_string();
    let result = re_ns_inkscape.replace_all(&result, "").to_string();
    let result = re_ns_ai.replace_all(&result, "").to_string();
    let result = re_ns_graph.replace_all(&result, "").to_string();
    let result = re_ns_serif.replace_all(&result, "").to_string();
    let result = re_attr_sodipodi.replace_all(&result, "").to_string();
    let result = re_attr_inkscape.replace_all(&result, "").to_string();
    let result = re_attr_ai.replace_all(&result, "").to_string();
    re_attr_serif.replace_all(&result, "").to_string()
}

/// Collapse whitespace: remove leading/trailing spaces on each line,
/// collapse multiple spaces/newlines into single space, but preserve content
/// inside `<style>`, `<text>`, and `<tspan>` elements.
pub fn collapse_whitespace(svg: &str) -> String {
    // Strategy: split on tags with content-protecting regions.
    // Simpler approach: normalize all whitespace between tags, then restore
    // protected content inside <style>, <text>, <tspan>.

    // First, protect style and text content by extracting them.
    let mut protected: Vec<String> = Vec::new();

    // Protect <style>...</style>
    let style_re = re(r"(?s)(<style\b[^>]*>)(.*?)(</style>)");
    let svg = style_re
        .replace_all(svg, |caps: &regex::Captures| {
            let idx = protected.len();
            protected.push(caps[2].to_string());
            format!("{}__SVGTIDY_PROTECTED_STYLE_{idx}__{}", &caps[1], &caps[3])
        })
        .to_string();

    // Protect <text>...</text> and <tspan>...</tspan>
    let text_re = re(r"(?s)(<(?:text|tspan)\b[^>]*>)(.*?)(</(?:text|tspan)>)");
    let svg = text_re
        .replace_all(&svg, |caps: &regex::Captures| {
            let idx = protected.len();
            protected.push(caps[2].to_string());
            format!(
                "{}__SVGTIDY_PROTECTED_TEXT_{idx}__{}",
                &caps[1], &caps[3]
            )
        })
        .to_string();

    // Now collapse whitespace: remove leading/trailing whitespace, collapse
    // multiple whitespace chars between tags/attributes into single space.
    let re_multi_space = re(r"\s{2,}");
    let svg = re_multi_space.replace_all(&svg, " ").to_string();

    // Remove whitespace between tags: `> <` → `><`
    let re_between_tags = re(r">\s+<");
    let svg = re_between_tags.replace_all(&svg, "><").to_string();

    // Trim
    let svg = svg.trim().to_string();

    // Restore protected content
    let restore_re = re(r"__SVGTIDY_PROTECTED_(STYLE|TEXT)_(\d+)__");
    let svg = restore_re
        .replace_all(&svg, |caps: &regex::Captures| {
            let idx: usize = caps[2].parse().unwrap_or(0);
            protected.get(idx).cloned().unwrap_or_default()
        })
        .to_string();

    svg
}

/// Remove empty `<g>` groups (groups with no children or only whitespace/empty children).
/// Runs recursively since removing one empty group may make its parent empty.
pub fn remove_empty_groups(svg: &str) -> Result<String, CoreError> {
    let mut result = svg.to_string();
    let empty_g_re = re(r"(?s)<g\b[^>]*>\s*</g>");

    loop {
        let new_result = empty_g_re.replace_all(&result, "").to_string();
        if new_result.len() == result.len() {
            break;
        }
        result = new_result;
    }

    Ok(result)
}

/// Reduce path coordinate precision in `d="..."` attributes.
/// Rounds all numeric values to `precision` decimal places.
pub fn optimize_paths(svg: &str, precision: u32) -> Result<String, CoreError> {
    if precision > 6 {
        return Err(CoreError::InvalidPrecision(precision));
    }

    // Match path d="..." attribute values
    let path_re = re(r#"(?s)(\bd\s*=\s*")([^"]*)(")"#);
    let result = path_re
        .replace_all(svg, |caps: &regex::Captures| {
            let prefix = &caps[1];
            let path_data = &caps[2];
            let suffix = &caps[3];

            // Split path data into tokens: commands (letters) and numbers
            let number_re = re(r"(\d+\.?\d*|\.\d+)");
            let optimized = number_re
                .replace_all(path_data, |nc: &regex::Captures| {
                    round_number_str(&nc[1], precision)
                })
                .to_string();

            format!("{prefix}{optimized}{suffix}")
        })
        .to_string();

    // Also handle CSS path() values in style attributes
    let css_path_re = re(r"(?s)(path\s*\(\s*\")([^\"]*)(\")");
    let result = css_path_re
        .replace_all(&result, |caps: &regex::Captures| {
            let prefix = &caps[1];
            let path_data = &caps[2];
            let suffix = &caps[3];
            let number_re = re(r"(\d+\.?\d*|\.\d+)");
            let optimized = number_re
                .replace_all(path_data, |nc: &regex::Captures| {
                    round_number_str(&nc[1], precision)
                })
                .to_string();
            format!("{prefix}{optimized}{suffix}")
        })
        .to_string();

    Ok(result)
}

/// Fix missing or broken viewBox. If viewBox is absent but width/height are present,
/// synthesize `viewBox="0 0 w h"`.
pub fn fix_viewbox(svg: &str) -> Result<String, CoreError> {
    let has_viewbox = re(r"\bviewBox\s*=").is_match(svg);

    if has_viewbox {
        // Already has viewBox — no change needed
        return Ok(svg.to_string());
    }

    // Try to extract width and height
    let width_re = re(r#"\bwidth\s*=\s*"(\d+(?:\.\d+)?)""#);
    let height_re = re(r#"\bheight\s*=\s*"(\d+(?:\.\d+)?)""#);

    let width: Option<f64> = width_re
        .captures(svg)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok());

    let height: Option<f64> = height_re
        .captures(svg)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok());

    match (width, height) {
        (Some(w), Some(h)) => {
            let viewbox_attr = format!(r#"viewBox="0 0 {} {}" "#, w, h);
            // Insert viewBox right after the <svg tag name, before attributes close
            let svg_tag_re = re(r"(<svg\b[^>]*?)\s*/?>");
            let result = svg_tag_re
                .replace(svg, |caps: &regex::Captures| {
                    let tag_rest = &caps[1];
                    // Check if self-closing
                    let full_match = caps.get(0).unwrap().as_str();
                    if full_match.ends_with("/>") {
                        // Was self-closing; insert viewBox and re-self-close
                        format!("{tag_rest} {viewbox_attr}/>")
                    } else {
                        format!("{tag_rest} {viewbox_attr}>")
                    }
                })
                .to_string();
            Ok(result)
        }
        _ => {
            // Can't determine dimensions — return as-is
            Ok(svg.to_string())
        }
    }
}

/// Convert SVG to a `data:image/svg+xml;base64,...` / percent-encoded URI.
pub fn svg_to_data_uri(svg: &str) -> Result<String, CoreError> {
    if svg.is_empty() {
        return Err(CoreError::EmptyInput);
    }

    // Use percent-encoding for the SVG content (more compact than base64 for text)
    // We also URL-encode # to %23 to avoid fragment issues

    let encoded: String = svg
        .chars()
        .map(|c| match c {
            // Characters safe in data URIs
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "%20".to_string(),
            '#' => "%23".to_string(),
            '"' => "%22".to_string(),
            '%' => "%25".to_string(),
            '<' => "%3C".to_string(),
            '>' => "%3E".to_string(),
            '&' => "%26".to_string(),
            '\'' => "%27".to_string(),
            other => {
                // Encode as UTF-8 bytes and then percent-encode
                let mut buf = [0u8; 4];
                let encoded_char = other.encode_utf8(&mut buf);
                encoded_char
                    .as_bytes()
                    .iter()
                    .map(|b| format!("%{b:02X}"))
                    .collect::<Vec<_>>()
                    .join("")
            }
        })
        .collect();

    Ok(format!("data:image/svg+xml,{encoded}"))
}

/// Apply all optimizations: minify + path optimize + fix viewBox + remove empty groups.
/// Returns the optimized SVG and a stats summary.
pub fn full_optimize(svg: &str, precision: u32) -> Result<(String, SvgStats), CoreError> {
    if svg.trim().is_empty() {
        return Err(CoreError::EmptyInput);
    }

    let original_size = svg.len();

    // Count elements before removal
    let before_element_count = count_elements_internal(svg);
    let before_path_count = re(r"<\s*path\b").find_iter(svg).count();

    // Pipeline
    let mut result = remove_comments(svg);
    result = remove_metadata(&result);
    result = remove_editor_data(&result);
    result = collapse_whitespace(&result);

    let before_groups = result.clone();
    result = remove_empty_groups(&result)?;
    let after_groups = &result;

    result = optimize_paths(&result, precision)?;
    result = fix_viewbox(&result)?;

    // Count paths after optimization for comparison
    let after_element_count = count_elements_internal(&result);

    let optimized_size = result.len();
    let savings_percent = if original_size > 0 {
        ((original_size - optimized_size) as f32 / original_size as f32) * 100.0
    } else {
        0.0
    };

    // Count how many groups were removed
    let groups_before = re(r"<\s*g\b").find_iter(&before_groups).count();
    let groups_after = re(r"<\s*g\b").find_iter(after_groups).count();
    let groups_removed = groups_before.saturating_sub(groups_after);

    let elements_removed = before_element_count.saturating_sub(after_element_count) + groups_removed;

    Ok((
        result,
        SvgStats {
            original_size,
            optimized_size,
            savings_percent: (savings_percent * 10.0).round() / 10.0, // Round to 1 decimal
            paths_optimized: before_path_count,
            elements_removed,
        },
    ))
}

/// Minify SVG (remove comments, metadata, editor data, collapse whitespace)
/// without touching path precision or viewBox.
pub fn minify_svg(svg: &str) -> Result<String, CoreError> {
    if svg.trim().is_empty() {
        return Err(CoreError::EmptyInput);
    }

    let mut result = remove_comments(svg);
    result = remove_metadata(&result);
    result = remove_editor_data(&result);
    result = collapse_whitespace(&result);

    Ok(result)
}

/// Count XML/SVG elements (approximate, via opening tags).
fn count_elements_internal(svg: &str) -> usize {
    let element_re = re(r"<\s*(\w+)[\s/>]");
    element_re.find_iter(svg).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_comments() {
        let svg = r#"<svg><!-- a comment --><path d="M0,0"/><!--another--></svg>"#;
        let result = remove_comments(svg);
        assert!(!result.contains("comment"));
        assert!(result.contains("<path"));
        assert!(result.contains("<svg"));
    }

    #[test]
    fn test_remove_metadata() {
        let svg = r#"<svg><metadata><rdf:RDF><cc:Work><dc:format>image/svg+xml</dc:format></cc:Work></rdf:RDF></metadata><path d="M0,0"/></svg>"#;
        let result = remove_metadata(svg);
        assert!(!result.contains("metadata"));
        assert!(!result.contains("RDF"));
        assert!(result.contains("<path"));
    }

    #[test]
    fn test_remove_editor_data() {
        let svg = r#"<svg xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" sodipodi:docname="test.svg" inkscape:version="1.0"><path d="M0,0"/></svg>"#;
        let result = remove_editor_data(svg);
        assert!(!result.contains("sodipodi"));
        assert!(!result.contains("inkscape"));
        assert!(result.contains("<path"));
    }

    #[test]
    fn test_collapse_whitespace() {
        let svg = r#"<svg   width="100"  height="100"  >
          <path   d="M0,0 L10,10"   fill="red" />
        </svg>"#;
        let result = collapse_whitespace(svg);
        assert!(!result.contains("  "));
        assert!(result.contains("<svg "));
    }

    #[test]
    fn test_collapse_whitespace_preserves_text() {
        let svg = r#"<svg><text x="10" y="20">Hello   World</text></svg>"#;
        let result = collapse_whitespace(svg);
        assert!(result.contains("Hello   World"));
    }

    #[test]
    fn test_remove_empty_groups() {
        let svg = r#"<svg><g></g><g><path d="M0,0"/></g><g>  </g></svg>"#;
        let result = remove_empty_groups(svg).unwrap();
        assert_eq!(
            result.matches("<g").count(),
            1,
            "Only the non-empty group should remain"
        );
    }

    #[test]
    fn test_optimize_paths_precision_0() {
        let svg = r#"<svg><path d="M10.52341,20.18472 L30.71452,40.13256"/></svg>"#;
        let result = optimize_paths(svg, 0).unwrap();
        assert!(result.contains("M11,20 L31,40"));
    }

    #[test]
    fn test_optimize_paths_precision_2() {
        let svg = r#"<svg><path d="M10.52341,20.18472 L30.71452,40.13256"/></svg>"#;
        let result = optimize_paths(svg, 2).unwrap();
        assert!(result.contains("M10.52,20.18"));
    }

    #[test]
    fn test_optimize_paths_invalid_precision() {
        let svg = "<svg></svg>";
        let result = optimize_paths(svg, 7);
        assert!(result.is_err());
    }

    #[test]
    fn test_fix_viewbox_adds_missing() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="200"><path d="M0,0"/></svg>"#;
        let result = fix_viewbox(svg).unwrap();
        assert!(result.contains(r#"viewBox="0 0 100 200""#));
    }

    #[test]
    fn test_fix_viewbox_keeps_existing() {
        let svg =
            r#"<svg viewBox="10 20 100 200" width="100" height="200"><path d="M0,0"/></svg>"#;
        let result = fix_viewbox(svg).unwrap();
        assert!(result.contains(r#"viewBox="10 20 100 200""#));
        // Should still only have one viewBox
        assert_eq!(result.matches("viewBox").count(), 1);
    }

    #[test]
    fn test_svg_to_data_uri() {
        let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0,0\"/></svg>";
        let result = svg_to_data_uri(svg).unwrap();
        assert!(result.starts_with("data:image/svg+xml,"));
    }

    #[test]
    fn test_svg_to_data_uri_empty() {
        let result = svg_to_data_uri("");
        assert!(result.is_err());
    }

    #[test]
    fn test_full_optimize() {
        let svg = r#"<?xml version="1.0"?>
<!-- Generator: Adobe Illustrator -->
<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
     xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
     sodipodi:docname="test.svg"
     inkscape:version="1.0"
     width="100"
     height="100">
  <metadata>
    <rdf:RDF><cc:Work><dc:format>image/svg+xml</dc:format></cc:Work></rdf:RDF>
  </metadata>
  <g></g>
  <g>
    <path d="M10.52341,20.18472 L30.71452,40.13256"/>
    <path d="M5.12345,10.56789"/>
  </g>
</svg>"#;

        let (result, stats) = full_optimize(svg, 2).unwrap();

        assert!(stats.savings_percent > 0.0);
        assert!(stats.original_size > stats.optimized_size);
        // Should NOT contain comments, metadata, editor data
        assert!(!result.contains("<!--"));
        assert!(!result.contains("metadata"));
        assert!(!result.contains("sodipodi"));
        assert!(!result.contains("inkscape"));
        // Should have viewBox added
        assert!(result.contains("viewBox"));
        // Path coordinates should be rounded
        assert!(result.contains("M10.52,20.18"));
    }

    #[test]
    fn test_full_optimize_empty_input() {
        let result = full_optimize("", 2);
        assert!(result.is_err());
        let result = full_optimize("   ", 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_minify_svg() {
        let svg = r#"<!-- banner -->
<svg width="10" height="10">
  <metadata><rdf:RDF/></metadata>
  <path d="M0,0"/>
</svg>"#;
        let result = minify_svg(svg).unwrap();
        assert!(!result.contains("<!--"));
        assert!(!result.contains("metadata"));
        assert!(result.contains("<path"));
    }

    #[test]
    fn test_round_number_str() {
        assert_eq!(round_number_str("10.52341", 2), "10.52");
        assert_eq!(round_number_str("10.5", 0), "11");
        assert_eq!(round_number_str("20", 0), "20");
        assert_eq!(round_number_str("3.14159", 6), "3.14159");
        assert_eq!(round_number_str("0.1234567", 6), "0.123457"); // rounds up
    }
}
