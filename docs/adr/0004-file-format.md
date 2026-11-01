# ADR 0004: Versioned JSON Document Format (.lawh)

- **Status**: **Proposed**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Lawh documents must be persisted locally on disk. We need a document format that is durable, forward-compatible, human-readable, easy to track in version control (Git), and straightforward to deserialize into Rust domain structs.

---

## 2. Decision Drivers
- **Human-Readability & Git Diffability**: Users should be able to view, diff, and merge diagrams in Git repositories or text editors.
- **Forward & Backward Compatibility**: Old versions of the application should not crash when opening documents created by newer versions.
- **Simplicity**: Zero custom binary parsing libraries; reliance on well-tested, standard serialization libraries (`serde_json`).
- **Extensibility**: Ability to introduce new shape types and styling attributes without breaking existing boards.

---

## 3. Considered Options
- **Option 1**: Versioned Plaintext JSON (`.lawh`) (Human-readable, diffable, standard schema).
- **Option 2**: Binary Serialization (MessagePack, CBOR, or Bincode) (Smaller file size, faster parsing, but opaque binary blobs unsuitable for Git diffs).
- **Option 3**: Embedded SQLite Database (ACID transactions, single-file container, but overkill for whiteboard documents and hostile to Git merges).
- **Option 4**: Zip Container with JSON + Assets (Matches `.docx` or Excalidraw exports, but adds container overhead for early milestones).

---

## 4. Decision Outcome & Rationale
Proposed option: **Option 1 (Versioned Plaintext JSON with `.lawh` extension)**.

### Rationale
- **Simplicity & Tooling**: Every language and editor understands JSON. Developers can write scripts to generate or transform `.lawh` diagrams automatically.
- **Git Friendly**: Text diffs in GitHub and Git work out of the box, aligning perfectly with developer study vaults and repositories.
- **Forward Compatibility Rules**:
  - Root `version: 1` field establishes explicit schema generations.
  - Rust structs ignore unknown fields by default during deserialization.
  - If document size becomes an issue in future milestones (e.g., boards with 50,000+ points), transparent zstd/gzip compression can be introduced under a `.lawhz` extension.
