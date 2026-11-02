# Ruqa Sample Study Boards

This directory contains three hand-crafted `.ruqa` whiteboard files you can open in Ruqa
(`File → Open` or drag-and-drop) to explore the application and study classic computer-science topics.

---

## Boards

### `dijkstra-shortest-path.ruqa`
A weighted **graph traversal diagram** illustrating Dijkstra's shortest-path algorithm.
- Source node (blue, filled) and sink node (green, filled) are visually distinct.
- Intermediate nodes are drawn with rough ellipses and labeled manually.
- Weighted edges are rendered as arrows; the shortest-path is highlighted in amber (`#FFD166`).

### `distributed-systems-cache.ruqa`
A **microservice architecture** diagram showing a cache-aside eviction flow.
- Client services (blue, semi-transparent rectangles) talk to a central cache (amber hachure).
- Cache-miss path (red diamond) routes requests to the backing data store (green).
- A purple lavender bypass arrow shows the read-through fallback.

### `binary-search-tree.ruqa`
A **binary search tree** diagram drawn with rough ellipses and connecting lines.
- Root node in amber at the top; left/right subtrees in cyan.
- Leaf nodes in green.
- All edges use rough lines with the default 1.2 roughness factor.

---

## How to use in tests

The boards are parsed in `crates/ruqa-core/tests/` to verify the `.ruqa` format round-trips correctly:

```bash
cargo test --package ruqa-core example_boards
```

These files serve as golden-image sources for any future screenshot / regression test tooling.

---

## Contributing a board

1. Create your `.ruqa` file with any text editor (it is plain JSON).
2. Validate it parses: `cargo test --package ruqa-core`.
3. Open a PR with `area: docs` and `type: chore` labels.
