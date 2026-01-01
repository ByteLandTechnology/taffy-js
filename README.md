# Taffy WebAssembly Bindings

> **High-performance Flexbox and CSS Grid layout for JavaScript/TypeScript, powered by Rust and WebAssembly.**

![License](https://img.shields.io/npm/l/taffy-js?style=flat-square) ![Version](https://img.shields.io/npm/v/taffy-js?style=flat-square) ![WASM](https://img.shields.io/badge/platform-wasm-blueviolet?style=flat-square)

**Taffy** is a generic, high-performance UI layout library written in Rust. This package (`taffy-js`) provides WebAssembly bindings, allowing you to use Taffy's standards-compliant Flexbox and Grid algorithms directly in your web or Node.js applications with near-native performance.

## ✨ Features

- **🚀 High Performance**: Leverages the speed of Rust and WebAssembly for complex layout computations.
- **📦 Tiny Footprint**: Highly optimized WASM binary size.
- **🎨 Modern Layouts**: Full support for **Flexbox** and **CSS Grid** specifications.
- **🛠 Framework Agnostic**: Use it with React, Vue, Svelte, vanilla JS, or even in Node.js for server-side layout calculation.
- **🔒 Type-Safe**: Fully typed API with TypeScript definitions included.

## 📦 Installation

```bash
npm install taffy-js
```

> **Note**: This package relies on WebAssembly. Ensure your runtime (modern browser or Node.js) supports WASM.

## 🚀 Usage

Here is a complete example showing how to create a layout tree, style nodes, and compute results.

```typescript
import init, {
  new_leaf,
  new_with_children,
  compute_layout,
  get_layout,
  Display,
  FlexDirection,
  AlignItems,
  JustifyContent,
  Style,
} from "taffy-js";

async function run() {
  // 1. Initialize the WASM module
  await init();

  // 2. Define Styles
  // Styles match CSS properties. You can mix specific units like Pixels, Percent, or Auto.
  const boxStyle: Style = {
    display: Display.Flex,
    width: { value: 100, unit: "Pixels" },
    height: { value: 100, unit: "Pixels" },
    justify_content: JustifyContent.Center,
    align_items: AlignItems.Center,
  };

  const rootStyle: Style = {
    display: Display.Flex,
    // layout 500x500 container
    width: { value: 500, unit: "Pixels" },
    height: { value: 500, unit: "Pixels" },
    flex_direction: FlexDirection.Row,
    justify_content: JustifyContent.SpaceAround,
    align_items: AlignItems.Center,
    gap: {
      width: { value: 20, unit: "Pixels" },
      height: { value: 0, unit: "Pixels" },
    }, // Example of potential gap usage if supported
  };

  // 3. Create Tree Nodes
  // Leaf nodes
  const child1 = new_leaf(boxStyle);
  const child2 = new_leaf(boxStyle);

  // Root node with children
  const root = new_with_children(rootStyle, [child1, child2]);

  // 4. Compute Layout
  // You can pass available space constraints (width/height).
  // Passing values corresponds to "Definite" size, null corresponds to "MaxContent/Available".
  compute_layout(root, {
    width: 500,
    height: 500,
  });

  // 5. Retrieve Results
  const rootLayout = get_layout(root);
  const child1Layout = get_layout(child1);
  const child2Layout = get_layout(child2);

  console.log("Root:", rootLayout); // { x: 0, y: 0, width: 500, height: 500 }
  console.log("Child 1:", child1Layout); // { x: ..., y: ..., width: 100, height: 100 }
  console.log("Child 2:", child2Layout); // { x: ..., y: ..., width: 100, height: 100 }
}

run();
```

## 📐 Architecture

`taffy-js` acts as a thin wrapper around the [Taffy](https://github.com/DioxusLabs/taffy) Rust crate.

1.  **Node Tree**: You build a flat tree of nodes in the WASM memory space using integer IDs (`u32`).
2.  **Style Transfer**: Styles are serialized from JS objects to Rust structs via `serde-wasm-bindgen`.
3.  **Computation**: Rust runs the layout algorithms (Flexbox/Grid).
4.  **Readout**: You query the final computed geometry (x, y, width, height) back to JS.

## 📚 API Reference

### Lifecycle

| Function | Description                                                                     |
| :------- | :------------------------------------------------------------------------------ |
| `init()` | Initializes the WASM module. Must be awaited before calling any other function. |
| `free()` | (Optional) Manually free memory if required by your specific WASM loader setup. |

### Node Management

| Function            | Signature                                      | Description                             |
| :------------------ | :--------------------------------------------- | :-------------------------------------- |
| `new_leaf`          | `(style: Style) -> number`                     | Creates a leaf node (no children).      |
| `new_with_children` | `(style: Style, children: number[]) -> number` | Creates a node containing child nodes.  |
| `add_child`         | `(parent: number, child: number) -> void`      | Appends a child to a parent.            |
| `remove_child`      | `(parent: number, child: number) -> void`      | Removes a specific child from a parent. |
| `set_children`      | `(parent: number, children: number[]) -> void` | Replaces all children of a node.        |
| `remove_node`       | `(node: number) -> void`                       | Deletes a node and frees its memory.    |

### Layout & Style

| Function         | Signature                                       | Description                                                                 |
| :--------------- | :---------------------------------------------- | :-------------------------------------------------------------------------- |
| `set_style`      | `(node: number, style: Style) -> void`          | Updates the style properties of a node.                                     |
| `compute_layout` | `(root: number, space: AvailableSpace) -> void` | Triggers the layout calculation algorithm.                                  |
| `get_layout`     | `(node: number) -> Layout`                      | Returns `{x, y, width, height}` for a node.                                 |
| `mark_dirty`     | `(node: number) -> void`                        | Manually validates a node (usually handled automatically by style setters). |

### Type Definitions

#### `Style`

A comprehensive object mirroring CSS properties.

- **Display**: `Display.Flex`, `Display.Grid`, `Display.None`
- **Dimensions**: `{ value: number, unit: "Pixels" | "Percent" | "Auto" }`
- **Flexbox**: `flex_direction`, `justify_content`, `align_items`, `flex_wrap`, etc.
- **Grid**: `grid_template_rows`, `grid_template_columns`, etc.

#### `AvailableSpace`

Used when triggering computation.

```typescript
interface AvailableSpace {
  width: number | null; // null = unlimited/content-based
  height: number | null; // null = unlimited/content-based
}
```

## 🛠 Building from Source

If you want to contribute or build the WASM binary yourself:

1.  **Prerequisites**: Install Rust and `wasm-pack`.
    ```bash
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    ```
2.  **Build**:
    ```bash
    npm install
    npm run build
    ```
    The artifacts will be generated in the `pkg/` directory.

## 📄 License

MIT License © 2024 ByteLand Technology

This project wraps [Taffy](https://github.com/DioxusLabs/taffy), which is also MIT licensed.
