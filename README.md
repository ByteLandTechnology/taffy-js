# Taffy-JS

[![npm version](https://badge.fury.io/js/taffy-js.svg)](https://www.npmjs.com/package/taffy-js)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance WebAssembly bindings for the [Taffy](https://github.com/DioxusLabs/taffy) layout engine, bringing CSS Flexbox and Grid layout algorithms to JavaScript with near-native performance.

## ✨ Features

- **🚀 High Performance**: WebAssembly-powered layout calculations
- **📦 Complete CSS Support**: Full Flexbox and CSS Grid implementation
- **🔧 Custom Measurement**: Support for custom text/content measurement callbacks
- **📝 TypeScript Ready**: Complete type definitions included
- **🌳 Tree-Based API**: Efficient tree structure for complex layouts
- **💡 Familiar API**: CSS-like property names and values

## 📦 Installation

```bash
npm install taffy-js
```

## 🚀 Quick Start

```typescript
import {
  loadTaffy,
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
} from "taffy-js";

async function main() {
  // Initialize WebAssembly module
  await loadTaffy();

  // Create a layout tree
  const tree = new TaffyTree();

  // Create container style
  const containerStyle = new Style();
  containerStyle.display = Display.Flex;
  containerStyle.flexDirection = FlexDirection.Column;
  containerStyle.alignItems = AlignItems.Center;
  containerStyle.size = { width: 300, height: 200 };
  containerStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

  // Create child styles
  const childStyle = new Style();
  childStyle.flexGrow = 1;
  childStyle.size = { width: "100%", height: "auto" };

  // Create nodes
  const child1 = tree.newLeaf(childStyle);
  const child2 = tree.newLeaf(childStyle);
  const container = tree.newWithChildren(
    containerStyle,
    BigUint64Array.from([child1, child2]),
  );

  // Compute layout
  tree.computeLayout(container, { width: 300, height: 200 });

  // Read computed layouts
  const containerLayout = tree.getLayout(container);
  const child1Layout = tree.getLayout(child1);
  const child2Layout = tree.getLayout(child2);

  console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
  console.log(
    `Child 1: ${child1Layout.width}x${child1Layout.height} at (${child1Layout.x}, ${child1Layout.y})`,
  );
  console.log(
    `Child 2: ${child2Layout.width}x${child2Layout.height} at (${child2Layout.x}, ${child2Layout.y})`,
  );
}

main();
```

## 📖 API Reference

### TaffyTree

The main class for managing layout trees.

```typescript
class TaffyTree {
  // Construction
  constructor();
  static withCapacity(capacity: number): TaffyTree;

  // Node Creation (throws TaffyError on failure)
  newLeaf(style: Style): bigint;
  newLeafWithContext(style: Style, context: any): bigint;
  newWithChildren(style: Style, children: BigUint64Array): bigint;

  // Tree Operations
  clear(): void;
  remove(node: bigint): bigint; // throws TaffyError
  totalNodeCount(): number;

  // Child Management (throws TaffyError on failure)
  addChild(parent: bigint, child: bigint): void;
  removeChild(parent: bigint, child: bigint): bigint;
  setChildren(parent: bigint, children: BigUint64Array): void;
  children(parent: bigint): BigUint64Array;
  childCount(parent: bigint): number;
  parent(child: bigint): bigint | undefined;

  // Style Management (throws TaffyError on failure)
  setStyle(node: bigint, style: Style): void;
  getStyle(node: bigint): Style;

  // Layout Computation (throws TaffyError on failure)
  computeLayout(node: bigint, availableSpace: Size<AvailableSpace>): void;
  computeLayoutWithMeasure(
    node: bigint,
    availableSpace: Size<AvailableSpace>,
    measureFunc: MeasureFunction,
  ): void;

  // Layout Results (throws TaffyError on failure)
  getLayout(node: bigint): Layout;
  unroundedLayout(node: bigint): Layout;

  // Dirty Tracking (throws TaffyError on failure)
  markDirty(node: bigint): void;
  dirty(node: bigint): boolean;

  // Configuration
  enableRounding(): void;
  disableRounding(): void;
}
```

### Style

Configuration object for node layout properties.

```typescript
class Style {
  constructor();

  // Layout Mode
  display: Display; // Block, Flex, Grid, None
  position: Position; // Relative, Absolute
  boxSizing: BoxSizing; // BorderBox, ContentBox
  overflow: Point<Overflow>; // Overflow handling

  // Flexbox Properties
  flexDirection: FlexDirection; // Row, Column, RowReverse, ColumnReverse
  flexWrap: FlexWrap; // NoWrap, Wrap, WrapReverse
  flexGrow: number; // Growth factor (default: 0)
  flexShrink: number; // Shrink factor (default: 1)
  flexBasis: Dimension; // Initial size

  // Alignment Properties
  alignItems: AlignItems | undefined;
  alignSelf: AlignSelf | undefined;
  alignContent: AlignContent | undefined;
  justifyContent: JustifyContent | undefined;
  justifyItems: AlignItems | undefined; // Grid container default justify
  justifySelf: AlignSelf | undefined; // Grid item self-justify

  // Sizing
  size: Size<Dimension>; // Width and height
  minSize: Size<Dimension>; // Minimum constraints
  maxSize: Size<Dimension>; // Maximum constraints
  aspectRatio: number | undefined; // Width/height ratio

  // Spacing
  margin: Rect<LengthPercentageAuto>;
  padding: Rect<LengthPercentage>;
  border: Rect<LengthPercentage>;
  gap: Size<LengthPercentage>; // Row and column gap
  inset: Rect<LengthPercentageAuto>; // For absolute positioning

  // Block Layout Properties
  itemIsTable: boolean; // Is this a table element?
  itemIsReplaced: boolean; // Is this a replaced element (img, video)?
  textAlign: TextAlign; // Legacy text alignment
  scrollbarWidth: number; // Scrollbar gutter width in pixels

  // CSS Grid Container Properties
  gridAutoFlow: GridAutoFlow; // Row, Column, RowDense, ColumnDense
  gridTemplateRows: GridTrack[]; // Track sizing for rows
  gridTemplateColumns: GridTrack[]; // Track sizing for columns
  gridAutoRows: TrackSizing[]; // Size for implicit rows
  gridAutoColumns: TrackSizing[]; // Size for implicit columns
  gridTemplateAreas: GridArea[]; // Named grid areas
  gridTemplateRowNames: string[][]; // Named lines between rows
  gridTemplateColumnNames: string[][]; // Named lines between columns

  // CSS Grid Item Properties
  gridRow: Line<GridPlacement>; // grid-row (start/end)
  gridColumn: Line<GridPlacement>; // grid-column (start/end)
}
```

### Layout

Read-only computed layout result.

```typescript
class Layout {
  // Position (relative to parent)
  readonly x: number;
  readonly y: number;

  // Size
  readonly width: number;
  readonly height: number;

  // Content size (for scrollable content)
  readonly contentWidth: number;
  readonly contentHeight: number;

  // Spacing
  readonly paddingTop: number;
  readonly paddingRight: number;
  readonly paddingBottom: number;
  readonly paddingLeft: number;

  readonly borderTop: number;
  readonly borderRight: number;
  readonly borderBottom: number;
  readonly borderLeft: number;

  readonly marginTop: number;
  readonly marginRight: number;
  readonly marginBottom: number;
  readonly marginLeft: number;

  // Scrollbars
  readonly scrollbarWidth: number;
  readonly scrollbarHeight: number;

  // Rendering order
  readonly order: number;
}
```

### Enums

```typescript
enum Display {
  Block,
  Flex,
  Grid,
  None,
}
enum Position {
  Relative,
  Absolute,
}
enum FlexDirection {
  Row,
  Column,
  RowReverse,
  ColumnReverse,
}
enum FlexWrap {
  NoWrap,
  Wrap,
  WrapReverse,
}
enum AlignItems {
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Baseline,
  Stretch,
}
enum AlignSelf {
  Auto,
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Baseline,
  Stretch,
}
enum AlignContent {
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Stretch,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}
enum JustifyContent {
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Stretch,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}
enum Overflow {
  Visible,
  Clip,
  Hidden,
  Scroll,
}
enum BoxSizing {
  BorderBox,
  ContentBox,
}
enum TextAlign {
  Auto,
  LegacyLeft,
  LegacyRight,
  LegacyCenter,
}
enum GridAutoFlow {
  Row,
  Column,
  RowDense,
  ColumnDense,
}
```

### Types

```typescript
// Dimension values (CSS-like syntax)
type Dimension = number | `${number}%` | "auto"; // e.g., 100, "50%", "auto"
type LengthPercentage = number | `${number}%`; // e.g., 10, "25%"
type LengthPercentageAuto = number | `${number}%` | "auto";

// Geometry
interface Size<T> {
  width: T;
  height: T;
}
interface Rect<T> {
  left: T;
  right: T;
  top: T;
  bottom: T;
}
interface Point<T> {
  x: T;
  y: T;
}

// Available space for layout computation
type AvailableSpace = number | "minContent" | "maxContent";

// Grid Placement (CSS grid-row-start / grid-column-start)
type GridPlacement = "auto" | number | { span: number };

// Grid Line (CSS grid-row / grid-column shorthand)
interface Line<T> {
  start: T;
  end: T;
}

// Grid Template Area
interface GridArea {
  name: string;
  row_start: number;
  row_end: number;
  column_start: number;
  column_end: number;
}

// Measure function for custom content measurement
type MeasureFunction = (
  knownDimensions: Size<number | undefined>,
  availableSpace: Size<AvailableSpace>,
  node: bigint,
  context: any,
  style: Style,
) => Size<number>;
```

## 📐 Custom Text Measurement

For text nodes or other content that needs dynamic measurement:

```typescript
const textNode = tree.newLeafWithContext(textStyle, { text: "Hello, World!" });

tree.computeLayoutWithMeasure(
  rootNode,
  { width: 800, height: "maxContent" },
  (known, available, node, context, style) => {
    if (context?.text) {
      // Your text measurement logic here
      const width = measureTextWidth(context.text);
      const height = measureTextHeight(context.text, available.width);
      return { width, height };
    }
    return { width: 0, height: 0 };
  },
);
```

## 🔧 Error Handling

Methods that can fail throw a `TaffyError` as a JavaScript exception. Use try-catch to handle errors:

```typescript
try {
  const nodeId = tree.newLeaf(style);
  console.log("Created node:", nodeId);
} catch (error) {
  // error is a TaffyError instance
  console.error("Error:", error.message);
}
```

## 🌐 Browser Support

Taffy-JS works in all modern browsers that support WebAssembly:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## 📚 Examples

### Flexbox Row Layout

```typescript
const rowStyle = new Style();
rowStyle.display = Display.Flex;
rowStyle.flexDirection = FlexDirection.Row;
rowStyle.justifyContent = JustifyContent.SpaceBetween;
rowStyle.gap = { width: 10, height: 0 };
```

### CSS Grid Layout

```typescript
import { Style, Display, GridAutoFlow } from "taffy-js";

const gridStyle = new Style();
gridStyle.display = Display.Grid;
gridStyle.gridAutoFlow = GridAutoFlow.Row;
gridStyle.gap = { width: 10, height: 10 };

// Grid item placement
const itemStyle = new Style();
itemStyle.gridRow = { start: 1, end: 3 }; // Spans 2 rows
itemStyle.gridColumn = { start: 1, end: { span: 2 } }; // Spans 2 columns
```

### Grid Template Areas

```typescript
const gridStyle = new Style();
gridStyle.display = Display.Grid;
gridStyle.gridTemplateAreas = [
  { name: "header", row_start: 1, row_end: 2, column_start: 1, column_end: 4 },
  { name: "sidebar", row_start: 2, row_end: 4, column_start: 1, column_end: 2 },
  { name: "main", row_start: 2, row_end: 4, column_start: 2, column_end: 4 },
  { name: "footer", row_start: 4, row_end: 5, column_start: 1, column_end: 4 },
];

// Named grid lines
gridStyle.gridTemplateRowNames = [
  ["header-start"],
  ["header-end", "content-start"],
  ["content-end", "footer-start"],
  ["footer-end"],
];
```

### Absolute Positioning

```typescript
const absoluteStyle = new Style();
absoluteStyle.position = Position.Absolute;
absoluteStyle.inset = { left: 10, top: 10, right: "auto", bottom: "auto" };
absoluteStyle.size = { width: 100, height: 50 };
```

### Percentage Sizing

```typescript
const percentStyle = new Style();
percentStyle.size = {
  width: "50%", // 50% of parent
  height: "100%", // 100% of parent
};
```

### Block Layout with Replaced Elements

```typescript
const imgStyle = new Style();
imgStyle.itemIsReplaced = true;
imgStyle.aspectRatio = 16 / 9; // 16:9 aspect ratio
imgStyle.size = { width: "100%", height: "auto" };
```

## 🏗️ Building from Source

```bash
# Clone the repository
git clone https://github.com/ByteLandTechnology/taffy-js.git
cd taffy-js

# Install dependencies
npm install

# Build the WebAssembly module
npm run build

# Run tests
npm test
```

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- [Taffy](https://github.com/DioxusLabs/taffy) - The Rust layout engine this project wraps
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/WebAssembly interoperability
