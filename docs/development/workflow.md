# Development Workflow & Review Cadence

Lawh follows a disciplined git and review workflow to maintain high code quality, reproducible builds, and linear git history.

---

## 1. Golden Rules of Development

1. **Never Commit Directly to `main`**: All work—whether written by the founder, community contributors, or AI agents—must be developed on a feature branch and merged via a Pull Request.
2. **Never Force-Push to Protected Branches**: Linear history is maintained via squash merging.
3. **Green CI Required**: Pull requests cannot be merged unless all formatting, clippy linter, and unit test checks pass.
4. **Keep Commits Small & Reviewable**: Aim for small, single-purpose commits structured around Conventional Commits.

---

## 2. Branching Strategy

Branch names must follow a standard prefix:
- `feat/<name>`: New capabilities or tools (e.g., `feat/ellipse-tool`)
- `fix/<name>`: Defect repairs and bug fixes (e.g., `fix/viewport-zoom-offset`)
- `chore/<name>`: Tooling, dependency updates, and maintenance (e.g., `chore/ci-caching`)
- `docs/<name>`: Documentation additions and ADR updates (e.g., `docs/adr-vello-benchmark`)
- `refactor/<name>`: Internal structure cleanup with zero behavior change

```bash
# Create and switch to a new branch from main
git checkout main
git pull origin main
git checkout -b feat/smooth-pen-stroke
```

---

## 3. Conventional Commits Standard

Every commit message must follow the [Conventional Commits](https://www.conventionalcommits.org/) format:

```text
<type>(<optional scope>): <imperative subject>

[optional body explaining context and trade-offs]

[optional footer / issue references]
```

### Commit Types
- `feat`: A new feature for the user
- `fix`: A bug fix
- `docs`: Documentation only changes
- `chore`: Repository configuration, CI, or dependency updates
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `test`: Adding or correcting tests

*Example*:
```text
feat(pen): add velocity-based stroke width modulation

Calculate instantaneous velocity between consecutive mouse points
and adjust stroke width dynamically to simulate pen pressure.

Closes #14
```

---

## 4. Pull Request & Review Process

1. **Self-Review**: Run `cargo fmt --check`, `cargo clippy`, and `cargo test` locally.
2. **Submit PR**: Open a pull request against `main` using `.github/PULL_REQUEST_TEMPLATE.md`.
3. **Disclose AI Assistance**: If AI tools (Claude, Gemini, Copilot, ChatGPT) generated or assisted the PR, check the AI-assisted checkbox and attach the `workflow: ai-assisted` label.
4. **Owner Review**: The repository owner acts as gatekeeper, reviewing architectural fit, test evidence, and documentation.
5. **Squash & Merge**: Merged exclusively as a single squash commit into `main`. The feature branch is automatically deleted upon merge.
