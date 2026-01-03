# Taffy-JS: WebAssembly Bindings for Taffy Layout Engine

> **High-performance Flexbox and CSS Grid layout for JavaScript/TypeScript, powered by Rust and WebAssembly.**

![License](https://img.shields.io/npm/l/taffy-js?style=flat-square) ![Version](https://img.shields.io/npm/v/taffy-js?style=flat-square) ![WASM](https://img.shields.io/badge/platform-wasm-blueviolet?style=flat-square)

**Taffy** is a high-performance UI layout library written in Rust. This package (`taffy-js`) provides WebAssembly bindings, enabling JavaScript/TypeScript applications to use standards-compliant Flexbox, CSS Grid, and Block layout algorithms with near-native performance.

## ✨ Features

- 🚀 **High Performance** – Rust + WebAssembly for complex layout computations
- 📦 **Tiny Footprint** – Optimized WASM binary size
- 🎨 **Modern Layouts** – Full Flexbox, CSS Grid, and Block layout support
- 🛠 **Framework Agnostic** – Works with React, Vue, Svelte, vanilla JS, Node.js
- 🔒 **Type-Safe** – Full TypeScript definitions included
- 📐 **Custom Measurement** – Support for text measurement via callback functions

## 📦 Installation

```bash
npm install taffy-js
```

> **Note**: Requires a runtime that supports WebAssembly (all modern browsers and Node.js 12+).

## 🚀 Quick Start

```typescript
import init, {
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
  JustifyContent,
} from "taffy-js";

async function main() {
  // 1. Initialize WASM module
  await init();

  // 2. Create tree and styles
  const tree = new TaffyTree();

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.flex_direction = FlexDirection.Row;
  rootStyle.justify_content = JustifyContent.SpaceAround;
  rootStyle.align_items = AlignItems.Center;
  rootStyle.size = { width: { Length: 500 }, height: { Length: 400 } };

  const childStyle = new Style();
  childStyle.size = { width: { Length: 100 }, height: { Length: 100 } };

  // 3. Build the tree
  const child1 = tree.newLeaf(childStyle);
  const child2 = tree.newLeaf(childStyle);
  const root = tree.newWithChildren(rootStyle, [child1, child2]);

  // 4. Compute layout
  tree.computeLayout(root, {
    width: { Definite: 500 },
    height: { Definite: 400 },
  });

  // 5. Read results
  console.log("Root:", tree.getLayout(root));
  console.log("Child 1:", tree.getLayout(child1));
  console.log("Child 2:", tree.getLayout(child2));
}

main();
```

## 📐 Architecture

The library is organized into four main components:

| Component     | Description                                                                           |
| :------------ | :------------------------------------------------------------------------------------ |
| **Enums**     | CSS layout enum types (Display, Position, FlexDirection, FlexWrap, AlignItems, etc.)  |
| **DTOs**      | Data Transfer Objects for JS ↔ Rust serialization (JsDimension, JsSize, JsRect, etc.) |
| **Style**     | Node style configuration with getter/setter methods for all CSS layout properties     |
| **TaffyTree** | Layout tree manager for node creation, tree manipulation, and layout computation      |

### How It Works

1. **Create Tree** – Instantiate a `TaffyTree` to manage layout nodes
2. **Define Styles** – Create `Style` objects and set CSS layout properties
3. **Build Nodes** – Use `newLeaf()` or `newWithChildren()` to create nodes
4. **Compute Layout** – Call `computeLayout()` on the root node
5. **Read Results** – Use `getLayout()` to retrieve computed positions and sizes

---

## 📚 API Reference

### TaffyTree Class

The main entry point for layout computation.

#### Constructors

| Method                      | Description                                              |
| :-------------------------- | :------------------------------------------------------- |
| `new TaffyTree()`           | Creates a new empty layout tree                          |
| `TaffyTree.withCapacity(n)` | Creates a tree with pre-allocated capacity for `n` nodes |

#### Configuration

| Method              | Description                                              |
| :------------------ | :------------------------------------------------------- |
| `enableRounding()`  | Enables rounding layout values to whole pixels (default) |
| `disableRounding()` | Disables rounding for sub-pixel precision                |

#### Node Creation

| Method               | Signature                                     | Description                               |
| :------------------- | :-------------------------------------------- | :---------------------------------------- |
| `newLeaf`            | `(style: Style) → number`                     | Creates a leaf node (no children)         |
| `newLeafWithContext` | `(style: Style, context: any) → number`       | Creates a leaf with attached context data |
| `newWithChildren`    | `(style: Style, children: number[]) → number` | Creates a container node with children    |

#### Style Management

| Method     | Signature                             | Description                          |
| :--------- | :------------------------------------ | :----------------------------------- |
| `setStyle` | `(node: number, style: Style) → void` | Updates a node's style (marks dirty) |
| `getStyle` | `(node: number) → Style`              | Returns a copy of the node's style   |

#### Tree Operations

| Method                | Signature                         | Description                    |
| :-------------------- | :-------------------------------- | :----------------------------- |
| `addChild`            | `(parent, child) → void`          | Appends a child to a parent    |
| `removeChild`         | `(parent, child) → number`        | Removes and returns the child  |
| `removeChildAtIndex`  | `(parent, index) → number`        | Removes child at index         |
| `insertChildAtIndex`  | `(parent, index, child) → void`   | Inserts child at index         |
| `replaceChildAtIndex` | `(parent, index, child) → number` | Replaces and returns old child |
| `setChildren`         | `(parent, children[]) → void`     | Replaces all children          |
| `remove`              | `(node) → number`                 | Removes node from tree         |
| `clear`               | `() → void`                       | Removes all nodes              |

#### Tree Queries

| Method            | Signature                  | Description                 |
| :---------------- | :------------------------- | :-------------------------- |
| `parent`          | `(child) → number \| null` | Returns parent node ID      |
| `children`        | `(parent) → number[]`      | Returns array of child IDs  |
| `childCount`      | `(parent) → number`        | Returns number of children  |
| `getChildAtIndex` | `(parent, index) → number` | Returns child at index      |
| `totalNodeCount`  | `() → number`              | Returns total nodes in tree |

#### Dirty Tracking

| Method      | Signature          | Description                    |
| :---------- | :----------------- | :----------------------------- |
| `markDirty` | `(node) → void`    | Marks node for re-layout       |
| `dirty`     | `(node) → boolean` | Checks if node needs re-layout |

#### Layout Computation

| Method                     | Signature                                  | Description                           |
| :------------------------- | :----------------------------------------- | :------------------------------------ |
| `computeLayout`            | `(node, availableSpace) → void`            | Computes layout for subtree           |
| `computeLayoutWithMeasure` | `(node, availableSpace, measureFn) → void` | Computes with custom measure function |

#### Layout Results

| Method            | Signature         | Description                           |
| :---------------- | :---------------- | :------------------------------------ |
| `getLayout`       | `(node) → Layout` | Returns computed layout (rounded)     |
| `unroundedLayout` | `(node) → Layout` | Returns layout with fractional values |

#### Node Context

| Method           | Signature                | Description             |
| :--------------- | :----------------------- | :---------------------- |
| `setNodeContext` | `(node, context) → void` | Attaches data to node   |
| `getNodeContext` | `(node) → any`           | Retrieves attached data |

#### Debug

| Method            | Description                      |
| :---------------- | :------------------------------- |
| `printTree(node)` | Prints tree structure to console |

---

### Style Class

Configuration object for CSS layout properties.

```typescript
const style = new Style();
```

#### Layout Mode

| Property   | Type       | Description                               |
| :--------- | :--------- | :---------------------------------------- |
| `display`  | `Display`  | Layout algorithm: Block, Flex, Grid, None |
| `position` | `Position` | Positioning: Relative, Absolute           |

#### Flexbox Properties

| Property         | Type            | Description                                       |
| :--------------- | :-------------- | :------------------------------------------------ |
| `flex_direction` | `FlexDirection` | Main axis: Row, Column, RowReverse, ColumnReverse |
| `flex_wrap`      | `FlexWrap`      | Wrap behavior: NoWrap, Wrap, WrapReverse          |
| `flex_grow`      | `number`        | Grow factor (default: 0)                          |
| `flex_shrink`    | `number`        | Shrink factor (default: 1)                        |
| `flex_basis`     | `Dimension`     | Initial size before grow/shrink                   |

#### Alignment

| Property          | Type              | Description                        |
| :---------------- | :---------------- | :--------------------------------- |
| `align_items`     | `AlignItems?`     | Cross-axis alignment for children  |
| `align_self`      | `AlignSelf?`      | Cross-axis alignment for this item |
| `align_content`   | `AlignContent?`   | Multi-line cross-axis alignment    |
| `justify_content` | `JustifyContent?` | Main-axis alignment                |

#### Sizing

| Property       | Type                | Description              |
| :------------- | :------------------ | :----------------------- |
| `size`         | `{ width, height }` | Element dimensions       |
| `min_size`     | `{ width, height }` | Minimum size constraints |
| `max_size`     | `{ width, height }` | Maximum size constraints |
| `aspect_ratio` | `number?`           | Width-to-height ratio    |
| `box_sizing`   | `BoxSizing`         | Size calculation mode    |

#### Spacing

| Property  | Type                           | Description                           |
| :-------- | :----------------------------- | :------------------------------------ |
| `margin`  | `{ left, right, top, bottom }` | Outer spacing (supports Auto)         |
| `padding` | `{ left, right, top, bottom }` | Inner spacing                         |
| `border`  | `{ left, right, top, bottom }` | Border width                          |
| `gap`     | `{ width, height }`            | Gap between children (column/row gap) |
| `inset`   | `{ left, right, top, bottom }` | Absolute positioning offsets          |

#### Overflow

| Property   | Type       | Description                |
| :--------- | :--------- | :------------------------- |
| `overflow` | `{ x, y }` | Overflow behavior per axis |

---

### Type Definitions

#### Dimension (JsDimension)

Values for size properties:

```typescript
// Fixed pixel value
{
  Length: 100;
}

// Percentage of parent
{
  Percent: 0.5;
} // 50%

// Automatic sizing
("Auto");
```

#### LengthPercentage (JsLengthPercentage)

For properties that don't support Auto (padding, border):

```typescript
{
  Length: 10;
}
{
  Percent: 0.1;
}
```

#### LengthPercentageAuto (JsLengthPercentageAuto)

For properties that support Auto (margin, inset):

```typescript
{
  Length: 10;
}
{
  Percent: 0.1;
}
("Auto");
```

#### AvailableSpace (JsAvailableSize)

Constraints for layout computation:

```typescript
{
  width: { Definite: 800 },   // Fixed width
  height: { Definite: 600 }   // Fixed height
}

{
  width: "MaxContent",        // Intrinsic max width
  height: "MinContent"        // Intrinsic min height
}
```

#### Layout Result

Returned by `getLayout()`:

```typescript
{
  order: number,
  size: { width: number, height: number },
  location: { x: number, y: number },
  padding: { left, right, top, bottom },
  border: { left, right, top, bottom },
  scrollbar_size: { width, height },
  content_size: { width, height }
}
```

---

### Enums

#### Display

```typescript
Display.Block; // Block layout (default)
Display.Flex; // Flexbox container
Display.Grid; // CSS Grid container
Display.None; // Hidden, takes no space
```

#### Position

```typescript
Position.Relative; // Normal document flow (default)
Position.Absolute; // Removed from flow, positioned via inset
```

#### FlexDirection

```typescript
FlexDirection.Row; // Horizontal, left to right
FlexDirection.Column; // Vertical, top to bottom
FlexDirection.RowReverse; // Horizontal, right to left
FlexDirection.ColumnReverse; // Vertical, bottom to top
```

#### FlexWrap

```typescript
FlexWrap.NoWrap; // Single line (default)
FlexWrap.Wrap; // Wrap to multiple lines
FlexWrap.WrapReverse; // Wrap in reverse order
```

#### AlignItems / AlignSelf

```typescript
AlignItems.Start; // Align to start
AlignItems.End; // Align to end
AlignItems.FlexStart; // Align to flex start
AlignItems.FlexEnd; // Align to flex end
AlignItems.Center; // Center alignment
AlignItems.Baseline; // Baseline alignment
AlignItems.Stretch; // Stretch to fill

AlignSelf.Auto; // Inherit from parent (AlignSelf only)
```

#### AlignContent

```typescript
AlignContent.Start;
AlignContent.End;
AlignContent.FlexStart;
AlignContent.FlexEnd;
AlignContent.Center;
AlignContent.Stretch;
AlignContent.SpaceBetween;
AlignContent.SpaceAround;
AlignContent.SpaceEvenly;
```

#### JustifyContent

```typescript
JustifyContent.Start;
JustifyContent.End;
JustifyContent.FlexStart;
JustifyContent.FlexEnd;
JustifyContent.Center;
JustifyContent.Stretch;
JustifyContent.SpaceBetween;
JustifyContent.SpaceAround;
JustifyContent.SpaceEvenly;
```

#### Overflow

```typescript
Overflow.Visible; // Content not clipped
Overflow.Hidden; // Content clipped
Overflow.Scroll; // Always show scrollbars
Overflow.Auto; // Show scrollbars when needed
```

#### BoxSizing

```typescript
BoxSizing.BorderBox; // Include padding/border in size (default)
BoxSizing.ContentBox; // Size is content only
```

---

## 📏 Custom Measurement

For nodes with intrinsic sizes (like text), use `computeLayoutWithMeasure`:

```typescript
tree.computeLayoutWithMeasure(
  root,
  { width: { Definite: 800 }, height: { Definite: 600 } },
  (knownDimensions, availableSpace, context) => {
    // knownDimensions: { width: number | null, height: number | null }
    // availableSpace: { width: AvailableSpace, height: AvailableSpace }
    // context: The value attached via setNodeContext/newLeafWithContext

    // Return the measured size
    return { width: 100, height: 20 };
  },
);
```

**Example with text measurement:**

```typescript
// Create a text node with context
const textNode = tree.newLeafWithContext(style, { text: "Hello World" });

// Measure function
tree.computeLayoutWithMeasure(root, availableSpace, (known, available, ctx) => {
  if (ctx?.text) {
    // Use your text measurement library here
    const measured = measureText(ctx.text, available.width);
    return { width: measured.width, height: measured.height };
  }
  return { width: 0, height: 0 };
});
```

---

## 🛠 Building from Source

1. **Prerequisites**: Install Rust and `wasm-pack`

   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

2. **Build**

   ```bash
   npm install
   npm run build
   ```

3. **Build with debug features**

   ```bash
   wasm-pack build --features console_error_panic_hook
   ```

---

## 📄 License

MIT License © 2024 ByteLand Technology

This project wraps [Taffy](https://github.com/DioxusLabs/taffy), which is also MIT licensed.
