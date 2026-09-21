pub mod error;
pub mod info;
pub mod optimize;

pub use error::CoreError;
pub use info::{SvgInfo, analyze_svg};
pub use optimize::{
    SvgStats, fix_viewbox, full_optimize, minify_svg, optimize_paths, remove_empty_groups,
    svg_to_data_uri,
};
