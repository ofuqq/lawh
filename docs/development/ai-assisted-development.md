# AI-Assisted Development Policy & Guidelines

Ruqa is built using AI coding agents in partnership with a human solo founder. To maintain rigorous code quality, architectural consistency, clean licensing, and auditability, all AI-generated code must adhere strictly to these policies.

---

## 1. Core Principles for AI Agents

1. **Human in the Loop**: AI agents propose; the human owner reviews, tests, and decides. No AI agent may merge code into `main`, close issues unilaterally, or bypass review checks unless explicitly directed in autonomous bootstrap runs.
2. **Clean-Room Guarantee**: AI agents are strictly forbidden from copying, transpiling, or adapting source code from copyleft projects (e.g., AGPLv3 `drafft-ink`, GPL `Rnote`, `Xournal++`) or proprietary commercial tools. Permissive code (MIT/Apache-2.0) may only be vendored with explicit attribution in `NOTICE`.
3. **Commit Attribution Trailer**: All commits generated with AI assistance must include the `Assisted-by: <tool>:<model>` trailer in the commit message (never `Co-authored-by`).
4. **No Unreviewed Dependencies**: AI agents are strictly forbidden from adding third-party crates to `Cargo.toml` without an approved issue or ADR. Every new dependency increases compilation time, binary size, and supply chain attack surface.
5. **Zero Unjustified `unsafe`**: Rust’s memory safety is a foundational principle. AI agents must never introduce `unsafe` blocks without written proof of memory safety necessity and explicit owner approval.
6. **Preserve Accepted ADRs**: Decisions marked as "Accepted" in `docs/adr/` are immutable contracts. If an AI agent discovers a superior alternative, it must propose a new superseding ADR, not overwrite historical records.
7. **Module Size Ceiling (~350 LOC)**: Keep source code modular. No single `.rs` file should exceed ~350 lines of code; decompose larger components into focused submodules.

---

## 2. Review Checklist for AI-Generated PRs

Before submitting a PR, the agent must verify:
- [ ] Code compiles cleanly with zero warnings (`cargo clippy --workspace --all-targets -- -D warnings`).
- [ ] Code is formatted strictly according to workspace rustfmt rules (`cargo fmt --all --check`).
- [ ] All unit and integration tests pass cleanly (`cargo test --workspace`).
- [ ] Dependency and license checks pass (`cargo deny check licenses bans sources`).
- [ ] Every new function, struct, and public method has concise doc comments explaining parameters and failure modes.
- [ ] The PR description provides step-by-step instructions for the maintainer to review and verify the changes in under 15 minutes.
- [ ] Commit messages carry the `Assisted-by:` trailer.

---

## 3. Teaching Mode: Explaining Rust to the Founder

The project founder is an experienced technologist but learning Rust. AI agents must operate in **Teaching Mode**:
- **Deconstruct Idioms**: Explain why idiomatic Rust patterns (e.g., using `Option::map` vs nested `match`, or using borrowing vs `.clone()`) were chosen.
- **Explain Compiler Diagnostics**: If explaining a bug fix, explain why the borrow checker or lifetime rules flagged the original code.
- **Highlight Hidden Traps**: Proactively note edge cases like accidental heap allocations in tight render loops or integer overflow risks.
