# AI Agent Guidelines for Lawh

> If `.local/AI_BRIEF.md` exists in your workspace, read it first for runtime context.

## 1. Project Summary
Lawh is an open-source, lightweight whiteboard built in Rust. It delivers an Excalidraw-like hand-drawn aesthetic, infinite canvas, dark theme by default, and mouse-first usability for studying and visual explanation.

## 2. Repository Map
- `crates/lawh-core`: Data models, geometry, scene graph, undo/redo history, and `.lawh` serialization.
- `crates/`: Future crates for rendering (`lawh-render`) and windowing/app runtime (`lawh-app`).
- `docs/`: Canonical specifications. Read before changing architecture or data models:
  - `docs/product/`: Vision, principles, competitive positioning, and MVP requirements.
  - `docs/architecture/`: Technical design, data models, rendering, and file format specs.
  - `docs/adr/`: Architectural Decision Records.
  - `docs/development/`: Workflow, setup, and AI collaboration policy.
- `.github/`: Pinned CI workflows, issue/PR templates, and rulesets.
- `.local/`: Gitignored local files for the repository owner (Arabic guide, agent briefs, prompt templates).

## 3. Build, Test, and Lint Commands
Run these exact commands before opening or updating any PR:
```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## 4. Code Style & Testing Rules
- Follow standard Rust formatting rules (`rustfmt.toml`).
- Keep code idiomatic: prefer pattern matching, explicit error types (`thiserror`), and iterator combinators.
- Never write speculative trait abstractions; keep interfaces concrete until multiple implementations exist.
- Unit tests live alongside code in modules (`#[cfg(test)] mod tests`); integration tests live in `tests/`.
- Every bug fix must include a regression test. Every data model change must test JSON serialization round-trips.

## 5. Git & PR Rules
- Never push directly to `main`. Never merge PRs. Never force-push or delete branches.
- Branch naming: `feat/<name>`, `fix/<name>`, `chore/<name>`, `docs/<name>`, `refactor/<name>`.
- Commit messages: Use Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`).
- Keep PRs small, focused, and reviewable (target: < 400 lines of change).
- Label PRs with `workflow: ai-assisted` and mark the AI checkbox in `.github/PULL_REQUEST_TEMPLATE.md`.

## 6. Strict "Never Do" List
1. **Never add a new crate/dependency** to `Cargo.toml` without an existing issue or approved ADR.
2. **Never use `unsafe` code** without written justification, memory safety proof, and prior approval.
3. **Never modify or overwrite an Accepted ADR** in `docs/adr/` (create a superseding ADR instead).
4. **Never merge a pull request or modify rulesets/protection settings** directly.
5. **Never commit secrets, credentials, or personal tokens** under any circumstance.

## 7. Teaching Mode (Explaining Rust to the Owner)
The repository owner is a solo founder learning Rust. When presenting code or architectural changes:
- Explain the *why* behind Rust idioms (e.g., ownership, borrowing, lifetimes, `Option`/`Result`).
- Contrast Rust patterns with higher-level languages (e.g., why clone vs borrow, why enum vs inheritance).
- Keep code examples small, focused, and clearly commented.
- Proactively highlight subtle traps (e.g., borrow checker conflicts, allocation overhead, re-entrancy).
