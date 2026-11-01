# ADR 0009: AI-Assisted Development Policy & Hygiene Standards

- **Status**: **Accepted**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Ruqa is built by a solo founder learning Rust, collaborating directly with advanced AI engineering agents.
To ensure the repository remains maintainable, legally compliant, auditable, and of exceptional engineering quality, clear standards must govern AI-assisted contributions.

---

## 2. Decision Outcome

We establish five binding rules for AI-assisted development in the Ruqa project:

### Rule 1: Clean-Room Implementation
- AI agents are strictly prohibited from copying, decompiling, or transcribing code from copyleft projects (e.g. AGPLv3 `drafft-ink`, GPL `Rnote`, `Xournal++`) or proprietary commercial software.
- High-level architectural patterns from open research or public specifications are permitted.
- Any vendored permissive snippet (MIT / Apache-2.0 / BSD) must retain its original copyright notice and be documented in the repository `NOTICE` file.

### Rule 2: Commit Attribution Trailer
- Commits generated or co-authored with AI agents must include a Git trailer identifying the tool and model:
  ```git
  Assisted-by: antigravity:gemini-3.8-flash
  ```
- Do not use `Co-authored-by: ... <email>` for AI agents to avoid spoofing fake GitHub committer identities.

### Rule 3: Source Module Size Cap (~350 LOC)
- To prevent monolithic files that overwhelm AI context windows and degrade human reviewability, Rust source files have a soft ceiling of **~350 lines of code**.
- Large modules must be decomposed into logical submodules with clear, concrete interfaces.

### Rule 4: Verification Before Proposing
- Agents must verify crate existence, exact versions, and API signatures via `cargo check`, `cargo doc`, or registry inspection before introducing new code.
- Speculative trait hierarchies and premature generic abstractions are forbidden.

### Rule 5: Explanatory Mode for the Solo Founder
- When introducing idiomatic Rust patterns (lifetimes, borrowing, `Option`/`Result`, RAII, pattern matching), agents must provide clear, concise educational explanations contrasting with higher-level languages.
