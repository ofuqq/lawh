# ADR 0001: Use Rust as Core Implementation Language

- **Status**: **Accepted**
- **Date**: 2026-11-01
- **Deciders**: Abdulrahman Hossam (Founder / Owner)

---

## 1. Context and Problem Statement
Mirsam aims to provide a lightweight, instant-starting, offline study scratchpad and whiteboard. Most existing tools (Excalidraw, tldraw, Obsidian Canvas) run on web runtimes (Chromium, Electron, DOM/Canvas) that consume 150–400 MB of RAM, launch slowly, and suffer from garbage collection pauses during handwriting strokes.

We require a systems programming language that compiles to lean native binaries, guarantees memory safety without garbage collection, delivers predictable 60+ FPS rendering, and offers rich 2D vector and typography libraries.

---

## 2. Decision Drivers
- **Cold Boot Time**: Must start up in $< 150\text{ ms}$.
- **Memory Footprint**: Must stay under $< 100\text{ MB}$ RSS.
- **Predictable Latency**: Zero garbage collection pauses during mouse drawing.
- **Cross-Platform Portability**: Native compilation across Linux and Windows.
- **Compiler Safety for AI Collaboration**: Strong static typing and compiler diagnostics that guide AI coding agents to write reliable, verifiable code.

---

## 3. Considered Options
- **Option 1**: Rust (Native binary, zero-cost abstractions, strong typing, active ecosystem).
- **Option 2**: TypeScript / Electron or Tauri (Rapid web ecosystem, but higher memory footprint).
- **Option 3**: C++ / Qt (Native speed, mature desktop GUI, but memory safety risks and complex build toolchains).
- **Option 4**: Go (Fast compilation, simple concurrency, but garbage-collected runtime).

---

## 4. Decision Outcome & Rationale
Chosen option: **Option 1 (Rust)**.

### Rationale
1. **Performance & Footprint**: Rust binaries start immediately, consume minimal memory, and have zero GC runtime overhead.
2. **Compiler-Guaranteed Safety**: Rust’s borrow checker eliminates data races, null pointer dereferences, and memory corruption bugs.
3. **Flourishing Graphics & Typography Ecosystem**: Modern pure-Rust crates (`tiny-skia`, `parley`, `roughr`, `freedraw`, `winit`, `softbuffer`) allow a greenfield desktop build without C++ build dependencies.
4. **Contingency Safeguard**: If low-level graphics complexity stalls the project, Rust backends integrate cleanly with a pre-approved Tauri v2 hybrid fallback (see ADR 0006).

### Negative Consequences & Mitigations
- **Learning Curve**: The founder is a Rust beginner.  
  *Mitigation*: AI agents operate in "Teaching Mode" (as codified in `AGENTS.md`), explaining Rust idioms, ownership, and borrow-checker patterns clearly. Module sizes are capped at ~350 LOC.
