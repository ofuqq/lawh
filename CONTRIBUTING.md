# Contributing to Lawh

Welcome! Lawh is an open-source, lightweight whiteboard built in Rust. We appreciate your interest in contributing to the project.

## Code of Conduct

All contributors and participants must adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Please report any unacceptable behavior to [ahossam.tech@gmail.com](mailto:ahossam.tech@gmail.com).

## Licensing of Contributions (Inbound = Outbound)

Lawh is dual-licensed under both the **MIT License** and the **Apache License (Version 2.0)**. 

By contributing code, documentation, or other assets to this repository, you agree that your contributions are licensed under this same dual MIT/Apache-2.0 license (inbound = outbound).
- **No CLA**: We do not require a Contributor License Agreement.
- **No DCO**: We do not currently enforce Developer Certificate of Origin (`Signed-off-by`) sign-offs, keeping contribution lightweight for early developers.

## Development Workflow

1. **Find or Open an Issue**: Before submitting non-trivial work, please open an issue to discuss your proposal with the maintainer.
2. **Branch from `main`**: Never commit directly to `main`. Create a descriptive branch (e.g., `feat/rectangle-tool`, `fix/canvas-pan-offset`, `docs/adr-rendering`).
3. **Commit Messages**: Follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` new user-facing functionality
   - `fix:` bug fixes
   - `docs:` documentation updates
   - `chore:` maintenance, dependency bumps, or tool configuration
   - `refactor:` code cleanup without behavior changes
   - `test:` adding or improving tests
4. **Code Quality**:
   - Format with `cargo fmt --check`
   - Run linter with `cargo clippy --workspace --all-targets -- -D warnings`
   - Ensure all tests pass with `cargo test --workspace`
5. **Open a Pull Request**: Submit your PR targeting `main` using the provided [Pull Request Template](.github/PULL_REQUEST_TEMPLATE.md).
   - If AI tools or agents contributed to the PR, disclose this by checking the AI-assisted box in the PR template.

For detailed instructions on local setup and design philosophy, see [docs/development/setup.md](docs/development/setup.md) and [docs/development/workflow.md](docs/development/workflow.md).
