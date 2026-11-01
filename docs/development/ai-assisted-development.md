# AI-Assisted Development Policy & Guidelines

Lawh is intentionally bootstrapped and built using autonomous AI coding agents in partnership with a human founder. To maintain rigorous code quality, architectural consistency, and security, all AI-generated code must adhere to these policies.

---

## 1. Core Principles for AI Agents

1. **Human in the Loop**: AI agents propose; the human owner reviews, tests, and decides. No AI agent may merge code, close issues unilaterally, or bypass review checks.
2. **Mandatory Disclosure**: Any pull request containing code or documentation written with AI assistance must check the AI-assisted box in the PR template and carry the `workflow: ai-assisted` label.
3. **No Unreviewed Dependencies**: AI agents are strictly forbidden from adding third-party crates to `Cargo.toml` without an approved issue or ADR. Every new dependency increases compilation time, binary size, and supply chain attack surface.
4. **Zero Unjustified `unsafe`**: Rust’s memory safety is our primary advantage. AI agents must never introduce `unsafe` blocks without written proof of necessity and prior approval from the maintainer.
5. **Preserve Accepted ADRs**: Decisions marked as "Accepted" in `docs/adr/` are immutable contracts. If an AI agent discovers a superior alternative, it must propose a new superseding ADR, not overwrite historical records.

---

## 2. Review Checklist for AI-Generated PRs

Before submitting a PR, the agent must verify:
- [ ] Code compiles cleanly with zero warnings (`cargo clippy --workspace --all-targets -- -D warnings`).
- [ ] Code is formatted strictly according to workspace rustfmt rules (`cargo fmt --check`).
- [ ] All unit and integration tests pass cleanly (`cargo test --workspace`).
- [ ] Every new function, struct, and public method has concise doc comments explaining parameters and failure modes.
- [ ] The PR description provides step-by-step instructions for the maintainer to review and verify the changes in under 15 minutes.

---

## 3. Teaching Mode: Explaining Rust to the Founder

The project founder is an experienced technologist but a beginner in Rust. AI agents must operate in **Teaching Mode**:
- **Deconstruct Idioms**: Explain why idiomatic Rust patterns (e.g., using `Option::map` vs nested `match`, or using borrowing vs `.clone()`) were chosen.
- **Explain Compiler Diagnostics**: If explaining a bug fix, explain why the borrow checker or lifetime rules flagged the original code.
- **Highlight Hidden Traps**: Proactively note edge cases like accidental memory allocations in tight loops or integer overflow risks.
