## Summary
<!-- Briefly explain the motivation, context, and what changes are introduced. -->

## Linked Issue
<!-- Link the associated issue (e.g. Closes #123, Fixes #456). -->
Closes #

## Changes Introduced
- 

## Test Evidence
<!-- Provide terminal output from cargo test, screenshots, or benchmark evidence. -->
```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check licenses bans sources
```

## Documentation
- [ ] Documentation updated in `docs/` or `README.md` (if applicable)
- [ ] ADR updated or created in `docs/adr/` (if architectural changes were made)

## AI Assistance & Attribution
- [ ] **AI-Assisted**: This pull request was created or assisted by an AI agent (e.g. Antigravity, Claude, Gemini, Copilot).
  - *Tool & Model*: 
  - *Commit Trailer*: `Assisted-by: <tool>:<model>` included on all relevant commits.

## Contributor & Reviewer Checklist
- [ ] Code follows formatting guidelines (`cargo fmt --all --check`)
- [ ] Code passes all linter checks (`cargo clippy --workspace --all-targets -- -D warnings`)
- [ ] All tests pass cleanly (`cargo test --workspace`)
- [ ] Licenses and dependencies verified (`cargo deny check licenses bans sources`)
- [ ] Clean-room compliance: zero code copied or translated from copyleft projects (AGPL/GPL)
- [ ] Modular file size respected: modules kept under ~350 LOC
- [ ] No new dependencies added to `Cargo.toml` without an approved issue or ADR
- [ ] No `unsafe` code introduced without written justification and memory safety proof
- [ ] No secrets, tokens, or personal credentials included
