# Product Vision: Ruqa

## 1. Problem Statement

Modern visual thinking and whiteboard applications have split into two flawed extremes:

1. **Bloated Web/Electron Wrappers**: Tools like Goodnotes for Windows consume ~1.3 GB of idle RAM with noticeable stroke latency (reported by Noteastic Blog, May 2026, not independently verified). General web whiteboards like Excalidraw suffer performance degradation beyond 5,000 elements (GitHub Issue #7280), lack multi-board folder organization, and exhibit recurring right-to-left (RTL) bugs and missing Arabic sketch fonts (GitHub Issues #8394, #12404).
2. **Stylus-Centric Desktop Notebooks**: Tools like Rnote and Lorien target digital graphics tablets (Wacom, Surface, iPad) and GNOME desktop conventions. They lack mouse-first diagramming ergonomics, geometric box connectors, and in Lorien's case, lack even a basic text tool.

Laptop-based students and self-taught developers who study with a keyboard and mouse are stranded without a fast, native tool to sketch systems, draw algorithms, and organize bilingual technical notes.

---

## 2. The Ruqa Mission

**Ruqa** (Arabic: Ruq'ah, meaning pencil, drawing instrument, or artist's studio) is a fast, lightweight, open-source desktop whiteboard and study scratchpad written in native Rust by [Ofuq](https://github.com/ofuqq).

Ruqa combines the approachable hand-drawn aesthetic of Excalidraw with the local-first file organization of a developer study vault. It boots in under 100 milliseconds, idles under 80 MB of RAM, works 100% offline, offers first-class bilingual Arabic/Latin typography out of the box, and is optimized for the two-button mouse and keyboard.

---

## 3. Core Pillars

- **Low-RAM Native Speed**: Built with a lean native Rust runtime (tiny-skia on CPU + softbuffer). Zero Chromium/Electron runtime bloat, zero background telemetry.
- **The Local "Study Vault"**: Boards are saved as lightweight, human-readable versioned JSON files (`.ruqa`) in standard OS folders, accessible via a collapsible sidebar and cross-board search (`Ctrl+P`).
- **First-Class Bilingual Typography**: Out-of-the-box Arabic and Latin font pairing (Aref Ruqaa for headings, Vazirmatn for body notes) rendered with proper bidirectional shaping via Linebender's `parley`.
- **Mouse & Keyboard Ergonomics**: Velocity-simulated stroke tapering turns mouse movements into legible handwriting. Single-key tool shortcuts (R, O, L, A, P, E) enable rapid diagramming without stylus hardware.
- **Approachable Sketch Aesthetic**: Hand-drawn contours (via `roughr`) eliminate the intimidation of visual perfection, encouraging quick visual self-explanation.
