# svg_tidy 产品规格

## 产品一句话

让前端开发者和设计师在浏览器中即时优化 SVG：压缩注释和空白、降低路径精度、修复 viewBox、一键转 Data URI。全部在 WASM 中完成。

## 目标用户

- 需要压缩 SVG 体积以提升页面加载速度的前端开发者。
- 从 Illustrator / Figma / Sketch 导出 SVG 后想清理编辑器残留的设计师。
- 需要批量优化图标库的团队。
- 希望将 SVG 内联为 Data URI 的 CSS 开发者。

## 核心体验

1. 用户粘贴 SVG 代码到左侧面板，或拖放 .svg 文件。
2. 选择优化选项：精度（0-6）、移除注释、移除元数据、修复 viewBox。
3. 右侧实时展示优化结果，顶部预览渲染效果。
4. 查看统计数据：原始/优化后大小、节省百分比、优化的路径数、移除的元素数。
5. 一键复制优化 SVG、下载 .svg 文件、复制 Data URI。

## 优化管道

```
输入 SVG
  → 移除注释 (<!-- ... -->)
  → 移除元数据 (<metadata>, <rdf:RDF>, 等)
  → 移除编辑器数据 (sodipodi:*, inkscape:*, 等属性)
  → 压缩空白 (合并空格、移除缩进和换行)
  → 移除空组 (<g></g> 或仅含空白的 <g>)
  → 优化路径精度 (d="..." 中的数字四舍五入)
  → 修复 viewBox (缺失时从 width/height 推算)
  → 计算统计数据
  → 输出 (优化后 SVG, SvgStats)
```

## API

### 优化函数

- `minify_svg(svg) -> String` — 移除注释、空白、元数据和编辑器数据
- `optimize_paths(svg, precision) -> String` — 降低路径坐标精度
- `remove_empty_groups(svg) -> String` — 移除空 `<g>` 元素
- `fix_viewbox(svg) -> String` — 添加/修复 viewBox
- `svg_to_data_uri(svg) -> String` — 转为 data: URI
- `full_optimize(svg, precision) -> (String, SvgStats)` — 全管道优化

### 分析函数

- `analyze_svg(svg) -> SvgInfo` — 提取尺寸、元素计数、安全检查

### 结构体

```rust
pub struct SvgStats {
    pub original_size: usize,     // 原始字节数
    pub optimized_size: usize,    // 优化后字节数
    pub savings_percent: f32,     // 节省百分比
    pub paths_optimized: usize,   // 优化的路径数
    pub elements_removed: usize,  // 移除的元素数
}

pub struct SvgInfo {
    pub width: Option<f64>,       // width 属性值
    pub height: Option<f64>,      // height 属性值
    pub viewbox: Option<String>,  // viewBox 属性值
    pub element_count: usize,     // 总元素数
    pub path_count: usize,        // <path> 元素数
    pub group_count: usize,       // <g> 元素数
    pub has_scripts: bool,        // 是否含 <script>
    pub has_external_refs: bool,  // 是否含外部引用
}
```

## 精度控制

路径坐标精度从 0 到 6 位小数可调：

| 精度 | 效果 | 示例 |
|------|------|------|
| 0 | 整数坐标 | `M10,20L30,40` |
| 1 | 1 位小数 | `M10.5,20.2L30.7,40.1` |
| 2 | 2 位小数（默认） | `M10.52,20.18L30.71,40.13` |
| 6 | 6 位小数（近似无损） | `M10.523412,20.184721L30.714523,40.132564` |

## 安全

- 检测 `<script>` 元素和 `on*` 事件属性，通过 `SvgInfo.has_scripts` 暴露。
- 检测外部引用（`xlink:href`、`url()`），通过 `SvgInfo.has_external_refs` 暴露。
- 预览使用 `<img>` + Data URI 渲染，避免内联 SVG 的脚本执行风险。
- 所有处理在浏览器 WASM 中完成，SVG 内容不上传任何服务器。

## 非目标

- 不做完整的 XML 解析和 DOM 操作（使用正则/字符串操作保持体积小）。
- 不处理 CSS class 合并或样式去重。
- 不做 SVG 到其他格式的转换（PNG、WebP 等）。
- 不做图片内嵌（base64 图片转 data URI）。
- 不做 SVG 动画优化。

## 验收标准

- 典型 Illustrator SVG 优化后体积减少 20-60%。
- 路径精度优化后渲染效果肉眼无差异（精度 2+）。
- viewBox 修复后 SVG 在各浏览器中正确缩放。
- Data URI 可在 `<img src="...">` 和 CSS `background: url(...)` 中正常使用。
- 预览渲染与原 SVG 视觉一致。
- 所有处理在浏览器完成，无网络请求发出。
