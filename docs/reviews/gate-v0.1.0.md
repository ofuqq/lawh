# Gate Report — v0.1.0 (release candidate)

**Date**: 2026-11-03
**Gate for**: v0.1.0-rc1 (`v0.1.0-rc1` tag, code `34d3735`)
**Reviewer colour**: **RED**

## (a) What was built

A native Rust desktop whiteboard: `ruqa-core` (documents, geometry, history,
JSON), `ruqa-render` (tiny-skia / roughr / parley CPU rasterization), and
`ruqa-app` (winit 0.30 window, mouse/keyboard dispatch, custom chrome). The live
binary draws rough shapes and pen strokes, selects/moves/deletes, pans and
zooms, opens a Study Vault sidebar and a recent-boards dialog, edits bilingual
text, exports PNG, guards unsaved exits, and saves versioned `.ruqa` JSON.
Arabic + Latin shaping works and round-trips through the file.

## (b) Five steps the owner can run in 15 minutes

Run these on your normal desktop, from the repository root:

1. **Chrome and text.** `cargo run --release -p ruqa-app`. Dismiss the Recent
   Boards dialog with `X`, then reopen with `Ctrl+O` and close with `Esc`.
   Every label, board name, and button must be readable.
2. **Draw, pan, zoom.** Press `R`, drag a rectangle; press `P`, draw a stroke;
   middle-drag to pan; wheel to zoom. Shapes must follow the cursor and stay
   sharp.
3. **Bilingual note.** Press `T`, click the canvas, type
   `الخوارزمية Dijkstra تعمل في O(V log V)` **with spaces**, press Enter.
   Expected: words stay separated. **Currently they do not — see RED reason 1.**
4. **Vault.** `Ctrl+B`, expand a folder, click a `.ruqa` board. The board should
   open and the sidebar labels should be legible.
5. **Two new boards.** `Ctrl+O` → New Board, draw something, `Ctrl+S`. Repeat
   with a second new board. Expected: two separate files. **Currently the
   second save overwrites the first — see RED reason 2.**

## (c) Provisional decisions you can veto

- Budget limits in `docs/product/budget.md` (dependency counts, 7 MiB binary,
  250 ms cold-start guard, 0 % idle, 260 ms stress-scene guard). These are
  reviewer defaults, not product decisions.
- Untitled-save fix should pick the next free filename rather than add a native
  file-dialog dependency. This is the safe default; a dialog would need your
  approval as a new dependency.

## (d) Decisions that need the owner

None are blocking this gate. The two RED items are ordinary bug fixes. If you
want a real "Save As" dialog (native file picker) that is a new dependency and
therefore your call; the reviewer recommendation is to defer it.

## (e) Budget numbers vs limits

| Metric | Measured | Limit | Status |
|---|---|---|---|
| Direct deps (core/render/app) | 4 / 6 / 6 | 6 / 8 / 8 | OK |
| Unique crates | 176 | 200 | OK |
| Release binary | 5.62 MiB | 7.0 MiB | OK |
| Cold start (process → window) | 189 ms | 250 ms | OK (the "< 150 ms" claim is not met and should not be repeated) |
| Idle CPU | 0.00 % | 0.5 % | OK |
| Idle frames / pixels | 2 startup frames, 0 changed pixels in 15 s | ≤ 3 / 0 | OK |
| Stress scene, 2560x1440, 7,500 elements | 225 ms median | 260 ms | OK as a stress ceiling (~4.4 FPS; not the normal-board 60 FPS target) |
| Longest source module | 349 LOC | 400 LOC | OK |
| Tests | 298 passing | ≥ 298 | OK |
| `unsafe` | 0 | 0 | OK |

`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, and `cargo deny check licenses bans sources` all pass
on this commit. `cargo tree -d` shows no duplicate versions of `tiny-skia`,
`winit`, `softbuffer`, or `parley`.

## (f) Gate colour: RED

Two reproduced defects discard user content in the live binary:

1. **#153 — spaces are dropped while typing text.** `foo bar baz` saves as
   `foobarbaz`; Arabic + English notes collapse into a single run. Text is the
   product's differentiator, so this is a release blocker.
2. **#160 — untitled saves overwrite each other.** Two new boards saved in a
   row both write `untitled.ruqa`; the first file is replaced without a prompt.
   This is silent data loss.

Secondary (fix, but not gate-blocking): #152 (`--demo` behind the modal), #155
(bundled boards open partly off-screen), #159 (reproducible budget harness).

The gate reopens as **GREEN** when #153 and #160 have live evidence of being
fixed and the truth check is repeated on the new commit.
