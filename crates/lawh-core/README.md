# lawh-core

The foundation crate for the **Lawh** whiteboard.

Contains:
- 2D Canvas geometry, affine transforms, and viewport calculations.
- Element types, attributes, and styling models.
- Undo / Redo command history stack.
- `.lawh` (v1) JSON serialization and deserialization.

Designed to have zero OS window or graphics device dependencies so it can be tested completely headlessly.
