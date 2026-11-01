# ADR 0001: Use Rust as Core Implementation Language

- **Status**: **Accepted**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Lawh aims to provide a lightweight, instant-starting, offline whiteboard for studying and technical explanation. Most existing whiteboards (Excalidraw, tldraw, Obsidian Canvas) are built on TypeScript/JavaScript web stacks (Electron, React, Chromium Canvas). While web technologies offer rapid prototyping, they consume 150–400 MB of RAM, launch slowly, and introduce unpredictable garbage-collection stutter during pen strokes.

We need a systems programming language that compiles to lean native binaries, guarantees memory safety, delivers consistent 60+ FPS rendering, and offers rich 2D graphics libraries.

---

## 2. Decision Drivers
- **Cold Boot Time**: Must start up in $< 100\text{ ms}$.
- **Memory Footprint**: Must consume $< 60\text{ MB}$ RSS.
- **Predictable Latency**: Zero garbage collection pauses during continuous mouse drawing.
- **Cross-Platform Portability**: Native compilation across Linux, Windows, and macOS.
- **AI Agent Productivity**: Strong type system and compiler error diagnostics that allow AI coding agents to write reliable, verifiable code.

---

## 3. Considered Options
- **Option 1**: Rust (Native binary, zero-cost abstractions, strong typing, active ecosystem).
- **Option 2**: TypeScript / Electron or Tauri (Rapid web ecosystem, Excalidraw codebase familiarity, but higher memory and web runtime overhead).
- **Option 3**: C++ / Qt (Native speed, mature desktop GUI, but memory safety risks and complex build toolchains for a solo founder).
- **Option 4**: Go (Fast compilation, simple concurrency, but garbage collected and limited desktop 2D rendering ecosystem).

---

## 4. Decision Outcome & Rationale
Chosen option: **Option 1 (Rust)**.

### Rationale
1. **Performance & Footprint**: Rust binaries start immediately, consume negligible memory, and have zero GC runtime overhead.
2. **Compiler-Guaranteed Safety**: Rust’s borrow checker and type system eliminate data races, null pointer dereferences, and memory corruption bugs without a runtime cost.
3. **Flourishing 2D Graphics Ecosystem**: The Rust graphics community has produced world-class 2D rendering libraries (`tiny-skia`, `vello`), windowing libraries (`winit`), and text shapers (`cosmic-text`, `parley`).
4. **AI-Assisted Synergy**: The Rust compiler’s strict error messages act as a strong feedback harness for AI agents, catching logic bugs at compile time.

### Negative Consequences & Mitigations
- **Learning Curve**: The founder is a Rust beginner.  
  *Mitigation*: AI agents operate in "Teaching Mode" (as codified in `AGENTS.md`), explaining Rust idioms, ownership, and borrow-checker patterns clearly. Architecture is kept strictly modular with thin abstractions.
