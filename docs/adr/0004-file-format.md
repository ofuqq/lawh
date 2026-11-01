# ADR 0004: Versioned JSON Document Format (.mirsam)

- **Status**: **Accepted**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Mirsam documents must be persisted locally on disk. We require a document format that is durable, forward-compatible, human-readable, easy to track in version control (Git), and straightforward to deserialize into Rust domain structs.

---

## 2. Decision Drivers
- **Human-Readability & Git Diffability**: Users should be able to view, diff, and merge diagrams in Git repositories or text editors.
- **Forward & Backward Compatibility**: Old versions of the application should not crash when opening documents created by newer versions.
- **Atomic Persistence**: Zero data loss during unexpected application crashes or power cuts.
- **Interoperability**: Ability to export to open standards like JSON Canvas 1.0 (`.canvas`).

---

## 3. Decision Outcome & Rationale
Chosen option: **Own Versioned JSON Format (`.mirsam`) with Atomic Save**.

### Format Specification
1. **Root Magic & Version Key**: Every document begins with:
   ```json
   {
     "format": "mirsam",
     "schema_version": 1,
     "app_version": "0.1.0"
   }
   ```
2. **Vault-as-a-Folder**: A study vault is simply a standard operating system folder containing `.mirsam` files and subdirectories. Zero opaque single-file databases (like SQLite or monolithic ZIPs).
3. **Atomic Autosave**: File writing always writes to a temporary sibling file (`.name.mirsam.tmp`) before calling an atomic filesystem rename, ensuring zero partial-write corruption.
4. **JSON Canvas (.canvas) as Export Target**: JSON Canvas 1.0 cannot express freehand stroke pressure arrays, rough jitter seeds, or custom style properties. Therefore, Mirsam uses its own format as native storage and provides 1-click export to `.canvas` in v0.2 for Obsidian interoperability.
