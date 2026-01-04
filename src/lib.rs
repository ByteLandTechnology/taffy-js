//! # Taffy-JS: WebAssembly Bindings for Taffy Layout Engine
//!
//! This crate provides WebAssembly bindings for the Taffy layout engine, enabling
//! JavaScript/TypeScript applications to use high-performance Flexbox and CSS Grid
//! layout algorithms.
//!
//! ## Architecture Overview
//!
//! - **Enums**: CSS layout enum types (Display, Position, FlexDirection, etc.)
//! - **DTOs**: Data Transfer Objects for JS <-> Rust serialization
//! - **Style**: Node style configuration containing all CSS layout properties
//! - **TaffyTree**: Layout tree manager for node creation, tree manipulation, and layout computation

// =============================================================================
// Imports
// =============================================================================

/// Taffy core style types (renamed to TaffyStyle to avoid conflict with local Style)
use taffy::style::{Style as TaffyStyle, AvailableSpace, Dimension, LengthPercentage, LengthPercentageAuto, CompactLength};
/// Taffy geometry types: Size(width,height), Rect(left,right,top,bottom)
use taffy::geometry::{Size, Rect}; 
/// Serde serialization/deserialization for JS <-> Rust data conversion
use serde::{Serialize, Deserialize};
/// wasm-bindgen core macros and types
use wasm_bindgen::prelude::*;
/// Taffy node ID type for uniquely identifying nodes in the layout tree
use taffy::prelude::NodeId;
/// Taffy tree traversal trait providing parent/child node access
use taffy::TraversePartialTree;

// =============================================================================
// External JavaScript Function Declarations
// =============================================================================

/// FFI binding to JavaScript's console.log function for debug output
#[wasm_bindgen]
extern "C" {
    /// Calls JavaScript's console.log to output debug messages
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// =============================================================================
// Enum Type Definitions
// =============================================================================
//
// The following enums are WASM-friendly representations of CSS layout properties.
// Each enum implements bidirectional conversion with native Taffy types (From trait).
// =============================================================================

/// Display mode enum
/// 
/// Controls the layout algorithm type for an element. Corresponds to CSS `display` property.
/// 
/// # Variants
/// - `Block`: Block-level layout, element takes full width
/// - `Flex`: Flexbox layout, one-dimensional layout model
/// - `Grid`: CSS Grid layout, two-dimensional layout model  
/// - `None`: Hidden, element does not participate in layout calculation
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Display { Block = 0, Flex = 1, Grid = 2, None = 3 }
impl From<Display> for taffy::style::Display {
    fn from(val: Display) -> Self {
        match val {
            Display::Block => taffy::style::Display::Block,
            Display::Flex => taffy::style::Display::Flex,
            Display::Grid => taffy::style::Display::Grid,
            Display::None => taffy::style::Display::None,
        }
    }
}
impl From<taffy::style::Display> for Display {
    fn from(val: taffy::style::Display) -> Self {
        match val {
            taffy::style::Display::Block => Display::Block,
            taffy::style::Display::Flex => Display::Flex,
            taffy::style::Display::Grid => Display::Grid,
            taffy::style::Display::None => Display::None,
        }
    }
}

/// Position mode enum
/// 
/// Controls element positioning method. Corresponds to CSS `position` property.
/// 
/// # Variants
/// - `Relative`: Relative positioning, element stays in normal document flow
/// - `Absolute`: Absolute positioning, element removed from flow, positioned relative to nearest positioned ancestor
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Position { Relative = 0, Absolute = 1 }
impl From<Position> for taffy::style::Position {
    fn from(val: Position) -> Self { match val { Position::Relative => taffy::style::Position::Relative, Position::Absolute => taffy::style::Position::Absolute } }
}
impl From<taffy::style::Position> for Position {
    fn from(val: taffy::style::Position) -> Self { match val { taffy::style::Position::Relative => Position::Relative, taffy::style::Position::Absolute => Position::Absolute } }
}

/// Flex main axis direction enum
/// 
/// Defines the direction children are laid out in a flex container. Corresponds to CSS `flex-direction` property.
/// 
/// # Variants
/// - `Row`: Horizontal direction, left to right (LTR mode)
/// - `Column`: Vertical direction, top to bottom
/// - `RowReverse`: Horizontal reverse, right to left
/// - `ColumnReverse`: Vertical reverse, bottom to top
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum FlexDirection { Row = 0, Column = 1, RowReverse = 2, ColumnReverse = 3 }
impl From<FlexDirection> for taffy::style::FlexDirection {
    fn from(val: FlexDirection) -> Self {
        match val { FlexDirection::Row => taffy::style::FlexDirection::Row, FlexDirection::Column => taffy::style::FlexDirection::Column, FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse, FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse }
    }
}
impl From<taffy::style::FlexDirection> for FlexDirection {
    fn from(val: taffy::style::FlexDirection) -> Self {
        match val { taffy::style::FlexDirection::Row => FlexDirection::Row, taffy::style::FlexDirection::Column => FlexDirection::Column, taffy::style::FlexDirection::RowReverse => FlexDirection::RowReverse, taffy::style::FlexDirection::ColumnReverse => FlexDirection::ColumnReverse }
    }
}

/// Flex wrap mode enum
/// 
/// Controls whether flex items wrap onto multiple lines. Corresponds to CSS `flex-wrap` property.
/// 
/// # Variants
/// - `NoWrap`: No wrapping, all items compressed into single line/column
/// - `Wrap`: Automatic wrapping, items flow to next line when container overflows
/// - `WrapReverse`: Reverse wrapping, new lines appear above or to the left
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum FlexWrap { NoWrap = 0, Wrap = 1, WrapReverse = 2 }
impl From<FlexWrap> for taffy::style::FlexWrap {
    fn from(val: FlexWrap) -> Self { match val { FlexWrap::NoWrap => taffy::style::FlexWrap::NoWrap, FlexWrap::Wrap => taffy::style::FlexWrap::Wrap, FlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse } }
}
impl From<taffy::style::FlexWrap> for FlexWrap {
    fn from(val: taffy::style::FlexWrap) -> Self { match val { taffy::style::FlexWrap::NoWrap => FlexWrap::NoWrap, taffy::style::FlexWrap::Wrap => FlexWrap::Wrap, taffy::style::FlexWrap::WrapReverse => FlexWrap::WrapReverse } }
}

/// Cross-axis alignment enum for children (Align Items)
/// 
/// Defines how all children are aligned on the cross axis in a Flex/Grid container.
/// 
/// # Variants
/// - `Start/FlexStart`: Align to cross axis start
/// - `End/FlexEnd`: Align to cross axis end
/// - `Center`: Center alignment
/// - `Baseline`: Baseline alignment (text baseline)
/// - `Stretch`: Stretch to fill container
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum AlignItems { Start = 0, End = 1, FlexStart = 2, FlexEnd = 3, Center = 4, Baseline = 5, Stretch = 6 }
impl From<AlignItems> for taffy::style::AlignItems {
    fn from(val: AlignItems) -> Self { match val { AlignItems::Start => taffy::style::AlignItems::Start, AlignItems::End => taffy::style::AlignItems::End, AlignItems::FlexStart => taffy::style::AlignItems::FlexStart, AlignItems::FlexEnd => taffy::style::AlignItems::FlexEnd, AlignItems::Center => taffy::style::AlignItems::Center, AlignItems::Baseline => taffy::style::AlignItems::Baseline, AlignItems::Stretch => taffy::style::AlignItems::Stretch } }
}
impl From<taffy::style::AlignItems> for AlignItems {
    fn from(val: taffy::style::AlignItems) -> Self { match val { taffy::style::AlignItems::Start => AlignItems::Start, taffy::style::AlignItems::End => AlignItems::End, taffy::style::AlignItems::FlexStart => AlignItems::FlexStart, taffy::style::AlignItems::FlexEnd => AlignItems::FlexEnd, taffy::style::AlignItems::Center => AlignItems::Center, taffy::style::AlignItems::Baseline => AlignItems::Baseline, taffy::style::AlignItems::Stretch => AlignItems::Stretch } }
}

/// Cross-axis alignment enum for single element (Align Self)
/// 
/// Overrides parent's `align-items` for a single child element's cross-axis alignment.
/// 
/// # Variants
/// - `Auto`: Inherit parent's `align-items` value
/// - Other values have same meaning as `AlignItems`
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum AlignSelf { Auto = 0, Start = 1, End = 2, FlexStart = 3, FlexEnd = 4, Center = 5, Baseline = 6, Stretch = 7 }
impl From<AlignSelf> for taffy::style::AlignSelf {
    fn from(val: AlignSelf) -> Self { match val { AlignSelf::Auto => taffy::style::AlignSelf::Stretch, AlignSelf::Start => taffy::style::AlignSelf::Start, AlignSelf::End => taffy::style::AlignSelf::End, AlignSelf::FlexStart => taffy::style::AlignSelf::FlexStart, AlignSelf::FlexEnd => taffy::style::AlignSelf::FlexEnd, AlignSelf::Center => taffy::style::AlignSelf::Center, AlignSelf::Baseline => taffy::style::AlignSelf::Baseline, AlignSelf::Stretch => taffy::style::AlignSelf::Stretch } }
}
impl From<taffy::style::AlignSelf> for AlignSelf {
    fn from(val: taffy::style::AlignSelf) -> Self { match val { taffy::style::AlignSelf::Start => AlignSelf::Start, taffy::style::AlignSelf::End => AlignSelf::End, taffy::style::AlignSelf::FlexStart => AlignSelf::FlexStart, taffy::style::AlignSelf::FlexEnd => AlignSelf::FlexEnd, taffy::style::AlignSelf::Center => AlignSelf::Center, taffy::style::AlignSelf::Baseline => AlignSelf::Baseline, taffy::style::AlignSelf::Stretch => AlignSelf::Stretch } }
}

/// Multi-line content alignment enum (Align Content)
/// 
/// Controls spacing distribution between lines in a multi-line flex container.
/// Corresponds to CSS `align-content` property. Only effective when `flex-wrap: wrap`.
/// 
/// # Variants
/// - `SpaceBetween`: Lines evenly distributed, first/last lines flush with edges
/// - `SpaceAround`: Equal space on both sides of each line
/// - `SpaceEvenly`: All spacing completely equal
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum AlignContent { Start = 0, End = 1, FlexStart = 2, FlexEnd = 3, Center = 4, Stretch = 5, SpaceBetween = 6, SpaceAround = 7, SpaceEvenly = 8 }
impl From<AlignContent> for taffy::style::AlignContent {
    fn from(val: AlignContent) -> Self { match val { AlignContent::Start => taffy::style::AlignContent::Start, AlignContent::End => taffy::style::AlignContent::End, AlignContent::FlexStart => taffy::style::AlignContent::FlexStart, AlignContent::FlexEnd => taffy::style::AlignContent::FlexEnd, AlignContent::Center => taffy::style::AlignContent::Center, AlignContent::Stretch => taffy::style::AlignContent::Stretch, AlignContent::SpaceBetween => taffy::style::AlignContent::SpaceBetween, AlignContent::SpaceAround => taffy::style::AlignContent::SpaceAround, AlignContent::SpaceEvenly => taffy::style::AlignContent::SpaceEvenly } }
}
impl From<taffy::style::AlignContent> for AlignContent {
    fn from(val: taffy::style::AlignContent) -> Self { match val { taffy::style::AlignContent::Start => AlignContent::Start, taffy::style::AlignContent::End => AlignContent::End, taffy::style::AlignContent::FlexStart => AlignContent::FlexStart, taffy::style::AlignContent::FlexEnd => AlignContent::FlexEnd, taffy::style::AlignContent::Center => AlignContent::Center, taffy::style::AlignContent::Stretch => AlignContent::Stretch, taffy::style::AlignContent::SpaceBetween => AlignContent::SpaceBetween, taffy::style::AlignContent::SpaceAround => AlignContent::SpaceAround, taffy::style::AlignContent::SpaceEvenly => AlignContent::SpaceEvenly } }
}

/// Main axis alignment enum (Justify Content)
/// 
/// Defines alignment and spacing distribution of children along the main axis.
/// Corresponds to CSS `justify-content` property.
/// 
/// # Variants
/// - `Start/FlexStart`: Align to main axis start
/// - `End/FlexEnd`: Align to main axis end
/// - `Center`: Center alignment
/// - `SpaceBetween`: First and last items flush with edges, remaining space distributed evenly
/// - `SpaceAround`: Equal space on both sides of each item
/// - `SpaceEvenly`: All spacing completely equal
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum JustifyContent { Start = 0, End = 1, FlexStart = 2, FlexEnd = 3, Center = 4, Stretch = 5, SpaceBetween = 6, SpaceAround = 7, SpaceEvenly = 8 }
impl From<JustifyContent> for taffy::style::JustifyContent {
    fn from(val: JustifyContent) -> Self { match val { JustifyContent::Start => taffy::style::JustifyContent::Start, JustifyContent::End => taffy::style::JustifyContent::End, JustifyContent::FlexStart => taffy::style::JustifyContent::FlexStart, JustifyContent::FlexEnd => taffy::style::JustifyContent::FlexEnd, JustifyContent::Center => taffy::style::JustifyContent::Center, JustifyContent::Stretch => taffy::style::JustifyContent::Stretch, JustifyContent::SpaceBetween => taffy::style::JustifyContent::SpaceBetween, JustifyContent::SpaceAround => taffy::style::JustifyContent::SpaceAround, JustifyContent::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly } }
}
impl From<taffy::style::JustifyContent> for JustifyContent {
    fn from(val: taffy::style::JustifyContent) -> Self { match val { taffy::style::JustifyContent::Start => JustifyContent::Start, taffy::style::JustifyContent::End => JustifyContent::End, taffy::style::JustifyContent::FlexStart => JustifyContent::FlexStart, taffy::style::JustifyContent::FlexEnd => JustifyContent::FlexEnd, taffy::style::JustifyContent::Center => JustifyContent::Center, taffy::style::JustifyContent::Stretch => JustifyContent::Stretch, taffy::style::JustifyContent::SpaceBetween => JustifyContent::SpaceBetween, taffy::style::JustifyContent::SpaceAround => JustifyContent::SpaceAround, taffy::style::JustifyContent::SpaceEvenly => JustifyContent::SpaceEvenly } }
}

/// Overflow handling enum
/// 
/// Defines how content that overflows container boundaries is handled.
/// Corresponds to CSS `overflow` property.
/// 
/// # Variants
/// - `Visible`: Content is not clipped
/// - `Hidden`: Content is clipped, overflow hidden
/// - `Scroll`: Always show scrollbars
/// - `Auto`: Show scrollbars when needed (internally mapped to Scroll)
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Overflow { Visible = 0, Hidden = 1, Scroll = 2, Auto = 3 }
impl From<Overflow> for taffy::style::Overflow {
    fn from(val: Overflow) -> Self { match val { Overflow::Visible => taffy::style::Overflow::Visible, Overflow::Hidden => taffy::style::Overflow::Hidden, Overflow::Scroll => taffy::style::Overflow::Scroll, Overflow::Auto => taffy::style::Overflow::Scroll } }
}
impl From<taffy::style::Overflow> for Overflow {
    fn from(val: taffy::style::Overflow) -> Self { match val { taffy::style::Overflow::Visible => Overflow::Visible, taffy::style::Overflow::Hidden => Overflow::Hidden, taffy::style::Overflow::Scroll => Overflow::Scroll, taffy::style::Overflow::Clip => Overflow::Hidden } }
}

/// Box sizing enum
/// 
/// Controls how the total width and height of an element is calculated.
/// 
/// # Variants
/// - `BorderBox`: Width and height include content, padding, and border (default)
/// - `ContentBox`: Width and height include only the content
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BoxSizing { BorderBox = 0, ContentBox = 1 }
impl From<BoxSizing> for taffy::style::BoxSizing {
    fn from(val: BoxSizing) -> Self { match val { BoxSizing::BorderBox => taffy::style::BoxSizing::BorderBox, BoxSizing::ContentBox => taffy::style::BoxSizing::ContentBox } }
}
impl From<taffy::style::BoxSizing> for BoxSizing {
    fn from(val: taffy::style::BoxSizing) -> Self { match val { taffy::style::BoxSizing::BorderBox => BoxSizing::BorderBox, taffy::style::BoxSizing::ContentBox => BoxSizing::ContentBox } }
}

// =============================================================================
// Layout Output Type
// =============================================================================
//
// Layout represents the computed layout result for a node after running the
// layout algorithm. It wraps the native taffy::Layout struct.
// =============================================================================

/// Layout result struct
///
/// A wrapper around `taffy::Layout` that provides WASM bindings.
/// Contains the computed layout values for a node after calling `computeLayout()`.
/// All values are in pixels.
///
/// # Properties
/// - `order`: Rendering order (higher = on top)
/// - `x`, `y`: Position of top-left corner relative to parent
/// - `width`, `height`: Computed dimensions
/// - `contentWidth`, `contentHeight`: Size of scrollable content
/// - `scrollbarWidth`, `scrollbarHeight`: Size allocated for scrollbars
/// - `borderLeft`, `borderRight`, `borderTop`, `borderBottom`: Border widths
/// - `paddingLeft`, `paddingRight`, `paddingTop`, `paddingBottom`: Padding sizes
/// - `marginLeft`, `marginRight`, `marginTop`, `marginBottom`: Margin sizes
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Layout {
    /// The inner taffy::Layout object
    inner: taffy::Layout,
}

#[wasm_bindgen]
impl Layout {
    /// Gets the rendering order of the node.
    #[wasm_bindgen(getter)]
    pub fn order(&self) -> u32 { self.inner.order }
    
    /// Gets the x coordinate of the node's top-left corner.
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f32 { self.inner.location.x }
    
    /// Gets the y coordinate of the node's top-left corner.
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f32 { self.inner.location.y }
    
    /// Gets the computed width of the node.
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f32 { self.inner.size.width }
    
    /// Gets the computed height of the node.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f32 { self.inner.size.height }
    
    /// Gets the width of the scrollable content.
    #[wasm_bindgen(getter, js_name = contentWidth)]
    pub fn content_width(&self) -> f32 { self.inner.content_size.width }
    
    /// Gets the height of the scrollable content.
    #[wasm_bindgen(getter, js_name = contentHeight)]
    pub fn content_height(&self) -> f32 { self.inner.content_size.height }
    
    /// Gets the width of the vertical scrollbar.
    #[wasm_bindgen(getter, js_name = scrollbarWidth)]
    pub fn scrollbar_width(&self) -> f32 { self.inner.scrollbar_size.width }
    
    /// Gets the height of the horizontal scrollbar.
    #[wasm_bindgen(getter, js_name = scrollbarHeight)]
    pub fn scrollbar_height(&self) -> f32 { self.inner.scrollbar_size.height }
    
    /// Gets the left border width.
    #[wasm_bindgen(getter, js_name = borderLeft)]
    pub fn border_left(&self) -> f32 { self.inner.border.left }
    
    /// Gets the right border width.
    #[wasm_bindgen(getter, js_name = borderRight)]
    pub fn border_right(&self) -> f32 { self.inner.border.right }
    
    /// Gets the top border width.
    #[wasm_bindgen(getter, js_name = borderTop)]
    pub fn border_top(&self) -> f32 { self.inner.border.top }
    
    /// Gets the bottom border width.
    #[wasm_bindgen(getter, js_name = borderBottom)]
    pub fn border_bottom(&self) -> f32 { self.inner.border.bottom }
    
    /// Gets the left padding.
    #[wasm_bindgen(getter, js_name = paddingLeft)]
    pub fn padding_left(&self) -> f32 { self.inner.padding.left }
    
    /// Gets the right padding.
    #[wasm_bindgen(getter, js_name = paddingRight)]
    pub fn padding_right(&self) -> f32 { self.inner.padding.right }
    
    /// Gets the top padding.
    #[wasm_bindgen(getter, js_name = paddingTop)]
    pub fn padding_top(&self) -> f32 { self.inner.padding.top }
    
    /// Gets the bottom padding.
    #[wasm_bindgen(getter, js_name = paddingBottom)]
    pub fn padding_bottom(&self) -> f32 { self.inner.padding.bottom }
    
    /// Gets the left margin.
    #[wasm_bindgen(getter, js_name = marginLeft)]
    pub fn margin_left(&self) -> f32 { self.inner.margin.left }
    
    /// Gets the right margin.
    #[wasm_bindgen(getter, js_name = marginRight)]
    pub fn margin_right(&self) -> f32 { self.inner.margin.right }
    
    /// Gets the top margin.
    #[wasm_bindgen(getter, js_name = marginTop)]
    pub fn margin_top(&self) -> f32 { self.inner.margin.top }
    
    /// Gets the bottom margin.
    #[wasm_bindgen(getter, js_name = marginBottom)]
    pub fn margin_bottom(&self) -> f32 { self.inner.margin.bottom }
}

impl From<&taffy::Layout> for Layout {
    fn from(layout: &taffy::Layout) -> Self {
        Layout { inner: layout.clone() }
    }
}

impl From<taffy::Layout> for Layout {
    fn from(layout: taffy::Layout) -> Self {
        Layout { inner: layout }
    }
}

// =============================================================================
// Data Transfer Objects (DTOs)
// =============================================================================
//
// The following types are used for data serialization/deserialization between
// JavaScript and Rust. They are WASM-friendly wrappers for Taffy internal types
// with serde serialization support.
// =============================================================================

/// Dimension DTO (Data Transfer Object)
/// 
/// Used for transferring dimension values between JS and Rust.
/// Supports pixels, percentages, and auto modes.
/// 
/// # Variants
/// - `Length(f32)`: Fixed pixel value, e.g., `100.0` represents 100px
/// - `Percent(f32)`: Percentage value, e.g., `0.5` represents 50%
/// - `Auto`: Automatic calculation, determined by layout algorithm
#[derive(Deserialize, Serialize)]
pub enum JsDimension {
    Length(f32),
    Percent(f32),
    Auto,
}
impl From<JsDimension> for Dimension {
    fn from(v: JsDimension) -> Self {
        match v {
            JsDimension::Length(f) => Dimension::length(f),
            JsDimension::Percent(f) => Dimension::percent(f),
            JsDimension::Auto => Dimension::auto(),
        }
    }
}
impl From<Dimension> for JsDimension {
    fn from(d: Dimension) -> Self {
        if d.is_auto() {
            JsDimension::Auto
        } else {
            // Use into_raw() to access CompactLength
            match d.into_raw().tag() {
                CompactLength::LENGTH_TAG => JsDimension::Length(d.value()),
                CompactLength::PERCENT_TAG => JsDimension::Percent(d.value()),
                _ => JsDimension::Auto,
            }
        }
    }
}

/// Length or Percentage DTO
/// 
/// Used for properties that don't support `auto`, such as padding and border.
/// 
/// # Variants
/// - `Length(f32)`: Fixed pixel value
/// - `Percent(f32)`: Percentage value
#[derive(Deserialize, Serialize)]
pub enum JsLengthPercentage {
    Length(f32),
    Percent(f32),
}
impl From<JsLengthPercentage> for LengthPercentage {
    fn from(v: JsLengthPercentage) -> Self {
        match v {
            JsLengthPercentage::Length(f) => LengthPercentage::length(f),
            JsLengthPercentage::Percent(f) => LengthPercentage::percent(f),
        }
    }
}
impl From<LengthPercentage> for JsLengthPercentage {
    fn from(val: LengthPercentage) -> Self {
        // Use into_raw()
        let inner = val.into_raw();
        match inner.tag() {
             CompactLength::LENGTH_TAG => JsLengthPercentage::Length(inner.value()),
             CompactLength::PERCENT_TAG => JsLengthPercentage::Percent(inner.value()),
             _ => JsLengthPercentage::Length(0.0), 
        }
    }
}

/// Length/Percentage/Auto DTO
/// 
/// Used for properties that support `auto`, such as margin and inset.
/// 
/// # Variants
/// - `Length(f32)`: Fixed pixel value
/// - `Percent(f32)`: Percentage value
/// - `Auto`: Automatic calculation
#[derive(Deserialize, Serialize)]
pub enum JsLengthPercentageAuto {
    Length(f32),
    Percent(f32),
    Auto,
}
impl From<JsLengthPercentageAuto> for LengthPercentageAuto {
    fn from(v: JsLengthPercentageAuto) -> Self {
        match v {
            JsLengthPercentageAuto::Length(f) => LengthPercentageAuto::length(f),
            JsLengthPercentageAuto::Percent(f) => LengthPercentageAuto::percent(f),
            JsLengthPercentageAuto::Auto => LengthPercentageAuto::auto(),
        }
    }
}
impl From<LengthPercentageAuto> for JsLengthPercentageAuto {
    fn from(val: LengthPercentageAuto) -> Self {
        let inner = val.into_raw();
        if inner.is_auto() {
            JsLengthPercentageAuto::Auto
        } else {
            match inner.tag() {
                CompactLength::LENGTH_TAG => JsLengthPercentageAuto::Length(inner.value()),
                CompactLength::PERCENT_TAG => JsLengthPercentageAuto::Percent(inner.value()),
                _ => JsLengthPercentageAuto::Auto,
            }
        }
    }
}

/// Two-dimensional Size DTO
///
/// Generic struct for transferring width and height property pairs.
/// Commonly used for size, min_size, max_size, gap, etc.
#[derive(Deserialize, Serialize)]
pub struct JsSize<T> {
    pub width: T,
    pub height: T,
}
impl<T, U> From<JsSize<T>> for Size<U> where T: Into<U>, U: Copy {
    fn from(v: JsSize<T>) -> Self {
        Size { width: v.width.into(), height: v.height.into() }
    }
}

/// Four-sided Rectangle DTO
///
/// Generic struct for transferring edge properties (margin, padding, border, inset).
#[derive(Deserialize, Serialize)]
pub struct JsRect<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}
impl<T, U> From<JsRect<T>> for Rect<U> where T: Into<U>, U: Copy {
    fn from(v: JsRect<T>) -> Self {
        Rect { left: v.left.into(), right: v.right.into(), top: v.top.into(), bottom: v.bottom.into() }
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Serializes a Rust value to JsValue
/// 
/// # Arguments
/// - `val`: Any value implementing the Serialize trait
/// 
/// # Returns
/// - Success: Serialized JsValue
/// - Failure: JsValue::NULL
fn serialize<T: Serialize + ?Sized>(val: &T) -> JsValue {
    serde_wasm_bindgen::to_value(val).unwrap_or(JsValue::NULL)
}

/// Available Space Size DTO
///
/// Used for transferring available space constraints during layout computation.
#[derive(serde::Deserialize)]
pub struct JsAvailableSize {
    pub width: JsAvailableSpace,
    pub height: JsAvailableSpace,
}

/// Available Space enum
///
/// Defines the space constraint for a single dimension during layout computation.
/// 
/// # Variants
/// - `Definite(f32)`: A specific pixel value
/// - `MinContent`: Minimum content width/height
/// - `MaxContent`: Maximum content width/height
#[derive(serde::Deserialize)]
pub enum JsAvailableSpace {
    Definite(f32),
    MinContent,
    MaxContent,
}
impl From<JsAvailableSize> for Size<AvailableSpace> {
    fn from(s: JsAvailableSize) -> Self {
        Size {
            width: s.width.into(),
            height: s.height.into(),
        }
    }
}
impl From<JsAvailableSpace> for AvailableSpace {
    fn from(s: JsAvailableSpace) -> Self {
        match s {
            JsAvailableSpace::Definite(v) => AvailableSpace::Definite(v),
            JsAvailableSpace::MinContent => AvailableSpace::MinContent,
            JsAvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
}

// =============================================================================
// Style Struct
// =============================================================================
//
// Style is a wrapper for node style configuration. It encapsulates Taffy's native
// Style and provides a JavaScript-friendly getter/setter interface.
// =============================================================================

/// Node Style struct
///
/// Configuration object containing all CSS layout properties.
/// Access properties via getter/setter methods.
///
/// # Supported Property Categories
/// 
/// ## Layout Mode
/// - `display`: Display mode (Flex/Grid/Block/None)
/// - `position`: Position mode (Relative/Absolute)
/// 
/// ## Flexbox Properties
/// - `flex_direction`: Main axis direction
/// - `flex_wrap`: Wrap behavior
/// - `flex_grow`: Grow factor
/// - `flex_shrink`: Shrink factor
/// - `flex_basis`: Initial size
/// 
/// ## Alignment Properties
/// - `align_items`, `align_self`, `align_content`
/// - `justify_content`
/// 
/// ## Sizing Properties
/// - `size`, `min_size`, `max_size`
/// - `aspect_ratio`: Width-to-height ratio
/// 
/// ## Spacing Properties
/// - `margin`, `padding`, `border`
/// - `gap`: Gap between children
/// - `inset`: Absolute positioning offsets
#[wasm_bindgen]
pub struct Style {
    /// Internal Taffy style object (crate-internal access)
    pub(crate) inner: TaffyStyle,
}

#[wasm_bindgen]
impl Style {
    // =========================================================================
    // Constructor
    // =========================================================================
    
    /// Creates a new Style instance with default values.
    /// 
    /// All properties are initialized to their CSS default values:
    /// - display: Block
    /// - position: Relative
    /// - flex_direction: Row
    /// - All dimensions: Auto
    /// - All spacing (margin, padding, border): 0
    /// 
    /// # Returns
    /// A new Style instance with default configuration.
    /// 
    /// # Example
    /// ```javascript
    /// const style = new Style();
    /// style.display = Display.Flex;
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> Style {
        Style { inner: TaffyStyle::default() }
    }
    
    // =========================================================================
    // Layout Mode Properties
    // =========================================================================
    
    /// Gets the display mode (Block, Flex, Grid, or None).
    #[wasm_bindgen(getter)] 
    pub fn display(&self) -> Display { self.inner.display.into() }
    
    /// Sets the display mode. Controls which layout algorithm is used for children.
    /// - `Display.Block`: Block layout
    /// - `Display.Flex`: Flexbox layout
    /// - `Display.Grid`: CSS Grid layout
    /// - `Display.None`: Element is hidden and takes no space
    #[wasm_bindgen(setter)] 
    pub fn set_display(&mut self, val: Display) { self.inner.display = val.into(); }

    /// Gets the position mode (Relative or Absolute).
    #[wasm_bindgen(getter)] 
    pub fn position(&self) -> Position { self.inner.position.into() }
    
    /// Sets the position mode.
    /// - `Position.Relative`: Normal document flow
    /// - `Position.Absolute`: Removed from flow, positioned via inset properties
    #[wasm_bindgen(setter)] 
    pub fn set_position(&mut self, val: Position) { self.inner.position = val.into(); }

    // =========================================================================
    // Flexbox Properties
    // =========================================================================
    
    /// Gets the flex direction (Row, Column, RowReverse, ColumnReverse).
    #[wasm_bindgen(getter)] 
    pub fn flex_direction(&self) -> FlexDirection { self.inner.flex_direction.into() }
    
    /// Sets the flex direction. Defines the main axis for flex item layout.
    #[wasm_bindgen(setter)] 
    pub fn set_flex_direction(&mut self, val: FlexDirection) { self.inner.flex_direction = val.into(); }
    
    /// Gets the flex wrap mode (NoWrap, Wrap, WrapReverse).
    #[wasm_bindgen(getter)] 
    pub fn flex_wrap(&self) -> FlexWrap { self.inner.flex_wrap.into() }
    
    /// Sets the flex wrap mode. Controls whether items wrap to new lines.
    #[wasm_bindgen(setter)] 
    pub fn set_flex_wrap(&mut self, val: FlexWrap) { self.inner.flex_wrap = val.into(); }
    
    /// Gets the flex grow factor. Determines how much the item grows relative to siblings.
    #[wasm_bindgen(getter)] 
    pub fn flex_grow(&self) -> f32 { self.inner.flex_grow }
    
    /// Sets the flex grow factor. A value of 0 means the item won't grow.
    /// Higher values mean more growth relative to other items.
    #[wasm_bindgen(setter)] 
    pub fn set_flex_grow(&mut self, val: f32) { self.inner.flex_grow = val; }

    /// Gets the flex shrink factor. Determines how much the item shrinks relative to siblings.
    #[wasm_bindgen(getter)] 
    pub fn flex_shrink(&self) -> f32 { self.inner.flex_shrink }
    
    /// Sets the flex shrink factor. A value of 0 prevents shrinking.
    /// Default is 1.0 for flex items.
    #[wasm_bindgen(setter)] 
    pub fn set_flex_shrink(&mut self, val: f32) { self.inner.flex_shrink = val; }

    // =========================================================================
    // Alignment Properties
    // =========================================================================
    
    /// Gets the align-items property. Controls cross-axis alignment of children.
    #[wasm_bindgen(getter)] 
    pub fn align_items(&self) -> Option<AlignItems> { self.inner.align_items.map(AlignItems::from) }
    
    /// Sets the align-items property. Affects all children's cross-axis alignment.
    #[wasm_bindgen(setter)] 
    pub fn set_align_items(&mut self, val: Option<AlignItems>) { self.inner.align_items = val.map(taffy::style::AlignItems::from); }

    /// Gets the align-self property. Overrides parent's align-items for this element.
    /// Returns AlignSelf.Auto if not explicitly set.
    #[wasm_bindgen(getter)] 
    pub fn align_self(&self) -> Option<AlignSelf> { match self.inner.align_self { Some(v) => Some(AlignSelf::from(v)), None => Some(AlignSelf::Auto) } }
    
    /// Sets the align-self property. Use AlignSelf.Auto to inherit from parent.
    #[wasm_bindgen(setter)] 
    pub fn set_align_self(&mut self, val: Option<AlignSelf>) { self.inner.align_self = match val { Some(AlignSelf::Auto) => None, Some(other) => Some(taffy::style::AlignSelf::from(other)), None => None }; }

    /// Gets the align-content property. Controls spacing between lines in multi-line flex.
    #[wasm_bindgen(getter)] 
    pub fn align_content(&self) -> Option<AlignContent> { self.inner.align_content.map(AlignContent::from) }
    
    /// Sets the align-content property. Only effective when flex-wrap is enabled.
    #[wasm_bindgen(setter)] 
    pub fn set_align_content(&mut self, val: Option<AlignContent>) { self.inner.align_content = val.map(taffy::style::AlignContent::from); }

    /// Gets the justify-content property. Controls main-axis alignment and spacing.
    #[wasm_bindgen(getter)] 
    pub fn justify_content(&self) -> Option<JustifyContent> { self.inner.justify_content.map(JustifyContent::from) }
    
    /// Sets the justify-content property. Distributes space along the main axis.
    #[wasm_bindgen(setter)] 
    pub fn set_justify_content(&mut self, val: Option<JustifyContent>) { self.inner.justify_content = val.map(taffy::style::JustifyContent::from); }

    // =========================================================================
    // Sizing Properties
    // =========================================================================
    
    /// Gets the aspect ratio (width / height). Returns None if not set.
    #[wasm_bindgen(getter)] 
    pub fn aspect_ratio(&self) -> Option<f32> { self.inner.aspect_ratio }
    
    /// Sets the aspect ratio. For example, 16/9 = 1.777... for widescreen.
    /// Set to None to remove the constraint.
    #[wasm_bindgen(setter)] 
    pub fn set_aspect_ratio(&mut self, val: Option<f32>) { self.inner.aspect_ratio = val; }

    /// Gets the overflow behavior as a JS object with {x, y} properties.
    #[wasm_bindgen(getter)] 
    pub fn overflow(&self) -> JsValue { serialize(&self.inner.overflow) }
    
    /// Sets the overflow behavior. Accepts {x: Overflow, y: Overflow}.
    #[wasm_bindgen(setter)] 
    pub fn set_overflow(&mut self, val: JsValue) { if let Ok(o) = serde_wasm_bindgen::from_value(val) { self.inner.overflow = o; } }

    /// Gets the box sizing mode (BorderBox or ContentBox).
    #[wasm_bindgen(getter)]
    pub fn box_sizing(&self) -> BoxSizing { self.inner.box_sizing.into() }

    /// Sets the box sizing mode.
    /// - `BoxSizing.BorderBox`: Width and height include content, padding, and border (default)
    /// - `BoxSizing.ContentBox`: Width and height include only the content
    #[wasm_bindgen(setter)]
    pub fn set_box_sizing(&mut self, val: BoxSizing) { self.inner.box_sizing = val.into(); }

    /// Gets the flex-basis as a JsDimension (Length, Percent, or Auto).
    /// Flex-basis defines the initial main size before grow/shrink.
    #[wasm_bindgen(getter)] 
    pub fn flex_basis(&self) -> JsValue { 
        let d: JsDimension = self.inner.flex_basis.into();
        serialize(&d) 
    }
    
    /// Sets the flex-basis. Accepts { Length: number } | { Percent: number } | "Auto".
    #[wasm_bindgen(setter)]
    pub fn set_flex_basis(&mut self, val: JsValue) { 
        if let Ok(d) = serde_wasm_bindgen::from_value::<JsDimension>(val) { self.inner.flex_basis = d.into(); } 
    }

    // =========================================================================
    // Dimension Properties
    // =========================================================================
    
    /// Gets the size (width, height) as a JsSize<JsDimension>.
    /// Each dimension can be Length, Percent, or Auto.
    #[wasm_bindgen(getter)] 
    pub fn size(&self) -> JsValue { 
        let s: JsSize<JsDimension> = JsSize { width: self.inner.size.width.into(), height: self.inner.size.height.into() };
        serialize(&s) 
    }
    
    /// Sets the size (width, height).
    /// Accepts { width: Dimension, height: Dimension } where Dimension is Length/Percent/Auto.
    /// Logs an error to console if parsing fails.
    #[wasm_bindgen(setter)]
    pub fn set_size(&mut self, val: JsValue) { 
        match serde_wasm_bindgen::from_value::<JsSize<JsDimension>>(val.clone()) {
            Ok(s) => { self.inner.size = s.into(); }
            Err(e) => { 
                let json = js_sys::JSON::stringify(&val).ok().and_then(|s| s.as_string()).unwrap_or("?".to_string());
                log(&format!("set_size Error: {} | Input: {}", e, json)); 
            }
        }
    }

    /// Gets the minimum size constraints as a JsSize<JsDimension>.
    /// Prevents the element from shrinking below these values.
    #[wasm_bindgen(getter)] 
    pub fn min_size(&self) -> JsValue { 
        let s: JsSize<JsDimension> = JsSize { width: self.inner.min_size.width.into(), height: self.inner.min_size.height.into() };
        serialize(&s)
    }
    
    /// Sets the minimum size constraints.
    #[wasm_bindgen(setter)]
    pub fn set_min_size(&mut self, val: JsValue) { 
        if let Ok(s) = serde_wasm_bindgen::from_value::<JsSize<JsDimension>>(val) { self.inner.min_size = s.into(); }
    }

    /// Gets the maximum size constraints as a JsSize<JsDimension>.
    /// Prevents the element from growing beyond these values.
    #[wasm_bindgen(getter)] 
    pub fn max_size(&self) -> JsValue { 
        let s: JsSize<JsDimension> = JsSize { width: self.inner.max_size.width.into(), height: self.inner.max_size.height.into() };
        serialize(&s)
    }
    
    /// Sets the maximum size constraints.
    #[wasm_bindgen(setter)]
    pub fn set_max_size(&mut self, val: JsValue) { 
        if let Ok(s) = serde_wasm_bindgen::from_value::<JsSize<JsDimension>>(val) { self.inner.max_size = s.into(); }
    }
    
    // =========================================================================
    // Spacing Properties
    // =========================================================================
    
    /// Gets the margin as a JsRect<JsLengthPercentageAuto>.
    /// Margin is the outer spacing around the element's border.
    /// Supports Length, Percent, or Auto for each edge.
    #[wasm_bindgen(getter)] 
    pub fn margin(&self) -> JsValue { 
        let m: JsRect<JsLengthPercentageAuto> = JsRect { 
            left: self.inner.margin.left.into(), right: self.inner.margin.right.into(), 
            top: self.inner.margin.top.into(), bottom: self.inner.margin.bottom.into() 
        };
        serialize(&m) 
    }
    
    /// Sets the margin for all four edges.
    /// Accepts { left, right, top, bottom } with LengthPercentageAuto values.
    #[wasm_bindgen(setter)] 
    pub fn set_margin(&mut self, val: JsValue) {
        if let Ok(m) = serde_wasm_bindgen::from_value::<JsRect<JsLengthPercentageAuto>>(val) { self.inner.margin = m.into(); }
    }

    /// Gets the padding as a JsRect<JsLengthPercentage>.
    /// Padding is the inner spacing between the border and content.
    /// Supports Length or Percent for each edge (not Auto).
    #[wasm_bindgen(getter)] 
    pub fn padding(&self) -> JsValue { 
        let m: JsRect<JsLengthPercentage> = JsRect { 
            left: self.inner.padding.left.into(), right: self.inner.padding.right.into(), 
            top: self.inner.padding.top.into(), bottom: self.inner.padding.bottom.into() 
        };
        serialize(&m) 
    }
    
    /// Sets the padding for all four edges.
    /// Accepts { left, right, top, bottom } with LengthPercentage values.
    #[wasm_bindgen(setter)] 
    pub fn set_padding(&mut self, val: JsValue) {
        if let Ok(p) = serde_wasm_bindgen::from_value::<JsRect<JsLengthPercentage>>(val) { self.inner.padding = p.into(); }
    }
    
    /// Gets the border width as a JsRect<JsLengthPercentage>.
    /// Border width defines the thickness of element borders.
    /// Supports Length or Percent for each edge (not Auto).
    #[wasm_bindgen(getter)] 
    pub fn border(&self) -> JsValue { 
        let m: JsRect<JsLengthPercentage> = JsRect { 
            left: self.inner.border.left.into(), right: self.inner.border.right.into(), 
            top: self.inner.border.top.into(), bottom: self.inner.border.bottom.into() 
        };
        serialize(&m) 
    }
    
    /// Sets the border width for all four edges.
    /// Accepts { left, right, top, bottom } with LengthPercentage values.
    #[wasm_bindgen(setter)] 
    pub fn set_border(&mut self, val: JsValue) {
        if let Ok(b) = serde_wasm_bindgen::from_value::<JsRect<JsLengthPercentage>>(val) { self.inner.border = b.into(); }
    }
    
    /// Gets the gap between children as a JsSize<JsLengthPercentage>.
    /// Used in Flex and Grid layouts to add spacing between items.
    /// - width: column gap (horizontal spacing)
    /// - height: row gap (vertical spacing)
    #[wasm_bindgen(getter)] 
    pub fn gap(&self) -> JsValue { 
        let s: JsSize<JsLengthPercentage> = JsSize { width: self.inner.gap.width.into(), height: self.inner.gap.height.into() };
        serialize(&s) 
    }
    
    /// Sets the gap between children.
    /// Accepts { width: column_gap, height: row_gap } with LengthPercentage values.
    #[wasm_bindgen(setter)] 
    pub fn set_gap(&mut self, val: JsValue) {
        if let Ok(g) = serde_wasm_bindgen::from_value::<JsSize<JsLengthPercentage>>(val) { self.inner.gap = g.into(); }
    }
    
    /// Gets the inset (absolute positioning offsets) as a JsRect<JsLengthPercentageAuto>.
    /// Only effective when position is Absolute.
    /// Defines the distance from each edge of the containing block.
    #[wasm_bindgen(getter)] 
    pub fn inset(&self) -> JsValue { 
        let m: JsRect<JsLengthPercentageAuto> = JsRect { 
            left: self.inner.inset.left.into(), right: self.inner.inset.right.into(), 
            top: self.inner.inset.top.into(), bottom: self.inner.inset.bottom.into() 
        };
        serialize(&m) 
    }
    
    /// Sets the inset for absolute positioning.
    /// Accepts { left, right, top, bottom } with LengthPercentageAuto values.
    #[wasm_bindgen(setter)] 
    pub fn set_inset(&mut self, val: JsValue) {
        if let Ok(i) = serde_wasm_bindgen::from_value::<JsRect<JsLengthPercentageAuto>>(val) { self.inner.inset = i.into(); }
    }
}

// =============================================================================
// Layout Tree Manager (TaffyTree)
// =============================================================================
//
// TaffyTree is the main entry point for layout computation. It manages a tree
// of nodes, each with their own style and optional context data. The tree
// supports Flexbox, CSS Grid, and Block layout algorithms.
//
// ## Node Management
// - Nodes are identified by u64 IDs (internally NodeId)
// - Create nodes with new_leaf(), new_leaf_with_context(), new_with_children()
// - Modify tree structure with add_child(), remove_child(), set_children()
//
// ## Layout Computation
// - Call compute_layout() or compute_layout_with_measure() on root node
// - Results are cached until node is marked dirty
// - Retrieve results with get_layout()
// =============================================================================

/// Layout tree manager providing node creation, tree manipulation, and layout computation.
///
/// This is the main entry point for the Taffy layout engine. It wraps the native
/// `taffy::TaffyTree<JsValue>` to provide a JavaScript-friendly API.
///
/// # Features
/// - **Node Management**: Create, remove, and reorganize layout nodes
/// - **Style Control**: Get/set styles for any node
/// - **Layout Computation**: Run Flexbox/Grid/Block layout algorithms
/// - **Custom Measurement**: Support for text measurement via callback functions
/// - **Node Context**: Attach arbitrary JS data to nodes for custom logic
///
/// # Example
/// ```javascript
/// const tree = new TaffyTree();
/// const style = new Style();
/// style.display = Display.Flex;
/// const root = tree.newLeaf(style);
/// tree.computeLayout(root, { width: { Definite: 800 }, height: { Definite: 600 } });
/// const layout = tree.getLayout(root);
/// ```
#[wasm_bindgen]
pub struct TaffyTree { tree: taffy::TaffyTree<JsValue> }
#[wasm_bindgen]
impl TaffyTree {
    // =========================================================================
    // Constructors
    // =========================================================================
    
    /// Creates a new empty TaffyTree.
    /// 
    /// This is the primary constructor. Initializes panic hook in debug builds
    /// to provide better error messages in the browser console.
    /// 
    /// # Returns
    /// A new TaffyTree instance with no nodes.
    #[wasm_bindgen(constructor)] 
    pub fn new() -> TaffyTree { 
        #[cfg(feature = "console_error_panic_hook")] 
        console_error_panic_hook::set_once(); 
        TaffyTree { tree: taffy::TaffyTree::new() } 
    }
    
    /// Creates a new TaffyTree with pre-allocated capacity.
    /// 
    /// Use this when you know approximately how many nodes you'll create
    /// to avoid reallocation overhead.
    /// 
    /// # Arguments
    /// * `capacity` - The number of nodes to pre-allocate space for.
    /// 
    /// # Returns
    /// A new TaffyTree instance with pre-allocated capacity.
    #[wasm_bindgen(js_name = withCapacity)] 
    pub fn with_capacity(capacity: usize) -> TaffyTree { 
        #[cfg(feature = "console_error_panic_hook")] 
        console_error_panic_hook::set_once(); 
        TaffyTree { tree: taffy::TaffyTree::with_capacity(capacity) } 
    }
    
    // =========================================================================
    // Configuration
    // =========================================================================
    
    /// Enables rounding of layout values to whole pixels.
    /// 
    /// When enabled, all computed layout values (x, y, width, height) are
    /// rounded to the nearest integer. This is the default behavior.
    #[wasm_bindgen(js_name = enableRounding)] 
    pub fn enable_rounding(&mut self) { self.tree.enable_rounding(); }
    
    /// Disables rounding of layout values.
    /// 
    /// When disabled, layout values may have fractional pixel values.
    /// Use `unroundedLayout()` to get the pre-rounding values.
    #[wasm_bindgen(js_name = disableRounding)] 
    pub fn disable_rounding(&mut self) { self.tree.disable_rounding(); }
    
    // =========================================================================
    // Node Creation
    // =========================================================================
    
    /// Creates a new leaf node (no children) with the given style.
    /// 
    /// # Arguments
    /// * `style` - The Style object to apply to this node.
    /// 
    /// # Returns
    /// * `Ok(u64)` - The node ID of the newly created node.
    /// * `Err(JsValue)` - Error message if creation fails.
    #[wasm_bindgen(js_name = newLeaf)] 
    pub fn new_leaf(&mut self, style: &Style) -> Result<u64, JsValue> { 
        let id = self.tree.new_leaf(style.inner.clone()).map_err(|e| e.to_string())?; 
        Ok(id.into()) 
    }
    
    /// Creates a new leaf node with an attached context value.
    /// 
    /// The context can be any JavaScript value and is useful for associating
    /// custom data (like text content) with a node for use in measure functions.
    /// 
    /// # Arguments
    /// * `style` - The Style object to apply to this node.
    /// * `context` - Any JavaScript value to attach to this node.
    /// 
    /// # Returns
    /// * `Ok(u64)` - The node ID of the newly created node.
    /// * `Err(JsValue)` - Error message if creation fails.
    #[wasm_bindgen(js_name = newLeafWithContext)] 
    pub fn new_leaf_with_context(&mut self, style: &Style, context: JsValue) -> Result<u64, JsValue> { 
        let id = self.tree.new_leaf_with_context(style.inner.clone(), context).map_err(|e| e.to_string())?; 
        Ok(id.into()) 
    }
    
    /// Creates a new node with the given children.
    /// 
    /// # Arguments
    /// * `style` - The Style object to apply to this node.
    /// * `children` - Array of child node IDs.
    /// 
    /// # Returns
    /// * `Ok(u64)` - The node ID of the newly created node.
    /// * `Err(JsValue)` - Error message if creation fails.
    #[wasm_bindgen(js_name = newWithChildren)] 
    pub fn new_with_children(&mut self, style: &Style, children: Box<[u64]>) -> Result<u64, JsValue> { 
        let children_ids: Vec<NodeId> = children.iter().map(|&id| NodeId::from(id)).collect(); 
        let id = self.tree.new_with_children(style.inner.clone(), &children_ids).map_err(|e| e.to_string())?; 
        Ok(id.into()) 
    }
    
    // =========================================================================
    // Style Management
    // =========================================================================
    
    /// Sets the style for an existing node.
    /// 
    /// This will mark the node and its ancestors as dirty, triggering
    /// a re-layout on the next `computeLayout()` call.
    /// 
    /// # Arguments
    /// * `node` - The node ID to update.
    /// * `style` - The new Style to apply.
    #[wasm_bindgen(js_name = setStyle)] 
    pub fn set_style(&mut self, node: u64, style: &Style) -> Result<(), JsValue> { 
        self.tree.set_style(NodeId::from(node), style.inner.clone()).map_err(|e| e.to_string())?; 
        Ok(()) 
    }
    
    /// Gets the style for a node.
    /// 
    /// # Arguments
    /// * `node` - The node ID to query.
    /// 
    /// # Returns
    /// * `Ok(Style)` - A copy of the node's style.
    /// * `Err(JsValue)` - Error if the node doesn't exist.
    #[wasm_bindgen(js_name = getStyle)] 
    pub fn style(&self, node: u64) -> Result<Style, JsValue> { 
        let s = self.tree.style(NodeId::from(node)).map_err(|e| e.to_string())?; 
        Ok(Style { inner: s.clone() }) 
    }
    
    // =========================================================================
    // Tree Operations
    // =========================================================================
    
    /// Removes all nodes from the tree.
    /// 
    /// After calling this, the tree is empty and all previous node IDs are invalid.
    #[wasm_bindgen(js_name = clear)] 
    pub fn clear(&mut self) { self.tree.clear(); }
    
    /// Removes a node from the tree.
    /// 
    /// The node is detached from its parent (if any) and removed from the tree.
    /// Child nodes are NOT automatically removed.
    /// 
    /// # Arguments
    /// * `node` - The node ID to remove.
    /// 
    /// # Returns
    /// * `Ok(u64)` - The ID of the removed node.
    /// * `Err(JsValue)` - Error if the node doesn't exist.
    #[wasm_bindgen(js_name = remove)] 
    pub fn remove(&mut self, node: u64) -> Result<u64, JsValue> { 
        let id = self.tree.remove(NodeId::from(node)).map_err(|e| e.to_string())?; 
        Ok(id.into()) 
    }
    
    /// Appends a child node to a parent.
    /// 
    /// The child is added at the end of the parent's children list.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// * `child` - The child node ID to add.
    #[wasm_bindgen(js_name = addChild)] 
    pub fn add_child(&mut self, parent: u64, child: u64) -> Result<(), JsValue> { 
        self.tree.add_child(NodeId::from(parent), NodeId::from(child)).map_err(|e| e.to_string())?; 
        Ok(()) 
    }
    
    /// Removes a specific child from a parent.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// * `child` - The child node ID to remove.
    /// 
    /// # Returns
    /// * `Ok(u64)` - The ID of the removed child.
    /// * `Err(JsValue)` - Error if parent or child doesn't exist.
    #[wasm_bindgen(js_name = removeChild)] 
    pub fn remove_child(&mut self, parent: u64, child: u64) -> Result<u64, JsValue> { 
        let id = self.tree.remove_child(NodeId::from(parent), NodeId::from(child)).map_err(|e| e.to_string())?; 
        Ok(id.into()) 
    }
    
    /// Removes a child at a specific index.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// * `index` - The zero-based index of the child to remove.
    /// 
    /// # Returns
    /// * `Ok(u64)` - The ID of the removed child.
    /// * `Err(JsValue)` - Error if index is out of bounds.
    #[wasm_bindgen(js_name = removeChildAtIndex)] 
    pub fn remove_child_at_index(&mut self, parent: u64, index: usize) -> Result<u64, JsValue> { 
        let id = self.tree.remove_child_at_index(NodeId::from(parent), index).map_err(|e| e.to_string())?; 
        Ok(id.into()) 
    }
    
    /// Removes a range of children from a parent.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// * `start` - The start index (inclusive).
    /// * `end` - The end index (exclusive).
    #[wasm_bindgen(js_name = removeChildrenRange)] 
    pub fn remove_children_range(&mut self, parent: u64, start: usize, end: usize) -> Result<(), JsValue> { 
        self.tree.remove_children_range(NodeId::from(parent), start..end).map_err(|e| e.to_string())?; 
        Ok(()) 
    }
    
    /// Inserts a child at a specific index.
    /// 
    /// Existing children at and after the index are shifted right.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// * `index` - The zero-based index at which to insert.
    /// * `child` - The child node ID to insert.
    #[wasm_bindgen(js_name = insertChildAtIndex)] 
    pub fn insert_child_at_index(&mut self, parent: u64, index: usize, child: u64) -> Result<(), JsValue> { 
        self.tree.insert_child_at_index(NodeId::from(parent), index, NodeId::from(child)).map_err(|e| e.to_string())?; 
        Ok(()) 
    }
    
    /// Replaces a child at a specific index with a new child.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// * `index` - The zero-based index of the child to replace.
    /// * `child` - The new child node ID.
    /// 
    /// # Returns
    /// * `Ok(u64)` - The ID of the replaced (old) child.
    /// * `Err(JsValue)` - Error if index is out of bounds.
    #[wasm_bindgen(js_name = replaceChildAtIndex)] 
    pub fn replace_child_at_index(&mut self, parent: u64, index: usize, child: u64) -> Result<u64, JsValue> { 
        let id = self.tree.replace_child_at_index(NodeId::from(parent), index, NodeId::from(child)).map_err(|e| e.to_string())?; 
        Ok(id.into()) 
    }
    
    /// Replaces all children of a node.
    /// 
    /// Previous children are detached but not removed from the tree.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// * `children` - Array of new child node IDs.
    #[wasm_bindgen(js_name = setChildren)] 
    pub fn set_children(&mut self, parent: u64, children: Box<[u64]>) -> Result<(), JsValue> { 
        let children_ids: Vec<NodeId> = children.iter().map(|&id| NodeId::from(id)).collect(); 
        self.tree.set_children(NodeId::from(parent), &children_ids).map_err(|e| e.to_string())?; 
        Ok(()) 
    }
    
    // =========================================================================
    // Tree Queries
    // =========================================================================
    
    /// Gets the parent of a node.
    /// 
    /// # Arguments
    /// * `child` - The child node ID.
    /// 
    /// # Returns
    /// * `Some(u64)` - The parent node ID.
    /// * `None` - If the node has no parent (is a root).
    #[wasm_bindgen(js_name = parent)] 
    pub fn parent(&self, child: u64) -> Option<u64> { 
        self.tree.parent(NodeId::from(child)).map(|n| n.into()) 
    }
    
    /// Gets all children of a node.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// 
    /// # Returns
    /// * `Ok(Box<[u64]>)` - Array of child node IDs.
    /// * `Err(JsValue)` - Error if the node doesn't exist.
    #[wasm_bindgen(js_name = children)] 
    pub fn children(&self, parent: u64) -> Result<Box<[u64]>, JsValue> { 
        let children = self.tree.children(NodeId::from(parent)).map_err(|e| e.to_string())?; 
        let res: Vec<u64> = children.iter().map(|n| (*n).into()).collect(); 
        Ok(res.into_boxed_slice()) 
    }
    
    /// Gets the number of children of a node.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// 
    /// # Returns
    /// The number of children.
    #[wasm_bindgen(js_name = childCount)] 
    pub fn child_count(&self, parent: u64) -> usize { 
        self.tree.child_count(NodeId::from(parent)) 
    }
    
    /// Gets a child at a specific index.
    /// 
    /// # Arguments
    /// * `parent` - The parent node ID.
    /// * `index` - The zero-based index.
    /// 
    /// # Returns
    /// * `Ok(u64)` - The child node ID at the given index.
    /// * `Err(JsValue)` - Error if index is out of bounds.
    #[wasm_bindgen(js_name = getChildAtIndex)] 
    pub fn child_at_index(&self, parent: u64, index: usize) -> Result<u64, JsValue> { 
        let id = self.tree.child_at_index(NodeId::from(parent), index).map_err(|e| e.to_string())?; 
        Ok(id.into()) 
    }
    
    // =========================================================================
    // Dirty Tracking
    // =========================================================================
    
    /// Marks a node as dirty, requiring re-layout.
    /// 
    /// Call this when a node's content changes (e.g., text content)
    /// but its style hasn't. Style changes automatically mark nodes dirty.
    /// 
    /// # Arguments
    /// * `node` - The node ID to mark dirty.
    #[wasm_bindgen(js_name = markDirty)] 
    pub fn mark_dirty(&mut self, node: u64) -> Result<(), JsValue> { 
        self.tree.mark_dirty(NodeId::from(node)).map_err(|e| e.to_string())?; 
        Ok(()) 
    }
    
    /// Checks if a node is dirty (needs re-layout).
    /// 
    /// # Arguments
    /// * `node` - The node ID to check.
    /// 
    /// # Returns
    /// * `Ok(true)` - The node needs re-layout.
    /// * `Ok(false)` - The node's layout is up-to-date.
    /// * `Err(JsValue)` - Error if the node doesn't exist.
    #[wasm_bindgen(js_name = dirty)] 
    pub fn dirty(&self, node: u64) -> Result<bool, JsValue> { 
        self.tree.dirty(NodeId::from(node)).map_err(|e| e.to_string().into()) 
    }
    
    // =========================================================================
    // Layout Computation
    // =========================================================================
    
    /// Computes the layout for a subtree.
    /// 
    /// Runs the layout algorithm (Flexbox/Grid/Block) on the given node
    /// and all its descendants. Results are cached and can be retrieved
    /// with `getLayout()`.
    /// 
    /// # Arguments
    /// * `node` - The root node ID for layout computation.
    /// * `available_space` - The available space constraint, e.g.:
    ///   `{ width: { Definite: 800 }, height: { Definite: 600 } }`
    ///   
    /// Available space options per dimension:
    /// - `{ Definite: number }` - A specific size in pixels
    /// - `"MinContent"` - Use minimum content size
    /// - `"MaxContent"` - Use maximum content size
    #[wasm_bindgen(js_name = computeLayout)]
    pub fn compute_layout(&mut self, node: u64, available_space: JsValue) -> Result<(), JsValue> {
        let js_space: JsAvailableSize = serde_wasm_bindgen::from_value(available_space)?;
        let space: Size<AvailableSpace> = js_space.into();
        self.tree.compute_layout(NodeId::from(node), space).map_err(|e| e.to_string())?;
        Ok(())
    }
    
    /// Computes layout with a custom measure function for leaf nodes.
    /// 
    /// The measure function is called for leaf nodes to determine their
    /// intrinsic size (e.g., for text measurement).
    /// 
    /// # Arguments
    /// * `node` - The root node ID for layout computation.
    /// * `available_space` - The available space constraint.
    /// * `measure_func` - A JavaScript function with signature:
    ///   `(knownDimensions, availableSpace, context) => { width, height }`
    ///   
    /// The measure function receives:
    /// - `knownDimensions`: `{ width: number | null, height: number | null }`
    /// - `availableSpace`: `{ width: AvailableSpace, height: AvailableSpace }`
    /// - `context`: The context value attached to the node (or undefined)
    /// 
    /// And should return: `{ width: number, height: number }`
    #[wasm_bindgen(js_name = computeLayoutWithMeasure)]
    pub fn compute_layout_with_measure(&mut self, node: u64, available_space: JsValue, measure_func: js_sys::Function) -> Result<(), JsValue> {
        let js_space: JsAvailableSize = serde_wasm_bindgen::from_value(available_space)?;
        let space: Size<AvailableSpace> = js_space.into();
        let measure = |known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>, _node: NodeId, context: Option<&mut JsValue>, _style: &TaffyStyle| -> Size<f32> {
             let this = JsValue::NULL;
             let known_val = serde_wasm_bindgen::to_value(&known_dimensions).unwrap_or(JsValue::NULL);
             let available_val = serde_wasm_bindgen::to_value(&available_space).unwrap_or(JsValue::NULL);
             let ctx = context.cloned().unwrap_or(JsValue::UNDEFINED);
             let args = js_sys::Array::new();
             args.push(&known_val);
             args.push(&available_val);
             args.push(&ctx);
             let result_val = measure_func.apply(&this, &args).unwrap_or(JsValue::UNDEFINED);
             serde_wasm_bindgen::from_value(result_val).unwrap_or(Size::ZERO)
        };
        self.tree.compute_layout_with_measure(NodeId::from(node), space, measure).map_err(|e| e.to_string())?;
        Ok(())
    }
    
    // =========================================================================
    // Layout Results
    // =========================================================================
    
    /// Gets the computed layout for a node.
    /// 
    /// Must be called after `computeLayout()`.
    /// 
    /// # Arguments
    /// * `node` - The node ID to query.
    /// 
    /// # Returns
    /// A `Layout` object with computed position, size, and spacing values.
    #[wasm_bindgen(js_name = getLayout)] 
    pub fn layout(&self, node: u64) -> Result<Layout, JsValue> { 
        let layout = self.tree.layout(NodeId::from(node)).map_err(|e| e.to_string())?; 
        Ok(Layout::from(layout)) 
    }
    
    /// Gets the unrounded (fractional) layout for a node.
    /// 
    /// Useful when you need sub-pixel precision.
    /// 
    /// # Arguments
    /// * `node` - The node ID to query.
    /// 
    /// # Returns
    /// A `Layout` object with potentially fractional pixel values.
    #[wasm_bindgen(js_name = unroundedLayout)] 
    pub fn unrounded_layout(&self, node: u64) -> Result<Layout, JsValue> { 
        let layout = self.tree.unrounded_layout(NodeId::from(node)); 
        Ok(Layout::from(layout)) 
    }
    
    /// Gets detailed layout information (debug feature).
    /// 
    /// Only available when compiled with the `detailed_layout_info` feature.
    #[cfg(feature = "detailed_layout_info")]
    #[wasm_bindgen(js_name = detailedLayoutInfo)] 
    pub fn detailed_layout_info(&self, node: u64) -> Result<JsValue, JsValue> { 
        let info = self.tree.detailed_layout_info(NodeId::from(node)).map_err(|e| e.to_string())?; 
        let val = serde_wasm_bindgen::to_value(info)?; 
        Ok(val) 
    }
    
    // =========================================================================
    // Node Context
    // =========================================================================
    
    /// Sets a context value for a node.
    /// 
    /// Context values are passed to the measure function during layout.
    /// Use this to attach data like text content or custom metadata.
    /// 
    /// # Arguments
    /// * `node` - The node ID.
    /// * `context` - Any JavaScript value.
    #[wasm_bindgen(js_name = setNodeContext)] 
    pub fn set_node_context(&mut self, node: u64, context: JsValue) -> Result<(), JsValue> { 
        self.tree.set_node_context(NodeId::from(node), Some(context)).map_err(|e| e.to_string())?; 
        Ok(()) 
    }
    
    /// Gets the context value for a node.
    /// 
    /// # Arguments
    /// * `node` - The node ID.
    /// 
    /// # Returns
    /// The context value, or `undefined` if not set.
    #[wasm_bindgen(js_name = getNodeContext)] 
    pub fn get_node_context(&self, node: u64) -> Result<JsValue, JsValue> { 
        match self.tree.get_node_context(NodeId::from(node)) { 
            Some(ctx) => Ok(ctx.clone()), 
            None => Ok(JsValue::UNDEFINED) 
        } 
    }
    
    /// Gets a mutable reference to the context value for a node.
    /// 
    /// Note: In WASM, this returns a clone since we can't return mutable references.
    /// 
    /// # Arguments
    /// * `node` - The node ID.
    /// 
    /// # Returns
    /// The context value, or `undefined` if not set.
    #[wasm_bindgen(js_name = getNodeContextMut)] 
    pub fn get_node_context_mut(&mut self, node: u64) -> Result<JsValue, JsValue> { 
        match self.tree.get_node_context_mut(NodeId::from(node)) { 
            Some(ctx) => Ok(ctx.clone()), 
            None => Ok(JsValue::UNDEFINED) 
        } 
    }
    
    /// Gets context values for multiple nodes at once.
    /// 
    /// Useful for batch operations.
    /// 
    /// # Arguments
    /// * `children` - Array of node IDs to query.
    /// 
    /// # Returns
    /// Array of context values in the same order as input.
    #[wasm_bindgen(js_name = getDisjointNodeContextMut)] 
    pub fn get_disjoint_node_context_mut(&mut self, children: Box<[u64]>) -> Result<Box<[JsValue]>, JsValue> { 
        let mut results = Vec::with_capacity(children.len()); 
        for id in children.iter() { 
            match self.tree.get_node_context_mut(NodeId::from(*id)) { 
                Some(ctx) => results.push(ctx.clone()), 
                None => results.push(JsValue::UNDEFINED) 
            } 
        } 
        Ok(results.into_boxed_slice()) 
    }
    
    // =========================================================================
    // Utilities
    // =========================================================================
    
    /// Gets the total number of nodes in the tree.
    /// 
    /// # Returns
    /// The count of all nodes (including removed nodes that haven't been reclaimed).
    #[wasm_bindgen(js_name = totalNodeCount)] 
    pub fn total_node_count(&self) -> usize { 
        self.tree.total_node_count() 
    }
    
    /// Prints the tree structure to the console (for debugging).
    /// 
    /// # Arguments
    /// * `node` - The root node ID to start printing from.
    #[wasm_bindgen(js_name = printTree)] 
    pub fn print_tree(&mut self, node: u64) { 
        self.tree.print_tree(NodeId::from(node)); 
    }
}
