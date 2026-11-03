# Agent Working Agreement

Ruqa is developed by two independent agents working in parallel on the same
repository:

- **Builder** — one or more AI agents that implement product code in their own
  branch/worktree and open pull requests.
- **Reviewer** — an independent agent that verifies, reviews, tests, decides,
  researches, and reports. The reviewer does not write product code.

This document is the contract between them. Either agent can be started with a
single sentence ("continue per `docs/development/agent-protocol.md`").

## 1. Non-Negotiable Decisions

These are settled and are never reopened without the owner:

- Product name: **Ruqa**. License: **Apache-2.0** only (`cargo-deny` policy).
- **Clean-room**: no code, structure, or decompilation from AGPL/GPL projects
  (for example `drafft-ink`, `Rnote`).
- Native Rust, exactly three crates: `ruqa-core` (no UI/GPU dependencies),
  `ruqa-render` (no windowing), `ruqa-app` (windowing only).
- Stack: `winit 0.30.x`, `softbuffer`, `tiny-skia` CPU rasterization, `roughr`
  core, `parley`.
- Own versioned JSON document format; a vault is a plain folder.
- Out of scope: collaboration, mobile, PDF editing, in-app/cloud AI, stylus
  pressure, cloud sync, accounts.
- Product goal: **simple and strong**, never big.
- Tracked files are English-only. No stable release is published by an agent.
  Repository settings and rulesets are never touched by an agent.

Anything that would change one of the above is a `needs: owner` escalation.

## 2. Labels

| Label | Meaning |
|---|---|
| `review: needed` | Builder requests a review verdict. |
| `review: ok` | Reviewer approved; the PR may be merged. |
| `review: changes` | Reviewer found blockers; do not merge. |
| `merged-without-review` | Small safe PR merged by the builder under §4. |
| `decision: provisional` | A reversible default chosen by the reviewer; owner may veto. |
| `needs: owner` | Requires an owner decision or manual verification the agent cannot do. |
| `type: research` | Research question whose output is `docs/research/<topic>.md`. |
| `priority: high` | Urgent defect (data loss, crash, unreachable UI, broken main). |

## 3. Review Gate

1. The builder opens a PR against `main` and adds `review: needed`.
2. The reviewer checks out the branch in a temporary worktree and runs
   `cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`, and `cargo tree -d`.
3. The reviewer reads the diff cold and checks: scope against the linked issue,
   error paths, panics on user input, `unsafe`, licenses, clean-room,
   module size (soft cap ~350 LOC), docs/CHANGELOG, and that user-visible
   behavior is covered by a test that drives the real application state through
   the same update/render path as the live window.
4. The verdict is a PR review comment prefixed `Reviewer:` plus exactly one of
   `review: ok` or `review: changes`.
5. `review: changes` is reserved for real blockers: data loss, panic on user
   input, UI not reachable or not drawn in the live app, budget regression,
   license or clean-room problem, leaked secret, broken `main`, or a missing
   test for user-visible behavior. Style opinions become issues instead.
6. At most **two discussion rounds** per PR. After the second round the reviewer
   decides and states why.

## 4. Merge Rules

- The builder merges only after `review: ok`.
- Exception: a **small safe PR** (docs/tests only, or under 150 changed lines,
  no `unsafe`, no new dependency) waiting more than 12 hours may be merged by
  the builder, labeled `merged-without-review`. The reviewer then reviews it
  after the fact and opens issues for problems.
- The reviewer never edits builder branches, never merges builder PRs, and never
  closes an issue the builder is actively working on.
- The reviewer may reopen issues that are not truly done, close
  duplicates/obsolete issues as "not planned" with a reason, and fix labels and
  milestones.
- Reviewer-authored docs/process PRs may be self-merged after CI is green.
- Nobody pushes to `main`, force-pushes (except `--force-with-lease` on their own
  `review/*` branch), or rewrites the builder's history.

## 5. Decision Classes

- **Reversible** (default): the reviewer decides, records the decision in the
  issue or ADR, and moves on.
- **Hard to reverse** (file schema, large dependency, architecture): choose the
  safe default, label `decision: provisional`, write the alternative and the
  reason, and list it in the pinned Coordination issue. The builder proceeds on
  the default; the owner can veto.
- **Owner-only**: name, license, crate count, scope, release publication,
  repo settings, and any hard-to-reverse choice the owner has reserved.

## 6. Budget

Performance and simplicity are tracked in `docs/product/budget.md`. A change that
exceeds a budget limit needs an ADR with measured evidence. Idle behavior is part
of the budget: frames are rendered only on input, animation, or caret blink.

## 7. Research Desk

Research questions carry `type: research`. The reviewer writes
`docs/research/<topic>.md` with sources and dates, separating **facts** from
**inferences** and marking unverified claims. Numbers and quotes are never
invented. Each note ends with a recommendation.

## 8. Cycle and State

The reviewer runs a cycle of: PR queue → `main` health → truth check of recently
closed work → issue grooming → research desk → decisions → gate report. Progress
is kept in `.local/REVIEW_PROGRESS.md` (untracked) so a restart only needs the
word "continue". A cycle ends when two consecutive polls show nothing to do, or
after a gate report.

## 9. Escalation

Stop and escalate with `needs: owner` when: `main` is broken or CI is unfixable,
a license or clean-room violation appears, a secret is leaked, a data-loss bug is
found, a fundamental blocker appears, or a GitHub permission is missing. State
the evidence and a safe alternative instead of choosing a contingency.

## 10. Evidence Standard

A green test suite is not proof that the live application works. Claims about
user-visible behavior must be backed by executable evidence: a real (or
Xvfb-hosted) window driven with synthetic OS mouse/keyboard events, screenshots
inspected, and numbers measured. Anything not observed is reported as "needs
manual verification", never as passing.
