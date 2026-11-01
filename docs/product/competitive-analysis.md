# Competitive Analysis & Strategic Positioning

To build a differentiated tool, Lawh learns from existing whiteboarding tools while maintaining a strict focus on local-first solo study. All analysis below is based on verifiable technical facts and licensing models.

---

## 1. Feature & Positioning Comparison

| Feature / Dimension | **Lawh** (Our Project) | **Excalidraw** | **tldraw** | **Obsidian Canvas** | **drafft-ink** |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Primary Focus** | Solo study, local explanation | General sketch & web collab | Canvas SDK for web apps | Note-taking & card linking | Self-hosted collaborative whiteboard |
| **Core Technology** | Rust (Native Desktop) | TypeScript / React / Canvas | TypeScript / React / HTML | Electron / TypeScript | Rust / WebGPU |
| **License** | **Dual MIT / Apache-2.0** | **MIT** | **Proprietary / Custom SDK** | **Proprietary App** (Open JSON spec) | **AGPL-3.0** |
| **Aesthetic Style** | Hand-drawn rough sketch | Hand-drawn rough sketch | Modern clean vector/flat | Standard box-and-card | Smooth digital ink & vectors |
| **Offline Runtime** | 100% Native binary | Browser / Electron wrapper | Browser / Web SDK | Electron desktop app | Docker container / Web app |
| **Cold Startup Time** | **< 100 ms** (Target) | ~1.5–3.0 s (Browser/Electron) | ~1.5–3.0 s (Browser) | ~2.0–4.0 s (Full vault) | Depends on web browser |
| **Memory Footprint** | **~25–60 MB** | ~150–350 MB | ~150–400 MB | ~250–600 MB | ~150–300 MB (Client) |
| **Real-time Collaboration** | **None (Explicit Non-Goal)** | Full via WebSockets | Full via Liveblocks/Sync | None (Local vault) | Full live WebRTC/WebSocket |
| **Arabic / RTL Text** | **First-class goal (Post-v0.1)**| Basic browser BiDi | Basic browser BiDi | Standard Markdown BiDi | Limited WebGPU text shaping |
| **Account Requirement** | None (Ever) | Optional (Excalidraw+) | Optional (tldraw cloud) | None | Depends on deployment |

---

## 2. In-Depth Competitor Breakdown

### Excalidraw
- **Strengths**: The gold standard for hand-drawn sketch aesthetics. Huge community, permissive MIT license, rich library of community shape packs.
- **Weaknesses**: Primarily a web application. Running it as a desktop app requires Electron wrappers, bringing significant RAM and CPU overhead. Local file workflow is second-tier compared to browser local storage.
- **Lawh's Differentiation**: Lawh provides the beloved Excalidraw visual style in a lean, instant-starting native Rust desktop binary with zero web bloat.

### tldraw
- **Strengths**: Incredibly slick modern UI, stellar stroke rendering, sophisticated geometric snapping and SDK architecture.
- **Weaknesses**: License is restrictive (custom SDK license with commercial limitations). Focus is primarily on embedding canvas widgets into other SaaS web products.
- **Lawh's Differentiation**: Truly open source (permissive Dual MIT/Apache-2.0), desktop-native, and focused on end-user study rather than SaaS SDK monetization.

### Obsidian Canvas
- **Strengths**: Deeply integrated with Markdown notes, open JSON Canvas file format specification, excellent for knowledge workers organizing research cards.
- **Weaknesses**: Geared toward cards and documents rather than freehand sketching. The desktop client is proprietary and tied to the broader Obsidian vault lifecycle.
- **Lawh's Differentiation**: Pure hand-drawn sketching and drawing fluidity with an infinite canvas, rather than structured document-card layout.

### drafft-ink
- **Strengths**: Native Rust backend with WebGPU frontend, highly performant rendering, zero subscription self-hosting model.
- **Weaknesses**: Licensed under strict copyleft (AGPL-3.0), requiring downstream projects and self-hosters to share source code. Requires WebGPU-capable hardware and browser stack; focused on team collaboration and server deployment.
- **Lawh's Differentiation**: Permissive licensing, CPU-first rendering for 100% Linux/desktop compatibility without GPU drivers, and laser focus on solo desktop offline use.

---

## 3. Honest Gaps: Where Lawh Intentionally Trails

We deliberately choose **not** to compete in several areas:

1. **No Live Multi-User Collaboration**: Building CRDTs (Conflict-free Replicated Data Types) or WebSocket rooms introduces immense architectural complexity. Lawh is strictly for solo thinking and asynchronous sharing.
2. **No Web/Mobile App at Launch**: Lawh targets native desktop (Linux first, then Windows and macOS). There is no browser deployment in v0.1.
3. **No Enterprise Cloud Sync**: We will not run servers storing user boards or user logins. Files live on your local drive.
4. **No Full Arabic Text in v0.1**: While Arabic is a core organizational priority, full HarfBuzz shaping and bidirectional layout are deferred to v0.2 so v0.1 can ship a rock-solid foundation.
