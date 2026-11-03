# Live Truth Check — 2026-11-03 (v0.1.0, commit 34d3735)

Reviewer pass over the running desktop application, not the test suite. The
point was to answer one question: **does the app the owner launches actually
work with a mouse and keyboard?**

## Method

- Built `cargo build --workspace --release` on the review machine (Arch,
  i5-8250U), then ran `target/release/ruqa-app` from a throwaway vault folder.
- Hosted the window in `Xvfb :99` (1600x1000, window 1280x800) so the owner's
  desktop was not touched.
- Drove it with `xdotool` (XTEST): real pointer motion, button press/release,
  wheel, and keyboard events; captured the X root with ImageMagick `import`.
- Compared frames with `compare -metric AE`; read the screenshots directly.
- Used an isolated `XDG_CONFIG_HOME`/`HOME` with a seeded `recent_files.json` so
  the startup dialog could be exercised on demand.
- Measured idle CPU from `/proc/<pid>/stat` over 15 s and idle frames from the
  app's own exit log.

## What was observed working

| Interaction | Evidence |
|---|---|
| Startup "Recent Boards" dialog renders text, recent entries, New Board button | screenshot 01 |
| Close (`X`), `Escape`, and click-outside all dismiss the dialog | pixel-identical frames after each dismissal |
| `Ctrl+O` reopens the dialog | screenshot 03 |
| `Ctrl+B` sidebar: opaque, "Study Vault" + folder/board names rendered | screenshots 06–07 |
| Folder expand/collapse updates rows and highlights the row | screenshot 07 |
| Clicking a board in the sidebar opens it and renders its shapes | screenshot 10, log `opened whiteboard document` |
| Rectangle tool drag creates a rough rectangle; pen tool draws a stroke | screenshots 12–13 |
| Middle-drag pans; wheel zooms anchored at the pointer | screenshots 15–16 |
| Selection box, drag-move, `Ctrl+Z` undo, copy/paste, Delete | screenshots 28–32 |
| Bottom toolbar highlights the active tool; single-key shortcuts work | screenshots 12–13 |
| `Ctrl+T` light theme; text and chrome invert and stay legible | screenshot 24 |
| Quit guard appears on unsaved exit; Escape cancels; Save writes `untitled.ruqa` | screenshots 21–22 |
| PNG export (`Ctrl+E`) writes a valid 1183x563 RGBA PNG | `identify untitled.png` |
| Arabic + Latin text shapes and saves as UTF-8 | screenshot 23; saved JSON contains Arabic |

## Idle and performance

- Idle: 0 CPU ticks in 15 s, 0 changed pixels; a separate 20 s idle session
  presented 2 frames total (startup + one dismissal). RSS 22.3 MB.
- Cold start: 189 ms process → mapped window; 158 ms to "presenter ready".
- Fixed stress scene (5k shapes, 2k strokes, 500 text at 2560x1440): 225.5 ms
  median. See `docs/product/budget.md`.

## Defects found (filed)

- **#153 (high)**: spaces typed while editing text are silently dropped;
  `foo bar baz` saved as `foobarbaz`. Breaks the bilingual-note use case.
- **#152 (medium)**: `--demo` opens behind the Recent Boards modal.
- **#155 (medium)**: bundled example boards open partly outside the default
  1280x800 window.
- **#160 (high, data loss)**: every new untitled board saves to the same
  `untitled.ruqa`; saving two boards in a row replaces the first file silently.
  Reproduced live: board 1 (rectangle) and board 2 (ellipse) both wrote
  `untitled.ruqa`, with only the second surviving.

## Not verified here (needs manual verification)

- A physical mouse on the owner's real desktop session (XTEST is synthetic).
- Wayland, Windows, and macOS; only X11 was exercised.
- HiDPI/4K scale factors and multi-monitor placement.
- IME-based Arabic input (ibus/fcitx/Windows IME); only direct XTEST characters
  were injected. Tracked as #154.
- Long sessions, autosave races, and crash recovery.

## Gate colour

**RED**: the previously reported "mouse does nothing / no UI text" failures are
gone in the live binary, but #160 loses a user's board on the second untitled
save and #153 silently discards spaces in the product's core text feature.
Both must be fixed and re-verified before a v0.1.0 gate can be called green.
