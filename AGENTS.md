# AI Agent Guidelines for Mirsam

> If `.local/AI_BRIEF.md` exists in your workspace, read it first for runtime context.

## 1. Project Summary
Mirsam is an open-source, lightweight desktop whiteboard built in Rust. It delivers an Excalidraw-like hand-drawn aesthetic, infinite canvas, dark theme by default, and mouse-first usability for studying and visual explanation.

## 2. Repository Map
- `crates/mirsam-core`: Data models, geometry, scene graph, undo/redo history, and `.mirsam` serialization.
- `crates/mirsam-render`: 2D rasterization, roughr conversion, softbuffer presentation, and text layout.
- `crates/mirsam-app`: Native OS windowing (`winit`), mouse input dispatch, and UI chrome.
- `docs/`: Canonical specifications:
  - `docs/product/`: Vision, principles, users, competitive analysis, MVP scope, and kill criteria.
  - `docs/architecture/`: Technical design, data models, rendering, text/RTL, and file format specs.
  - `docs/adr/`: Architectural Decision Records (ADRs 0001–0009).
  - `docs/development/`: Workflow, setup, AI collaboration policy, and glossary.
- `.github/`: Pinned CI workflows, issue/PR templates, and rulesets.
- `.local/`: Gitignored local files for the repository owner (Arabic guide, agent briefs, prompt templates).

## 3. Build, Test, and Lint Commands
Run these exact commands before opening or updating any PR:
```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check licenses bans sources
```

## 4. Code Style & Quality Rules
- Keep code idiomatic: prefer pattern matching, explicit error types (`thiserror`), and iterator combinators.
- Soft cap: Keep Rust source modules under **~350 LOC**. Decompose larger files into focused submodules.
- Verify crate APIs on docs.rs or via `cargo doc` before introducing code; never invent APIs.
- Never write speculative trait abstractions; keep interfaces concrete until multiple implementations exist.
- Unit tests live alongside code (`#[cfg(test)] mod tests`); integration tests live in `tests/`.
- Clean-Room Guarantee: Never copy, decompile, or adapt code from copyleft projects (AGPL `drafft-ink`, GPL `Rnote`).

## 5. Git & PR Rules
- Never push directly to `main`. Never force-push or delete branches.
- Branch naming: `feat/<name>`, `fix/<name>`, `chore/<name>`, `docs/<name>`, `refactor/<name>`, `spike/<name>`.
- Commit messages: Use Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`, `spike:`).
- AI Attribution: Use `Assisted-by: <tool>:<model>` trailer in commit messages (never `Co-authored-by`).
- Keep PRs small, focused, and reviewable (target: < 400 lines of change).
- Check the AI-assisted box in `.github/PULL_REQUEST_TEMPLATE.md` and label with `workflow: ai-assisted`.

## 6. Strict "Never Do" List
1. **Never add a new crate/dependency** to `Cargo.toml` without an existing issue or approved ADR.
2. **Never use `unsafe` code** without written justification, memory safety proof, and prior approval.
3. **Never modify or overwrite an Accepted ADR** in `docs/adr/` (create a superseding ADR instead).
4. **Never merge a pull request or modify rulesets/protection settings** directly unless explicitly instructed in autonomous runs.
5. **Never commit secrets, credentials, or personal tokens** under any circumstance.

## 7. Teaching Mode (Explaining Rust to the Owner)
The repository owner is a solo founder learning Rust. When presenting code or architectural changes:
- Explain the *why* behind Rust idioms (e.g., ownership, borrowing, lifetimes, `Option`/`Result`).
- Contrast Rust patterns with higher-level languages (e.g., why clone vs borrow, why enum vs inheritance).
- Keep code examples small, focused, and clearly commented.
- Proactively highlight subtle traps (e.g., borrow checker conflicts, allocation overhead, re-entrancy).
