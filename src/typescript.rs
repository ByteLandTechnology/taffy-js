//! # TypeScript Custom Type Declarations Module
//!
//! This module contains additional TypeScript type declarations that are appended
//! to the generated `.d.ts` file via `wasm_bindgen(typescript_custom_section)`.
//!
//! ## Overview
//!
//! These types provide accurate TypeScript definitions for complex types that
//! wasm-bindgen cannot automatically generate, including:
//!
//! - `AvailableSpace`, `Size<T>`, `Rect<T>`, `Point<T>`
//! - `Dimension`, `LengthPercentage`, `LengthPercentageAuto`
//! - `MeasureFunction` callback signature
//! - Detailed grid layout info types
//! - `GridPlacement` and `Line<T>` for grid positioning

use wasm_bindgen::prelude::*;

// =============================================================================
// TypeScript Custom Type Declarations
// =============================================================================

/// Additional TypeScript type declarations appended to the generated `.d.ts` file
///
/// These types provide accurate TypeScript definitions for complex types that
/// wasm-bindgen cannot automatically generate.
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Available space constraint for layout computation.
 *
 * Specifies how much space is available for a node during layout calculation.
 * This is passed to `computeLayout()` to define the container constraints.
 *
 * @remarks
 * - Use `number` when you have a fixed container size
 * - Use `"minContent"` to shrink-wrap to the minimum content size
 * - Use `"maxContent"` to expand to fit all content without wrapping
 *
 * @example
 * ```typescript
 * import init, { TaffyTree, Style, type AvailableSpace, type Size } from 'taffy-js';
 *
 * await init();
 * const tree = new TaffyTree();
 * const root: bigint = tree.newLeaf(new Style());
 *
 * // Fixed size container with type annotation
 * const fixedSpace: Size<AvailableSpace> = {
 *   width: 800,
 *   height: 600
 * };
 * tree.computeLayout(root, fixedSpace);
 *
 * // Flexible width, fixed height
 * const flexibleSpace: Size<AvailableSpace> = {
 *   width: "maxContent",
 *   height: 400
 * };
 * tree.computeLayout(root, flexibleSpace);
 * ```
 */
export type AvailableSpace = number | "minContent" | "maxContent";

/**
 * Generic size type with width and height.
 *
 * A two-dimensional container for width and height values. The type parameter `T`
 * determines what kind of values are stored.
 *
 * @typeParam T - The type of each dimension (e.g., `number`, `Dimension`, `AvailableSpace`)
 *
 * @property width - The horizontal dimension value
 * @property height - The vertical dimension value
 *
 * @example
 * ```typescript
 * import type { Size, Dimension, AvailableSpace } from 'taffy-js';
 *
 * // Size with explicit type parameters
 * const pixelSize: Size<number> = { width: 200, height: 100 };
 *
 * const dimensionSize: Size<Dimension> = {
 *   width: 200,
 *   height: "50%"
 * };
 *
 * const availableSize: Size<AvailableSpace> = {
 *   width: 800,
 *   height: "maxContent"
 * };
 * ```
 */
export interface Size<T> {
  /** The horizontal dimension value */
  width: T;
  /** The vertical dimension value */
  height: T;
}

/**
 * Custom measure function for leaf nodes with text or other dynamic content.
 *
 * This callback is invoked during layout computation for leaf nodes that need
 * custom sizing based on their content (e.g., text nodes that need text measurement).
 *
 * @param knownDimensions - Dimensions already determined by constraints. Each dimension
 *                          is `number` if known, or `undefined` if needs to be measured.
 * @param availableSpace - The available space constraints for the node. Can be definite
 *                         pixels, "minContent", or "maxContent".
 * @param node - The node ID (`bigint`) of the node being measured
 * @param context - User-provided context attached to the node via `newLeafWithContext()`
 * @param style - The node's current Style configuration
 *
 * @returns - The measured size of the content in pixels
 *
 * @example
 * ```typescript
 * import init, { TaffyTree, Style, type MeasureFunction, type Size } from 'taffy-js';
 *
 * interface TextContext {
 *   text: string;
 *   fontSize: number;
 * }
 *
 * await init();
 * const tree = new TaffyTree();
 *
 * const style = new Style();
 * const context: TextContext = { text: "Hello, World!", fontSize: 16 };
 * const textNode: bigint = tree.newLeafWithContext(style, context);
 *
 * // Typed measure function
 * const measureText: MeasureFunction = (
 *   knownDimensions,
 *   availableSpace,
 *   node,
 *   context,
 *   style
 * ): Size<number> => {
 *   const ctx = context as TextContext | undefined;
 *   if (!ctx?.text) return { width: 0, height: 0 };
 *
 *   const width = knownDimensions.width ?? measureTextWidth(ctx.text, ctx.fontSize);
 *   const height = knownDimensions.height ?? ctx.fontSize * 1.2;
 *
 *   return { width, height };
 * };
 *
 * tree.computeLayoutWithMeasure(
 *   textNode,
 *   { width: 200, height: "maxContent" },
 *   measureText
 * );
 * ```
 */
export type MeasureFunction = (
  knownDimensions: Size<number | undefined>,
  availableSpace: Size<AvailableSpace>,
  node: bigint,
  context: any,
  style: Style,
) => Size<number>;

/**
 * Dimension type supporting length, percentage, or auto values.
 *
 * Used for sizing properties like `width`, `height`, `flexBasis`, etc.
 *
 * @remarks
 * - `number`: Fixed size in pixels
 * - `"{number}%"`: Percentage of parent's size (0-100)
 * - `"auto"`: Size determined by content or layout algorithm
 *
 * @example
 * ```typescript
 * import { Style, type Dimension, type Size } from 'taffy-js';
 *
 * const style = new Style();
 *
 * // With explicit type annotations
 * const fixedSize: Size<Dimension> = {
 *   width: 200,
 *   height: 100
 * };
 *
 * const percentSize: Size<Dimension> = {
 *   width: "50%",
 *   height: "100%"
 * };
 *
 * const autoSize: Size<Dimension> = {
 *   width: "auto",
 *   height: "auto"
 * };
 *
 * style.size = fixedSize;
 * ```
 */
export type Dimension = number | `${number}%` | "auto";

/**
 * Length or percentage value (no auto support).
 *
 * Used for properties that require explicit values, such as `padding`, `border`, and `gap`.
 *
 * @remarks
 * - `number`: Fixed size in pixels
 * - `"{number}%"`: Percentage of parent's size (0-100)
 *
 * @example
 * ```typescript
 * import { Style, type LengthPercentage, type Rect, type Size } from 'taffy-js';
 *
 * const style = new Style();
 *
 * const padding: Rect<LengthPercentage> = {
 *   left: 10,
 *   right: 10,
 *   top: 5,
 *   bottom: 5
 * };
 *
 * const gap: Size<LengthPercentage> = {
 *   width: "5%",
 *   height: "5%"
 * };
 *
 * style.padding = padding;
 * style.gap = gap;
 * ```
 */
export type LengthPercentage = number | `${number}%`;

/**
 * Length, percentage, or auto value.
 *
 * Used for properties that support auto values, such as `margin` and `inset`.
 *
 * @remarks
 * - `number`: Fixed size in pixels
 * - `"{number}%"`: Percentage of parent's size (0-100)
 * - `"auto"`: Automatic value (behavior depends on property)
 *
 * @example
 * ```typescript
 * import { Style, type LengthPercentageAuto, type Rect } from 'taffy-js';
 *
 * const style = new Style();
 *
 * // Auto margins for horizontal centering
 * const centerMargin: Rect<LengthPercentageAuto> = {
 *   left: "auto",
 *   right: "auto",
 *   top: 0,
 *   bottom: 0
 * };
 *
 * style.margin = centerMargin;
 * ```
 */
export type LengthPercentageAuto = number | `${number}%` | "auto";

/**
 * Point with x and y coordinates/values.
 *
 * Used for properties that have separate horizontal (x) and vertical (y) values,
 * such as `overflow`.
 *
 * @typeParam T - The type of each coordinate
 *
 * @property x - The horizontal value
 * @property y - The vertical value
 *
 * @example
 * ```typescript
 * import { Style, Overflow, type Point } from 'taffy-js';
 *
 * const style = new Style();
 *
 * const overflow: Point<typeof Overflow[keyof typeof Overflow]> = {
 *   x: Overflow.Hidden,
 *   y: Overflow.Scroll
 * };
 *
 * style.overflow = overflow;
 * ```
 */
export interface Point<T> {
  /** The horizontal (x-axis) value */
  x: T;
  /** The vertical (y-axis) value */
  y: T;
}

/**
 * Rectangle with left, right, top, and bottom values.
 *
 * Used for box model properties like `margin`, `padding`, `border`, and `inset`.
 *
 * @typeParam T - The type of each side value
 *
 * @property left - The left side value
 * @property right - The right side value
 * @property top - The top side value
 * @property bottom - The bottom side value
 *
 * @example
 * ```typescript
 * import { Style, type Rect, type LengthPercentage, type LengthPercentageAuto } from 'taffy-js';
 *
 * const style = new Style();
 *
 * // Typed padding
 * const padding: Rect<LengthPercentage> = {
 *   left: 10,
 *   right: 10,
 *   top: 10,
 *   bottom: 10
 * };
 *
 * // Typed margin with auto
 * const margin: Rect<LengthPercentageAuto> = {
 *   left: "auto",
 *   right: "auto",
 *   top: 10,
 *   bottom: 30
 * };
 *
 * style.padding = padding;
 * style.margin = margin;
 * ```
 */
export interface Rect<T> {
  /** The left side value */
  left: T;
  /** The right side value */
  right: T;
  /** The top side value */
  top: T;
  /** The bottom side value */
  bottom: T;
}

/**
 * Detailed layout information (for grid layouts).
 *
 * Returned by `detailedLayoutInfo()` for nodes using CSS Grid layout.
 * Contains detailed information about grid tracks and item placement.
 *
 * @remarks
 * This is only available when the `detailed_layout_info` feature is enabled.
 *
 * @example
 * ```typescript
 * import type { DetailedLayoutInfo, DetailedGridInfo } from 'taffy-js';
 *
 * const info: DetailedLayoutInfo = tree.detailedLayoutInfo(gridNode);
 *
 * if (info !== "None" && typeof info === 'object' && 'Grid' in info) {
 *   const grid: DetailedGridInfo = info.Grid;
 *   console.log('Rows:', grid.rows.sizes);
 *   console.log('Columns:', grid.columns.sizes);
 * }
 * ```
 */
export type DetailedLayoutInfo = DetailedGridInfo | null;

/**
 * Detailed information about a grid layout.
 *
 * Contains information about grid rows, columns, and item placement.
 *
 * @property rows - Information about row tracks
 * @property columns - Information about column tracks
 * @property items - Array of item placement information
 */
export interface DetailedGridInfo {
  /** Information about the grid's row tracks */
  rows: DetailedGridTracksInfo;
  /** Information about the grid's column tracks */
  columns: DetailedGridTracksInfo;
  /** Placement information for each grid item */
  items: DetailedGridItemsInfo[];
}

/**
 * Information about grid tracks (rows or columns).
 *
 * Provides detailed sizing and gutter information for a set of grid tracks.
 *
 * @property negative_implicit_tracks - Number of implicit tracks before explicit tracks
 * @property explicit_tracks - Number of explicitly defined tracks
 * @property positive_implicit_tracks - Number of implicit tracks after explicit tracks
 * @property gutters - Array of gutter sizes between tracks (in pixels)
 * @property sizes - Array of track sizes (in pixels)
 */
export interface DetailedGridTracksInfo {
  /** Number of implicit tracks before explicit tracks (for negative line numbers) */
  negative_implicit_tracks: number;
  /** Number of tracks explicitly defined in grid-template-rows/columns */
  explicit_tracks: number;
  /** Number of implicit tracks created after explicit tracks */
  positive_implicit_tracks: number;
  /** Gap sizes between tracks in pixels */
  gutters: number[];
  /** Computed sizes of each track in pixels */
  sizes: number[];
}

/**
 * Information about a grid item's placement.
 *
 * Specifies which grid lines the item spans on both axes.
 * Line numbers are 1-indexed, with 1 being the first line.
 *
 * @property row_start - Starting row line number (1-indexed)
 * @property row_end - Ending row line number (exclusive)
 * @property column_start - Starting column line number (1-indexed)
 * @property column_end - Ending column line number (exclusive)
 */
export interface DetailedGridItemsInfo {
  /** Starting row line (1-indexed) */
  row_start: number;
  /** Ending row line (exclusive) */
  row_end: number;
  /** Starting column line (1-indexed) */
  column_start: number;
  /** Ending column line (exclusive) */
  column_end: number;
}

/**
 * Grid placement type for positioning grid items.
 *
 * Specifies how an item is placed on a grid track (row or column).
 * Follows CSS `grid-row-start` / `grid-column-start` specification.
 *
 * @remarks
 * - `"auto"`: Auto-placement using the grid's flow algorithm
 * - `number`: Place at a specific line index (1-indexed, can be negative)
 * - `{ span: number }`: Span a specified number of tracks
 *
 * @example
 * ```typescript
 * import type { GridPlacement, Line } from 'taffy-js';
 *
 * // Line index (CSS: grid-row-start: 2)
 * const lineIndex: GridPlacement = 2;
 *
 * // Auto placement (CSS: grid-row-start: auto)
 * const auto: GridPlacement = "auto";
 *
 * // Span (CSS: grid-row-start: span 3)
 * const span: GridPlacement = { span: 3 };
 * ```
 */
export type GridPlacement = "auto" | number | { span: number };

/**
 * Line type representing start and end positions.
 *
 * A container for start and end values, used for CSS grid-row and grid-column
 * shorthand properties.
 *
 * @typeParam T - The type of start and end values
 *
 * @property start - The starting line/position
 * @property end - The ending line/position
 *
 * @example
 * ```typescript
 * import { Style, Display, type Line, type GridPlacement } from 'taffy-js';
 *
 * const style = new Style();
 * style.display = Display.Grid;
 *
 * // CSS: grid-row: 1 / 3
 * style.gridRow = { start: 1, end: 3 };
 *
 * // CSS: grid-column: 1 / span 2
 * style.gridColumn = { start: 1, end: { span: 2 } };
 *
 * // CSS: grid-row: auto / auto
 * style.gridRow = { start: "auto", end: "auto" };
 * ```
 */
export interface Line<T> {
  /** The starting position (CSS: *-start) */
  start: T;
  /** The ending position (CSS: *-end) */
  end: T;
}
"#;
