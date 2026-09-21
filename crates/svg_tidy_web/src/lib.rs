use svg_tidy_core::{
    CoreError, SvgStats, analyze_svg, fix_viewbox, full_optimize, minify_svg, optimize_paths,
    remove_empty_groups, svg_to_data_uri,
};
use wasm_bindgen::prelude::*;

/// Converts a CoreError into a JsValue error carrying a stable `code` field.
fn core_err(e: CoreError) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &e.code().into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &e.to_string().into()).ok();
    obj.into()
}

/// Serialize a Rust value to JsValue via serde_json.
fn serde_to_js<T: serde::Serialize>(value: &T) -> Result<JsValue, JsValue> {
    let json = serde_json::to_string(value)
        .map_err(|e| JsValue::from_str(&format!("Serialization failed: {e}")))?;
    js_sys::JSON::parse(&json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse failed: {e:?}")))
}

/// Helper: build a { svg, stats } JS object from an optimization result.
fn build_optimize_result(optimized: &str, stats: &SvgStats) -> Result<JsValue, JsValue> {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"svg".into(), &JsValue::from_str(optimized)).ok();
    js_sys::Reflect::set(&obj, &"stats".into(), &serde_to_js(stats)?).ok();
    Ok(obj.into())
}

// ── WASM exports ────────────────────────────────────────────────────

/// Minify SVG: remove comments, whitespace, metadata, and editor data.
#[wasm_bindgen]
pub fn wasm_minify_svg(svg: &str) -> Result<String, JsValue> {
    minify_svg(svg).map_err(core_err)
}

/// Optimize path precision in SVG. `precision` must be 0–6.
#[wasm_bindgen]
pub fn wasm_optimize_paths(svg: &str, precision: u32) -> Result<String, JsValue> {
    optimize_paths(svg, precision).map_err(core_err)
}

/// Remove empty `<g>` groups from SVG. Runs recursively.
#[wasm_bindgen]
pub fn wasm_remove_empty_groups(svg: &str) -> Result<String, JsValue> {
    remove_empty_groups(svg).map_err(core_err)
}

/// Add or repair viewBox attribute.
#[wasm_bindgen]
pub fn wasm_fix_viewbox(svg: &str) -> Result<String, JsValue> {
    fix_viewbox(svg).map_err(core_err)
}

/// Convert SVG to data URI (`data:image/svg+xml,...`).
#[wasm_bindgen]
pub fn wasm_svg_to_data_uri(svg: &str) -> Result<String, JsValue> {
    svg_to_data_uri(svg).map_err(core_err)
}

/// Analyze SVG: extract dimensions, element counts, security flags.
/// Returns a JS object matching `SvgInfo`.
#[wasm_bindgen]
pub fn wasm_analyze_svg(svg: &str) -> Result<JsValue, JsValue> {
    let info = analyze_svg(svg).map_err(core_err)?;
    serde_to_js(&info)
}

/// Run the full optimization pipeline.
/// Returns `{ svg: string, stats: { original_size, optimized_size, savings_percent, paths_optimized, elements_removed } }`.
#[wasm_bindgen]
pub fn wasm_full_optimize(svg: &str, precision: u32) -> Result<JsValue, JsValue> {
    let (optimized, stats) = full_optimize(svg, precision).map_err(core_err)?;
    build_optimize_result(&optimized, &stats)
}

/// Full optimization + data URI in one call.
#[wasm_bindgen]
pub fn wasm_full_optimize_with_uri(svg: &str, precision: u32) -> Result<JsValue, JsValue> {
    let (optimized, stats) = full_optimize(svg, precision).map_err(core_err)?;
    let data_uri = svg_to_data_uri(&optimized).map_err(core_err)?;

    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"svg".into(), &JsValue::from_str(&optimized)).ok();
    js_sys::Reflect::set(&obj, &"stats".into(), &serde_to_js(&stats)?).ok();
    js_sys::Reflect::set(&obj, &"dataUri".into(), &JsValue::from_str(&data_uri)).ok();

    Ok(obj.into())
}

// ── panic hook ──────────────────────────────────────────────────────

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}
