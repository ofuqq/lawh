# Contributing to Mirsam

Welcome! Mirsam is an open-source, lightweight whiteboard and study scratchpad built in Rust. We appreciate your interest in contributing to the project.

---

## Code of Conduct

All contributors and participants must adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Please report any unacceptable behavior to [ahossam.tech@gmail.com](mailto:ahossam.tech@gmail.com). Note that this contact address is public.

---

## Licensing of Contributions (Inbound = Outbound)

Mirsam is licensed exclusively under the **Apache License, Version 2.0**.

By contributing code, documentation, or other assets to this repository, you agree that your contributions are licensed under the Apache License, Version 2.0 (inbound = outbound) without additional terms or conditions.
- **Attribution & Notice**: Third-party permissive code (MIT/Apache/BSD) must retain copyright notices and be recorded in [NOTICE](NOTICE).
- **Clean-Room Requirement**: Code copied, decompiled, or adapted from copyleft projects (e.g., AGPLv3 `drafft-ink`, GPL `Rnote`, `Xournal++`) is strictly prohibited.
- **No CLA**: We do not require a Contributor License Agreement.
- **No DCO**: We do not currently enforce Developer Certificate of Origin (`Signed-off-by`) sign-offs, keeping contribution lightweight for early developers.

---

## Development Workflow

1. **Find or Open an Issue**: Before submitting non-trivial work, please open an issue to discuss your proposal with the maintainer.
2. **Branch from `main`**: Never commit directly to `main`. Create a descriptive branch (e.g., `feat/rectangle-tool`, `fix/canvas-pan-offset`, `docs/adr-rendering`, `spike/parley-shaping`).
3. **Commit Messages**: Follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` new user-facing functionality
   - `fix:` bug fixes
   - `docs:` documentation updates
   - `chore:` maintenance, dependency bumps, or tool configuration
   - `refactor:` code cleanup without behavior changes
   - `test:` adding or improving tests
   - `spike:` exploratory prototype code
4. **AI Assistance Disclosure**:
   - If AI tools or agents generated or assisted your commit, add the trailer: `Assisted-by: <tool>:<model>` (never `Co-authored-by`).
   - Check the AI-assisted checkbox in the PR template.
5. **Code Quality Verification**:
   - Format with `cargo fmt --all --check`
   - Run linter with `cargo clippy --workspace --all-targets -- -D warnings`
   - Ensure all tests pass with `cargo test --workspace`
   - Verify licenses and bans with `cargo deny check licenses bans sources`
6. **Open a Pull Request**: Submit your PR targeting `main` using the provided [Pull Request Template](.github/PULL_REQUEST_TEMPLATE.md).

For detailed instructions on local setup and design philosophy, see [docs/development/setup.md](docs/development/setup.md) and [docs/development/workflow.md](docs/development/workflow.md).
