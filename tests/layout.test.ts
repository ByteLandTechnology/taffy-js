import { describe, it, expect, beforeAll } from "vitest";
import { setupTaffy } from "./utils";
import {
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
  JustifyContent,
  GridAutoFlow,
} from "../src/index";

describe("Layout Computation", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  it("validates basic Flex layout", () => {
    const tree = new TaffyTree();

    // Root: 100x100 Flex Column
    const rootStyle = new Style();
    rootStyle.display = Display.Flex;
    rootStyle.flexDirection = FlexDirection.Column;
    rootStyle.size = { width: 100, height: 100 };

    const root = tree.newLeaf(rootStyle); // Leaf for now

    // Child 1: 50x50
    const childStyle = new Style();
    childStyle.size = { width: 50, height: 50 };
    const child1 = tree.newLeaf(childStyle);

    // Child 2: 50x50
    const child2 = tree.newLeaf(childStyle);

    // Add children
    tree.addChild(root, child1);
    tree.addChild(root, child2);

    // Compute
    tree.computeLayout(root, { width: 100, height: 100 });

    // Verify Root
    const rootLayout = tree.getLayout(root);
    expect(rootLayout.width).toBe(100);
    expect(rootLayout.height).toBe(100);

    // Verify Children positions (stacked vertically)
    const child1Layout = tree.getLayout(child1);
    expect(child1Layout.x).toBe(0);
    expect(child1Layout.y).toBe(0);
    expect(child1Layout.width).toBe(50);
    expect(child1Layout.height).toBe(50);

    const child2Layout = tree.getLayout(child2);
    expect(child2Layout.x).toBe(0);
    expect(child2Layout.y).toBe(50); // Stacked below child1
    expect(child2Layout.width).toBe(50);
    expect(child2Layout.height).toBe(50);

    // Clean up
    tree.free();
    // Note: Styles are copied into tree or referenced?
    // TaffyTree owns the styles once added? No, Style passed to newLeaf is copied.
    rootStyle.free();
    childStyle.free();
  });

  it("validates Center alignment", () => {
    const tree = new TaffyTree();
    const rootStyle = new Style();
    rootStyle.display = Display.Flex;
    rootStyle.size = { width: 100, height: 100 };
    rootStyle.alignItems = AlignItems.Center;
    rootStyle.justifyContent = JustifyContent.Center;

    const root = tree.newLeaf(rootStyle);

    const childStyle = new Style();
    childStyle.size = { width: 20, height: 20 };
    const child = tree.newLeaf(childStyle);

    tree.addChild(root, child);

    tree.computeLayout(root, { width: 100, height: 100 });

    const layout = tree.getLayout(child);
    // Centered in 100x100: (100-20)/2 = 40
    expect(layout.x).toBe(40);
    expect(layout.y).toBe(40);

    tree.free();
    rootStyle.free();
    childStyle.free();
  });
});

describe("Grid Layout Computation", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  it("validates basic Grid layout with explicit placement", () => {
    const tree = new TaffyTree();

    // Root: 100x100 Grid container
    const rootStyle = new Style();
    rootStyle.display = Display.Grid;
    rootStyle.size = { width: 100, height: 100 };

    const root = tree.newLeaf(rootStyle);

    // Child 1: placed at grid row 1, column 1
    const child1Style = new Style();
    child1Style.gridRow = { start: 1, end: 2 };
    child1Style.gridColumn = { start: 1, end: 2 };
    const child1 = tree.newLeaf(child1Style);

    // Child 2: placed at grid row 1, column 2
    const child2Style = new Style();
    child2Style.gridRow = { start: 1, end: 2 };
    child2Style.gridColumn = { start: 2, end: 3 };
    const child2 = tree.newLeaf(child2Style);

    tree.addChild(root, child1);
    tree.addChild(root, child2);

    tree.computeLayout(root, { width: 100, height: 100 });

    const rootLayout = tree.getLayout(root);
    expect(rootLayout.width).toBe(100);
    expect(rootLayout.height).toBe(100);

    // Verify child layouts - they should be placed side by side
    const child1Layout = tree.getLayout(child1);
    const child2Layout = tree.getLayout(child2);

    // Children should be in different positions
    expect(child1Layout.x).toBeDefined();
    expect(child2Layout.x).toBeDefined();
    // Child 2 should be to the right of child 1
    expect(child2Layout.x).toBeGreaterThanOrEqual(child1Layout.x);

    tree.free();
    rootStyle.free();
    child1Style.free();
    child2Style.free();
  });

  it("validates Grid auto-flow row", () => {
    const tree = new TaffyTree();

    const rootStyle = new Style();
    rootStyle.display = Display.Grid;
    rootStyle.size = { width: 100, height: 100 };
    rootStyle.gridAutoFlow = GridAutoFlow.Row;

    const root = tree.newLeaf(rootStyle);

    // Create 4 auto-placed children
    const childStyles: Style[] = [];
    const children: bigint[] = [];
    for (let i = 0; i < 4; i++) {
      const style = new Style();
      style.size = { width: 20, height: 20 };
      childStyles.push(style);
      const child = tree.newLeaf(style);
      children.push(child);
      tree.addChild(root, child);
    }

    tree.computeLayout(root, { width: 100, height: 100 });

    // Verify all children are placed
    for (const child of children) {
      const layout = tree.getLayout(child);
      expect(layout.width).toBe(20);
      expect(layout.height).toBe(20);
    }

    tree.free();
    rootStyle.free();
    for (const style of childStyles) {
      style.free();
    }
  });

  it("validates Grid with span", () => {
    const tree = new TaffyTree();

    const rootStyle = new Style();
    rootStyle.display = Display.Grid;
    rootStyle.size = { width: 100, height: 100 };

    const root = tree.newLeaf(rootStyle);

    // Child that spans 2 columns
    const childStyle = new Style();
    childStyle.gridRow = { start: 1, end: 2 };
    childStyle.gridColumn = { start: 1, end: { span: 2 } };
    const child = tree.newLeaf(childStyle);

    tree.addChild(root, child);

    tree.computeLayout(root, { width: 100, height: 100 });

    const childLayout = tree.getLayout(child);
    expect(childLayout.width).toBeDefined();
    expect(childLayout.height).toBeDefined();

    tree.free();
    rootStyle.free();
    childStyle.free();
  });

  it("validates Grid with gap", () => {
    const tree = new TaffyTree();

    const rootStyle = new Style();
    rootStyle.display = Display.Grid;
    rootStyle.size = { width: 100, height: 100 };
    rootStyle.gap = { width: 10, height: 10 };

    const root = tree.newLeaf(rootStyle);

    // Two children side by side
    const child1Style = new Style();
    child1Style.gridRow = { start: 1, end: 2 };
    child1Style.gridColumn = { start: 1, end: 2 };
    const child1 = tree.newLeaf(child1Style);

    const child2Style = new Style();
    child2Style.gridRow = { start: 1, end: 2 };
    child2Style.gridColumn = { start: 2, end: 3 };
    const child2 = tree.newLeaf(child2Style);

    tree.addChild(root, child1);
    tree.addChild(root, child2);

    tree.computeLayout(root, { width: 100, height: 100 });

    const child1Layout = tree.getLayout(child1);
    const child2Layout = tree.getLayout(child2);

    // There should be a gap between the children
    const child1Right = child1Layout.x + child1Layout.width;
    expect(child2Layout.x - child1Right).toBeCloseTo(10, 0);

    tree.free();
    rootStyle.free();
    child1Style.free();
    child2Style.free();
  });

  it("validates Grid with scrollbarWidth", () => {
    const tree = new TaffyTree();

    const rootStyle = new Style();
    rootStyle.display = Display.Grid;
    rootStyle.size = { width: 100, height: 100 };
    rootStyle.scrollbarWidth = 15;

    const root = tree.newLeaf(rootStyle);

    const childStyle = new Style();
    childStyle.size = { width: "100%", height: "100%" };
    const child = tree.newLeaf(childStyle);

    tree.addChild(root, child);

    tree.computeLayout(root, { width: 100, height: 100 });

    // The child should account for scrollbar width
    const childLayout = tree.getLayout(child);
    // Content area should be reduced by scrollbar width
    expect(childLayout.width).toBeLessThanOrEqual(100);
    expect(childLayout.height).toBeLessThanOrEqual(100);

    tree.free();
    rootStyle.free();
    childStyle.free();
  });
});

describe("Block Layout Computation", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  it("validates Block layout", () => {
    const tree = new TaffyTree();

    const rootStyle = new Style();
    rootStyle.display = Display.Block;
    rootStyle.size = { width: 100, height: 100 };

    const root = tree.newLeaf(rootStyle);

    const childStyle = new Style();
    childStyle.size = { width: 50, height: 50 };
    const child = tree.newLeaf(childStyle);

    tree.addChild(root, child);

    tree.computeLayout(root, { width: 100, height: 100 });

    const childLayout = tree.getLayout(child);
    expect(childLayout.width).toBe(50);
    expect(childLayout.height).toBe(50);

    tree.free();
    rootStyle.free();
    childStyle.free();
  });

  it("validates Block layout with itemIsReplaced", () => {
    const tree = new TaffyTree();

    const rootStyle = new Style();
    rootStyle.display = Display.Block;
    rootStyle.size = { width: 100, height: 100 };

    const root = tree.newLeaf(rootStyle);

    // Replaced element (like img) with aspect ratio
    const childStyle = new Style();
    childStyle.itemIsReplaced = true;
    childStyle.aspectRatio = 2; // width = 2 * height
    childStyle.size = { width: 50, height: "auto" };
    const child = tree.newLeaf(childStyle);

    tree.addChild(root, child);

    tree.computeLayout(root, { width: 100, height: 100 });

    const childLayout = tree.getLayout(child);
    expect(childLayout.width).toBe(50);
    // Height should be computed from aspect ratio: 50 / 2 = 25
    expect(childLayout.height).toBe(25);

    tree.free();
    rootStyle.free();
    childStyle.free();
  });
});
