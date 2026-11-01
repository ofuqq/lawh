# Product Principles

These six principles guide every architectural, feature, and UI decision in Lawh. When trade-offs arise, we evaluate them against these core tenets.

---

### 1. Light and Fast
Lawh must feel as instantaneous as grabbing a real piece of paper.
- Cold startup must remain under 100 milliseconds.
- Memory consumption should stay small (< 60 MB for average sketches).
- Rendering and panning must sustain a steady 60 FPS on modest integrated GPUs and CPUs.
- Avoid bulky runtimes, heavy web views, or unnecessary background services.

### 2. Offline and Local-First
Your thoughts and study materials belong exclusively on your computer.
- Lawh works 100% offline without requiring any network connection.
- Documents are saved as human-readable, versioned `.lawh` files directly in your filesystem.
- Zero analytics, zero telemetry tracking, and zero forced account creation.

### 3. Hand-Drawn and Approachable
Perfection inhibits exploratory learning and rapid problem solving.
- Shapes should feature controlled sketchiness and jitter (via the Rough algorithm).
- Diagramming should feel informal, invites edits, and reduces the intimidation of drawing complex system designs.
- Avoid clinical, hyper-precise CAD-style snap systems that slow down quick thinking.

### 4. Simple First
A focused tool that does five things exceptionally well beats a bloated tool that does fifty things mediocrity.
- Limit configuration knobs; provide intelligent, aesthetically pleasing defaults.
- Say "no" to features that primarily serve corporate enterprise meetings (e.g., voting timers, video calls, collaborative cursors).
- Every tool on the screen must earn its existence through frequent study and explanation usage.

### 5. Mouse-First Usability
A stylus or tablet should be a welcome luxury, never a requirement.
- Stroke smoothing and velocity-based stroke expansion must make standard desktop mouse drawing feel natural and legible.
- Canvas navigation (pan via middle-click or space-drag, zoom via scroll wheel) must be effortless using only one hand on the mouse.

### 6. Respect the Learner's Focus
Studying often happens late at night or during deep concentration blocks.
- Dark theme is the first-class default, pairing a high-contrast charcoal canvas with vibrant pastel pen strokes.
- Clean, minimal UI chrome that recedes into the background while drawing.
