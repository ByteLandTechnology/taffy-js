//! Taffy WebAssembly bindings
//!
//! This library provides WebAssembly bindings for the Taffy layout library,
//! allowing it to be used in JavaScript/TypeScript environments.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use taffy::{
    style as taffy_style,
    geometry::Size as TaffySize,
    tree::TaffyTree,
    prelude::*,
};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ============================================================================
// Type definitions
// ============================================================================

/// Represents a length value in CSS.
///
/// This struct corresponds to a dimension value that can be specified in pixels, percentage, or auto.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// The numeric value of the dimension.
    /// - For `Pixels`, this is the number of pixels.
    /// - For `Percent`, this is the percentage value (0.0 to 100.0, or sometimes 0.0 to 1.0 depending on context, handled by internal logic).
    /// - For `Auto`, this value is typically ignored.
    pub value: f32,
    /// The unit of the dimension.
    pub unit: DimensionUnit,
}

/// The unit of a dimension.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DimensionUnit {
    /// The dimension is specified in logical pixels.
    Pixels,
    /// The dimension is specified as a percentage of the parent's size.
    Percent,
    /// The dimension is determined automatically based on content or context.
    Auto,
}

/// Represents a point in 2D space.
///
/// Typically used for coordinates like absolute positioning offsets.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Point {
    /// The x-coordinate (horizontal).
    pub x: f32,
    /// The y-coordinate (vertical).
    pub y: f32,
}

/// Represents a size in 2D space.
///
/// Used for width and height dimensions.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Size {
    /// The width dimension.
    pub width: f32,
    /// The height dimension.
    pub height: f32,
}

impl From<TaffySize<f32>> for Size {
    fn from(size: TaffySize<f32>) -> Self {
        Size { width: size.width, height: size.height }
    }
}

/// Represents a rectangle defined by its edges.
///
/// Used for padding, margin, properties.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Rect {
    /// The left edge value.
    pub left: f32,
    /// The right edge value.
    pub right: f32,
    /// The top edge value.
    pub top: f32,
    /// The bottom edge value.
    pub bottom: f32,
}

/// Represents the computed layout of a node.
///
/// This struct contains the final position and size of a node after layout computation.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Layout {
    /// The absolute x-coordinate of the node relative to its parent.
    pub x: f32,
    /// The absolute y-coordinate of the node relative to its parent.
    pub y: f32,
    /// The computed width of the node.
    pub width: f32,
    /// The computed height of the node.
    pub height: f32,
}

impl From<taffy::Layout> for Layout {
    fn from(layout: taffy::Layout) -> Self {
        Layout {
            x: layout.location.x,
            y: layout.location.y,
            width: layout.size.width,
            height: layout.size.height,
        }
    }
}

/// The display style of a node.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Display {
    /// The node is hidden and does not take up space.
    None,
    /// The node behaves as a flex container.
    Flex,
    /// The node behaves as a grid container.
    Grid,
    /// The node behaves as a block element.
    Block,
}

/// Text alignment within a node (mostly ignored in flex/grid layout but preserved for compatibility).
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
    Justify,
}

/// The direction of the main axis for a flex container.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FlexDirection {
    /// Items are placed horizontally from left to right.
    Row,
    /// Items are placed vertically from top to bottom.
    Column,
    /// Items are placed horizontally from right to left.
    RowReverse,
    /// Items are placed vertically from bottom to top.
    ColumnReverse,
}

/// How items are distributed along the main axis.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum JustifyContent {
    /// Items are packed toward the start of the layout direction.
    Start,
    /// Items are packed toward the end of the layout direction.
    End,
    /// Items are packed toward the start of the flex-direction.
    FlexStart,
    /// Items are packed toward the end of the flex-direction.
    FlexEnd,
    /// Items are centered along the line.
    Center,
    /// Items are evenly distributed; the first item is at the start, the last at the end.
    SpaceBetween,
    /// Items are evenly distributed with equal space around them.
    SpaceAround,
    /// Items are evenly distributed with equal space between them using the same gap at ends.
    SpaceEvenly,
}

/// How items are aligned along the cross axis.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AlignItems {
    /// Items are aligned at the start of the cross axis.
    Start,
    /// Items are aligned at the end of the cross axis.
    End,
    /// Items are aligned at the start of the flex-direction cross axis.
    FlexStart,
    /// Items are aligned at the end of the flex-direction cross axis.
    FlexEnd,
    /// Items are aligned at the center of the cross axis.
    Center,
    /// Items are aligned based on their baselines.
    Baseline,
    /// Items are stretched to fill the container along the cross axis.
    Stretch,
}

/// How a single item is aligned along the cross axis, overriding `AlignItems`.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AlignSelf {
    /// items are aligned at the start of the cross axis.
    Start,
    /// Items are aligned at the end of the cross axis.
    End,
    /// Items are aligned at the start of the flex-direction cross axis.
    FlexStart,
    /// Items are aligned at the end of the flex-direction cross axis.
    FlexEnd,
    /// Items are aligned at the center of the cross axis.
    Center,
    /// Items are aligned based on their baselines.
    Baseline,
    /// Items are stretched to fill the container along the cross axis.
    Stretch,
}

/// How lines of content are aligned along the cross axis when there is extra space.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AlignContent {
    /// Lines are packed toward the start of the cross axis.
    Start,
    /// Lines are packed toward the end of the cross axis.
    End,
    /// Lines are packed toward the start of the flex-direction cross axis.
    FlexStart,
    /// Lines are packed toward the end of the flex-direction cross axis.
    FlexEnd,
    /// Lines are packed toward the center of the cross axis.
    Center,
    /// Lines are evenly distributed; the first line is at the start, the last at the end.
    SpaceBetween,
    /// Lines are evenly distributed with equal space around them.
    SpaceAround,
    /// Lines are evenly distributed with equal space between them.
    SpaceEvenly,
    /// Lines are stretched to take up the remaining space.
    Stretch,
}

/// Whether flex items are forced into a single line or can wrap onto multiple lines.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FlexWrap {
    /// Items are forced into a single line.
    NoWrap,
    /// Items wrap onto multiple lines.
    Wrap,
    /// Items wrap onto multiple lines in reverse order.
    WrapReverse,
}

/// Positioning strategy for a node.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Position {
    /// Relative to its normal position in the flow.
    Static, // Note: Taffy treats Static mostly same as Relative for layout purposes
    /// Relative to its normal position in the flow.
    Relative,
    /// Removed from the flow and positioned relative to its containing block.
    Absolute,
}

/// Grid auto-placement algorithm controls how auto-placed items get flowed into the grid.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GridAutoFlow {
    /// Items are placed by filling each row in turn, adding new rows as necessary.
    Row,
    /// Items are placed by filling each column in turn, adding new columns as necessary.
    Column,
    /// Items are placed by filling each row, attempting to fill holes earlier in the grid.
    RowDense,
    /// Items are placed by filling each column, attempting to fill holes earlier in the grid.
    ColumnDense,
}

/// Style values for a node.
///
/// This struct aggregates all layout styles that can be applied to a node.
/// Optional fields allow partial updates or default values.
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Style {
    /// The display mode of the node (e.g. Flex, Grid, None).
    pub display: Option<Display>,
    /// The positioning strategy (e.g. Relative, Absolute).
    pub position: Option<Position>,

    // Size
    /// The width of the node.
    pub width: Option<Dimension>,
    /// The height of the node.
    pub height: Option<Dimension>,
    /// The minimum width of the node.
    pub min_width: Option<Dimension>,
    /// The minimum height of the node.
    pub min_height: Option<Dimension>,
    /// The maximum width of the node.
    pub max_width: Option<Dimension>,
    /// The maximum height of the node.
    pub max_height: Option<Dimension>,

    // Position
    /// The offset from the left edge (used with Position::Absolute/Relative).
    pub left: Option<Dimension>,
    /// The offset from the right edge.
    pub right: Option<Dimension>,
    /// The offset from the top edge.
    pub top: Option<Dimension>,
    /// The offset from the bottom edge.
    pub bottom: Option<Dimension>,

    // Margin
    /// The margin on the left side.
    pub margin_left: Option<Dimension>,
    /// The margin on the right side.
    pub margin_right: Option<Dimension>,
    /// The margin on the top side.
    pub margin_top: Option<Dimension>,
    /// The margin on the bottom side.
    pub margin_bottom: Option<Dimension>,

    // Padding
    /// The padding on the left side.
    pub padding_left: Option<Dimension>,
    /// The padding on the right side.
    pub padding_right: Option<Dimension>,
    /// The padding on the top side.
    pub padding_top: Option<Dimension>,
    /// The padding on the bottom side.
    pub padding_bottom: Option<Dimension>,

    // Flexbox
    /// The flex direction (e.g. Row, Column).
    pub flex_direction: Option<FlexDirection>,
    /// Whether flex items should wrap.
    pub flex_wrap: Option<FlexWrap>,
    /// How much the item will grow relative to the rest of the flexible items.
    pub flex_grow: Option<f32>,
    /// How much the item will shrink relative to the rest of the flexible items.
    pub flex_shrink: Option<f32>,
    /// The initial main size of a flex item.
    pub flex_basis: Option<Dimension>,
    /// How items are distributed along the main axis.
    pub justify_content: Option<JustifyContent>,
    /// How items are aligned along the cross axis.
    pub align_items: Option<AlignItems>,
    /// How a single item is aligned along the cross axis.
    pub align_self: Option<AlignSelf>,
    /// How lines of content are aligned along the cross axis.
    pub align_content: Option<AlignContent>,
    /// The gap between rows (flex/grid).
    pub row_gap: Option<Dimension>,
    /// The gap between columns (flex/grid).
    pub column_gap: Option<Dimension>,

    // Grid
    /// Row track definitions for grid layout.
    #[wasm_bindgen(skip)]
    pub grid_template_rows: Option<Vec<TrackDefinition>>,
    /// Column track definitions for grid layout.
    #[wasm_bindgen(skip)]
    pub grid_template_columns: Option<Vec<TrackDefinition>>,
    /// Auto-generated row track definitions.
    #[wasm_bindgen(skip)]
    pub grid_auto_rows: Option<Vec<TrackDefinition>>,
    /// Auto-generated column track definitions.
    #[wasm_bindgen(skip)]
    pub grid_auto_columns: Option<Vec<TrackDefinition>>,
    /// Algorithm for auto-placing items in the grid.
    pub grid_auto_flow: Option<GridAutoFlow>,
    /// Placement of the item in grid rows.
    #[wasm_bindgen(skip)]
    pub grid_row: Option<Line>,
    /// Placement of the item in grid columns.
    #[wasm_bindgen(skip)]
    pub grid_column: Option<Line>,

    // Block
    /// Text alignment.
    pub text_align: Option<TextAlign>,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            display: Some(Display::Flex),
            text_align: None,
            position: Some(Position::Relative),
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            left: None,
            right: None,
            top: None,
            bottom: None,
            margin_left: None,
            margin_right: None,
            margin_top: None,
            margin_bottom: None,
            padding_left: None,
            padding_right: None,
            padding_top: None,
            padding_bottom: None,
            flex_direction: None,
            flex_wrap: None,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            justify_content: None,
            align_items: None,
            align_self: None,
            align_content: None,
            row_gap: None,
            column_gap: None,
            grid_template_rows: None,
            grid_template_columns: None,
            grid_auto_rows: None,
            grid_auto_columns: None,
            grid_auto_flow: None,
            grid_row: None,
            grid_column: None,
        }
    }
}

/// Definition of a single grid track (row or column).
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TrackDefinition {
    /// The numeric value of the track size.
    pub value: f32,
    /// The unit of the track size.
    pub unit: TrackUnit,
}

/// The unit for a grid track definition.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TrackUnit {
    /// The track size is specified in logical pixels.
    Pixels,
    /// The track size is specified as a percentage of the container.
    Percent,
    /// The track size is a fraction of the remaining free space (fr unit).
    Fraction, // 'fr' unit
    /// The track size is determined automatically.
    Auto,
    /// The track size is the minimum size needed to fit the content.
    MinContent,
    /// The track size is the maximum size needed to fit the content.
    MaxContent,
}

/// Represents a grid line placement (start and end).
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Line {
    /// The start line index (1-based).
    pub start: Option<i16>,
    /// The end line index (1-based).
    pub end: Option<i16>,
}

/// The available space for layout computation.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AvailableSpace {
    /// The available width (None means undefined/max-content).
    pub width: Option<f32>,
    /// The available height (None means undefined/max-content).
    pub height: Option<f32>,
}

// ============================================================================
// Conversion helpers
// ============================================================================

fn convert_style(style: &Style) -> taffy_style::Style {
    let mut taffy_style = taffy_style::Style::default();

    // Display
    if let Some(display) = style.display {
        taffy_style.display = match display {
            Display::None => taffy_style::Display::None,
            Display::Flex => taffy_style::Display::Flex,
            Display::Grid => taffy_style::Display::Grid,
            Display::Block => taffy_style::Display::Block,
        };
    }

    // Position
    if let Some(position) = style.position {
        taffy_style.position = match position {
            Position::Static => taffy_style::Position::Relative,
            Position::Relative => taffy_style::Position::Relative,
            Position::Absolute => taffy_style::Position::Absolute,
        };
    }

    // Size
    fn to_dimension(dim: &Dimension) -> taffy_style::Dimension {
        match dim.unit {
            DimensionUnit::Pixels => taffy_style::Dimension::length(dim.value),
            DimensionUnit::Percent => taffy_style::Dimension::percent(dim.value / 100.0),
            DimensionUnit::Auto => taffy_style::Dimension::auto(),
        }
    }

    fn to_length_percentage_auto(dim: &Dimension) -> taffy_style::LengthPercentageAuto {
        match dim.unit {
            DimensionUnit::Pixels => taffy_style::LengthPercentageAuto::length(dim.value),
            DimensionUnit::Percent => taffy_style::LengthPercentageAuto::percent(dim.value / 100.0),
            DimensionUnit::Auto => taffy_style::LengthPercentageAuto::auto(),
        }
    }

    fn to_length_percentage(dim: &Dimension) -> taffy_style::LengthPercentage {
        match dim.unit {
            DimensionUnit::Pixels => taffy_style::LengthPercentage::length(dim.value),
            DimensionUnit::Percent => taffy_style::LengthPercentage::percent(dim.value / 100.0),
            DimensionUnit::Auto => taffy_style::LengthPercentage::length(0.0),
        }
    }

    if let Some(ref w) = style.width { taffy_style.size.width = to_dimension(w); }
    if let Some(ref h) = style.height { taffy_style.size.height = to_dimension(h); }
    if let Some(ref w) = style.min_width { taffy_style.min_size.width = to_dimension(w); }
    if let Some(ref h) = style.min_height { taffy_style.min_size.height = to_dimension(h); }
    if let Some(ref w) = style.max_width { taffy_style.max_size.width = to_dimension(w); }
    if let Some(ref h) = style.max_height { taffy_style.max_size.height = to_dimension(h); }

    // Position (inset)
    if let Some(ref l) = style.left { taffy_style.inset.left = to_length_percentage_auto(l); }
    if let Some(ref r) = style.right { taffy_style.inset.right = to_length_percentage_auto(r); }
    if let Some(ref t) = style.top { taffy_style.inset.top = to_length_percentage_auto(t); }
    if let Some(ref b) = style.bottom { taffy_style.inset.bottom = to_length_percentage_auto(b); }

    // Margin
    if let Some(ref m) = style.margin_left { taffy_style.margin.left = to_length_percentage_auto(m); }
    if let Some(ref m) = style.margin_right { taffy_style.margin.right = to_length_percentage_auto(m); }
    if let Some(ref m) = style.margin_top { taffy_style.margin.top = to_length_percentage_auto(m); }
    if let Some(ref m) = style.margin_bottom { taffy_style.margin.bottom = to_length_percentage_auto(m); }

    // Padding
    if let Some(ref p) = style.padding_left { taffy_style.padding.left = to_length_percentage(p); }
    if let Some(ref p) = style.padding_right { taffy_style.padding.right = to_length_percentage(p); }
    if let Some(ref p) = style.padding_top { taffy_style.padding.top = to_length_percentage(p); }
    if let Some(ref p) = style.padding_bottom { taffy_style.padding.bottom = to_length_percentage(p); }

    // Flexbox
    if let Some(dir) = style.flex_direction {
        taffy_style.flex_direction = match dir {
            FlexDirection::Row => taffy_style::FlexDirection::Row,
            FlexDirection::Column => taffy_style::FlexDirection::Column,
            FlexDirection::RowReverse => taffy_style::FlexDirection::RowReverse,
            FlexDirection::ColumnReverse => taffy_style::FlexDirection::ColumnReverse,
        };
    }

    if let Some(wrap) = style.flex_wrap {
        taffy_style.flex_wrap = match wrap {
            FlexWrap::NoWrap => taffy_style::FlexWrap::NoWrap,
            FlexWrap::Wrap => taffy_style::FlexWrap::Wrap,
            FlexWrap::WrapReverse => taffy_style::FlexWrap::WrapReverse,
        };
    }

    if let Some(g) = style.flex_grow { taffy_style.flex_grow = g; }
    if let Some(s) = style.flex_shrink { taffy_style.flex_shrink = s; }
    if let Some(ref b) = style.flex_basis { taffy_style.flex_basis = to_dimension(b); }

    if let Some(jc) = style.justify_content {
        taffy_style.justify_content = Some(match jc {
            JustifyContent::Start => taffy_style::JustifyContent::Start,
            JustifyContent::End => taffy_style::JustifyContent::End,
            JustifyContent::FlexStart => taffy_style::JustifyContent::FlexStart,
            JustifyContent::FlexEnd => taffy_style::JustifyContent::FlexEnd,
            JustifyContent::Center => taffy_style::JustifyContent::Center,
            JustifyContent::SpaceBetween => taffy_style::JustifyContent::SpaceBetween,
            JustifyContent::SpaceAround => taffy_style::JustifyContent::SpaceAround,
            JustifyContent::SpaceEvenly => taffy_style::JustifyContent::SpaceEvenly,
        });
    }

    if let Some(ai) = style.align_items {
        taffy_style.align_items = Some(match ai {
            AlignItems::Start => taffy_style::AlignItems::Start,
            AlignItems::End => taffy_style::AlignItems::End,
            AlignItems::FlexStart => taffy_style::AlignItems::FlexStart,
            AlignItems::FlexEnd => taffy_style::AlignItems::FlexEnd,
            AlignItems::Center => taffy_style::AlignItems::Center,
            AlignItems::Baseline => taffy_style::AlignItems::Baseline,
            AlignItems::Stretch => taffy_style::AlignItems::Stretch,
        });
    }

    if let Some(align_self) = style.align_self {
        taffy_style.align_self = Some(match align_self {
            AlignSelf::Start => taffy_style::AlignItems::Start,
            AlignSelf::End => taffy_style::AlignItems::End,
            AlignSelf::FlexStart => taffy_style::AlignItems::FlexStart,
            AlignSelf::FlexEnd => taffy_style::AlignItems::FlexEnd,
            AlignSelf::Center => taffy_style::AlignItems::Center,
            AlignSelf::Baseline => taffy_style::AlignItems::Baseline,
            AlignSelf::Stretch => taffy_style::AlignItems::Stretch,
        });
    }

    if let Some(ac) = style.align_content {
        taffy_style.align_content = Some(match ac {
            AlignContent::Start => taffy_style::AlignContent::Start,
            AlignContent::End => taffy_style::AlignContent::End,
            AlignContent::FlexStart => taffy_style::AlignContent::FlexStart,
            AlignContent::FlexEnd => taffy_style::AlignContent::FlexEnd,
            AlignContent::Center => taffy_style::AlignContent::Center,
            AlignContent::SpaceBetween => taffy_style::AlignContent::SpaceBetween,
            AlignContent::SpaceAround => taffy_style::AlignContent::SpaceAround,
            AlignContent::SpaceEvenly => taffy_style::AlignContent::SpaceEvenly,
            AlignContent::Stretch => taffy_style::AlignContent::Stretch,
        });
    }

    if let Some(ref g) = style.row_gap { taffy_style.gap.width = to_length_percentage(g); }
    if let Some(ref g) = style.column_gap { taffy_style.gap.height = to_length_percentage(g); }

    // Grid
    fn to_track_sizing_function(track: &TrackDefinition) -> taffy_style::TrackSizingFunction {

        use taffy::geometry::MinMax;
        use taffy::style_helpers::{length, percent, fr, auto, min_content, max_content};

        match track.unit {
            TrackUnit::Pixels => MinMax {
                min: length(track.value),
                max: length(track.value),
            },
            TrackUnit::Percent => MinMax {
                min: percent(track.value / 100.0),
                max: percent(track.value / 100.0),
            },
            TrackUnit::Fraction => MinMax {
                min: auto(),
                max: fr(track.value),
            },
            TrackUnit::Auto => MinMax { min: auto(), max: auto() },
            TrackUnit::MinContent => MinMax { min: min_content(), max: min_content() },
            TrackUnit::MaxContent => MinMax { min: max_content(), max: max_content() },
        }
    }

    // Helper to convert TrackDefinition to NonRepeatedTrackSizingFunction if possible, 
    // or we construct GridTemplateComponent which supports Repeat.
    
    fn to_grid_template_blocks(tracks: &[TrackDefinition]) -> Vec<taffy_style::GridTemplateComponent<<taffy_style::Style as taffy_style::CoreStyle>::CustomIdent>> {
        tracks.iter().map(|t| {
             let sizing = to_track_sizing_function(t);
             taffy_style::GridTemplateComponent::Single(sizing)
        }).collect()
    }
    
    fn to_grid_auto_blocks(tracks: &[TrackDefinition]) -> Vec<taffy_style::TrackSizingFunction> {
        tracks.iter().map(|t| {
            to_track_sizing_function(t)
        }).collect()
    }

    if let Some(ref tracks) = style.grid_template_rows {
        taffy_style.grid_template_rows = to_grid_template_blocks(tracks);
    }
    if let Some(ref tracks) = style.grid_template_columns {
        taffy_style.grid_template_columns = to_grid_template_blocks(tracks);
    }
    if let Some(ref tracks) = style.grid_auto_rows {
        taffy_style.grid_auto_rows = to_grid_auto_blocks(tracks);
    }
    if let Some(ref tracks) = style.grid_auto_columns {
        taffy_style.grid_auto_columns = to_grid_auto_blocks(tracks);
    }

    if let Some(flow) = style.grid_auto_flow {
        taffy_style.grid_auto_flow = match flow {
            GridAutoFlow::Row => taffy_style::GridAutoFlow::Row,
            GridAutoFlow::Column => taffy_style::GridAutoFlow::Column,
            GridAutoFlow::RowDense => taffy_style::GridAutoFlow::RowDense,
            GridAutoFlow::ColumnDense => taffy_style::GridAutoFlow::ColumnDense,
        };
    }

    if let Some(ref line) = style.grid_row {
        taffy_style.grid_row = taffy::geometry::Line {
            start: line.start.map_or(taffy_style::GridPlacement::Auto, |v| taffy_style::GridPlacement::from_line_index(v)),
            end: line.end.map_or(taffy_style::GridPlacement::Auto, |v| taffy_style::GridPlacement::from_line_index(v)),
        };
    }
    if let Some(ref line) = style.grid_column {
        taffy_style.grid_column = taffy::geometry::Line {
            start: line.start.map_or(taffy_style::GridPlacement::Auto, |v| taffy_style::GridPlacement::from_line_index(v)),
            end: line.end.map_or(taffy_style::GridPlacement::Auto, |v| taffy_style::GridPlacement::from_line_index(v)),
        };
    }

    if let Some(align) = style.text_align {
        taffy_style.text_align = match align {
            TextAlign::Auto => taffy_style::TextAlign::Auto,
            TextAlign::Left => taffy_style::TextAlign::LegacyLeft,
            TextAlign::Right => taffy_style::TextAlign::LegacyRight,
            TextAlign::Center => taffy_style::TextAlign::LegacyCenter,
            TextAlign::Justify => taffy_style::TextAlign::Auto,
        };
    }

    taffy_style
}

fn to_available_space(space: &AvailableSpace) -> TaffySize<taffy_style::AvailableSpace> {
    TaffySize {
        width: match space.width {
            Some(w) => taffy_style::AvailableSpace::Definite(w),
            None => taffy_style::AvailableSpace::MaxContent,
        },
        height: match space.height {
            Some(h) => taffy_style::AvailableSpace::Definite(h),
            None => taffy_style::AvailableSpace::MaxContent,
        },
    }
}

// ============================================================================
// Taffy wrapper
// ============================================================================

thread_local! {
    static TAFFY: RefCell<TaffyTree<()>> = RefCell::new(TaffyTree::new());
    static NEXT_NODE_ID: RefCell<u32> = RefCell::new(0);
}

/// Creates a new leaf node with the specified style.
///
/// # Arguments
///
/// * `style` - The style object to apply to the new node.
///
/// # Returns
///
/// The ID of the created node as a `u32`.
///
/// # Errors
///
/// Returns a `JsValue` error if the style cannot be deserialized or if node creation fails.
#[wasm_bindgen]
pub fn new_leaf(style: JsValue) -> Result<u32, JsValue> {
    TAFFY.with(|taffy| {
        let style: Style = serde_wasm_bindgen::from_value(style)?;
        let taffy_style = convert_style(&style);

        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy_ref.new_leaf(taffy_style)
            .map_err(|e| JsValue::from_str(&format!("Failed to create leaf: {}", e)))?;

        let id: u64 = node_id.into();
        Ok(id as u32)
    })
}

/// Creates a new node with children and the specified style.
///
/// # Arguments
///
/// * `style` - The style object to apply to the new node.
/// * `children` - An array of child node IDs (`u32`) to attach to this node.
///
/// # Returns
///
/// The ID of the created node as a `u32`.
///
/// # Errors
///
/// Returns a `JsValue` error if the style cannot be deserialized or if node creation fails.
#[wasm_bindgen]
pub fn new_with_children(style: JsValue, children: &[u32]) -> Result<u32, JsValue> {
    TAFFY.with(|taffy| {
        let style: Style = serde_wasm_bindgen::from_value(style)?;
        let taffy_style = convert_style(&style);

        let child_ids: Vec<taffy::NodeId> = children.iter()
            .map(|id| taffy::NodeId::from(*id as u64))
            .collect();

        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy_ref.new_with_children(taffy_style, &child_ids)
            .map_err(|e| JsValue::from_str(&format!("Failed to create node: {}", e)))?;

        let id: u64 = node_id.into();
        Ok(id as u32)
    })
}

/// Adds a child node to a parent node.
///
/// # Arguments
///
/// * `parent` - The ID of the parent node.
/// * `child` - The ID of the child node to add.
///
/// # Errors
///
/// Returns a `JsValue` error if the operation fails (e.g., recursive hierarchy).
#[wasm_bindgen]
pub fn add_child(parent: u32, child: u32) -> Result<(), JsValue> {
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let parent_id = taffy::NodeId::from(parent as u64);
        let child_id = taffy::NodeId::from(child as u64);

        taffy_ref.add_child(parent_id, child_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to add child: {}", e)))?;
        Ok(())
    })
}

/// Removes a child node from a parent node.
///
/// # Arguments
///
/// * `parent` - The ID of the parent node.
/// * `child` - The ID of the child node to remove.
///
/// # Errors
///
/// Returns a `JsValue` error if the child is not found in the parent.
#[wasm_bindgen]
pub fn remove_child(parent: u32, child: u32) -> Result<(), JsValue> {
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let parent_id = taffy::NodeId::from(parent as u64);
        let child_id = taffy::NodeId::from(child as u64);

        taffy_ref.remove_child(parent_id, child_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to remove child: {}", e)))?;
        Ok(())
    })
}

/// Sets the children of a node, replacing any existing children.
///
/// # Arguments
///
/// * `parent` - The ID of the parent node.
/// * `children` - An array of child node IDs to set.
///
/// # Errors
///
/// Returns a `JsValue` error if the operation fails.
#[wasm_bindgen]
pub fn set_children(parent: u32, children: &[u32]) -> Result<(), JsValue> {
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let parent_id = taffy::NodeId::from(parent as u64);

        let child_ids: Vec<taffy::NodeId> = children.iter()
            .map(|id| taffy::NodeId::from(*id as u64))
            .collect();

        taffy_ref.set_children(parent_id, &child_ids)
            .map_err(|e| JsValue::from_str(&format!("Failed to set children: {}", e)))?;
        Ok(())
    })
}

/// Removes a node from the tree and frees its resources.
///
/// # Arguments
///
/// * `node` - The ID of the node to remove.
///
/// # Errors
///
/// Returns a `JsValue` error if the node does not exist or cannot be removed.
#[wasm_bindgen]
pub fn remove_node(node: u32) -> Result<(), JsValue> {
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy::NodeId::from(node as u64);

        taffy_ref.remove(node_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to remove node: {}", e)))?;
        Ok(())
    })
}

/// Retrieves the list of children IDs for a given node.
///
/// # Arguments
///
/// * `parent` - The ID of the parent node.
///
/// # Returns
///
/// A boxed array of child node IDs (`Box<[u32]>`).
///
/// # Errors
///
/// Returns a `JsValue` error if the node does not exist.
#[wasm_bindgen]
pub fn get_children(parent: u32) -> Result<Box<[u32]>, JsValue> {
    TAFFY.with(|taffy| {
        let taffy_ref = taffy.borrow();
        let parent_id = taffy::NodeId::from(parent as u64);

        let children = taffy_ref.children(parent_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to get children: {}", e)))?;

        Ok(children.iter().map(|id: &taffy::NodeId| {
            let val: u64 = (*id).into();
            val as u32
        }).collect())
    })
}

/// Retrieves the parent ID of a given node.
///
/// # Arguments
///
/// * `node` - The ID of the node to query.
///
/// # Returns
///
/// An `Option<u32>` containing the parent ID if it exists, or `None` if the node is a root or orphan.
///
/// # Errors
///
/// Returns a `JsValue` error if internal tree access fails.
#[wasm_bindgen]
pub fn get_parent(node: u32) -> Result<Option<u32>, JsValue> {
    TAFFY.with(|taffy| {
        let taffy_ref = taffy.borrow();
        let node_id = taffy::NodeId::from(node as u64);

        Ok(taffy_ref.parent(node_id).map(|id| {
            let val: u64 = id.into();
            val as u32
        }))
    })
}

/// Updates the style of an existing node.
///
/// # Arguments
///
/// * `node` - The ID of the node to update.
/// * `style` - The new style object to apply.
///
/// # Errors
///
/// Returns a `JsValue` error if the style cannot be deserialized or if the node does not exist.
#[wasm_bindgen]
pub fn set_style(node: u32, style: JsValue) -> Result<(), JsValue> {
    TAFFY.with(|taffy| {
        let style: Style = serde_wasm_bindgen::from_value(style)?;
        let taffy_style = convert_style(&style);

        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy::NodeId::from(node as u64);

        taffy_ref.set_style(node_id, taffy_style)
            .map_err(|e| JsValue::from_str(&format!("Failed to set style: {}", e)))?;
        Ok(())
    })
}

/// Computes the layout for a tree starting from the specified root node.
///
/// # Arguments
///
/// * `root` - The ID of the root node of the tree to lay out.
/// * `available_space` - The available space constraints for the layout.
///
/// # Errors
///
/// Returns a `JsValue` error if the layout computation fails.
#[wasm_bindgen]
pub fn compute_layout(root: u32, available_space: JsValue) -> Result<(), JsValue> {
    TAFFY.with(|taffy| {
        let space: AvailableSpace = serde_wasm_bindgen::from_value(available_space)?;
        let taffy_space = to_available_space(&space);

        let mut taffy_ref = taffy.borrow_mut();
        let root_id = taffy::NodeId::from(root as u64);

        taffy_ref.compute_layout(root_id, taffy_space)
            .map_err(|e| JsValue::from_str(&format!("Failed to compute layout: {}", e)))?;
        Ok(())
    })
}

/// Retrieves the computed layout information for a specific node.
///
/// # Arguments
///
/// * `node` - The ID of the node to query.
///
/// # Returns
///
/// A `Layout` object containing the x, y, width, and height of the node.
///
/// # Errors
///
/// Returns a `JsValue` error if the node does not exist or layout information is unavailable.
#[wasm_bindgen]
pub fn get_layout(node: u32) -> Result<JsValue, JsValue> {
    TAFFY.with(|taffy| {
        let taffy_ref = taffy.borrow();
        let node_id = taffy::NodeId::from(node as u64);

        let layout = taffy_ref.layout(node_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to get layout: {}", e)))?;

        let result = Layout::from(*layout);
        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize layout: {}", e)))
    })
}

/// Marks a node and its ancestors as dirty, requiring a layout re-computation.
///
/// # Arguments
///
/// * `node` - The ID of the node to mark dirty.
///
/// # Errors
///
/// Returns a `JsValue` error if the node does not exist.
#[wasm_bindgen]
pub fn mark_dirty(node: u32) -> Result<(), JsValue> {
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy::NodeId::from(node as u64);

        taffy_ref.mark_dirty(node_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to mark dirty: {}", e)))?;
        Ok(())
    })
}

/// Clear all nodes
#[wasm_bindgen]
pub fn clear() -> Result<(), JsValue> {
    TAFFY.with(|taffy| {
        taffy.borrow_mut().clear();
        Ok(())
    })
}

/// Get the total number of nodes
#[wasm_bindgen]
pub fn node_count() -> u32 {
    TAFFY.with(|taffy| {
        taffy.borrow().total_node_count() as u32
    })
}

/// Helper function to create a dimension
#[wasm_bindgen]
pub fn dimension(value: f32, unit: DimensionUnit) -> Dimension {
    Dimension { value, unit }
}

/// Helper function to create a pixel dimension
#[wasm_bindgen]
pub fn px(value: f32) -> Dimension {
    Dimension { value, unit: DimensionUnit::Pixels }
}

/// Helper function to create a percent dimension
#[wasm_bindgen]
pub fn percent(value: f32) -> Dimension {
    Dimension { value, unit: DimensionUnit::Percent }
}

/// Auto dimension constant
#[wasm_bindgen]
pub fn auto() -> Dimension {
    Dimension { value: 0.0, unit: DimensionUnit::Auto }
}

// ============================================================================
// Utility functions
// ============================================================================

/// Initialize console error panic hook
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// ============================================================================
// OO Wrapper (Yoga-like API)
// ============================================================================

#[wasm_bindgen]
pub struct TaffyNode {
    pub id: u32,
}

#[wasm_bindgen]
impl TaffyNode {
    #[wasm_bindgen(constructor)]
    pub fn new(style: JsValue) -> Result<TaffyNode, JsValue> {
        let id = new_leaf(style)?;
        Ok(TaffyNode { id })
    }

    pub fn free(self) -> Result<(), JsValue> {
        remove_node(self.id)
    }

    pub fn set_style(&self, style: JsValue) -> Result<(), JsValue> {
        set_style(self.id, style)
    }
    
    pub fn style(&self) -> Result<JsValue, JsValue> {
        // Taffy doesn't easily expose getting the style back in the same format yet without keeping a copy.
        // For now, this might not be supported or we could implement reading from Taffy.
        // Taffy's style() returns taffy::Style, we would need to reverse convert it to our Style struct.
        Err(JsValue::from_str("getting style is not yet supported"))
    }
    
    pub fn compute_layout(&self, available_space: JsValue) -> Result<(), JsValue> {
        compute_layout(self.id, available_space)
    }
    
    pub fn get_layout(&self) -> Result<Layout, JsValue> {
        TAFFY.with(|taffy| {
            let taffy_ref = taffy.borrow();
            let node_id = taffy::NodeId::from(self.id as u64);
            let layout = taffy_ref.layout(node_id)
                .map_err(|e| JsValue::from_str(&format!("Failed to get layout: {}", e)))?;
            Ok(Layout::from(*layout))
        })
    }
    
    pub fn add_child(&self, child: &TaffyNode) -> Result<(), JsValue> {
        add_child(self.id, child.id)
    }
    
    pub fn remove_child(&self, child: &TaffyNode) -> Result<(), JsValue> {
        remove_child(self.id, child.id)
    }
    
    pub fn set_children(&self, children: &[u32]) -> Result<(), JsValue> {
        // Note: this takes raw IDs because handling array of TaffyNode objects from JS is tricky in wasm-bindgen
        set_children(self.id, children)
    }
}
