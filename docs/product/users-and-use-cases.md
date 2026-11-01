# Users and Use Cases

## 1. Target Personas

### Persona A: "Ziad" — The Solo Computer Science Student / Self-Learner
- **Profile**: A university student or self-taught developer working through complex data structures, algorithms, and distributed systems topics late at night.
- **Pain Points**: Web-based whiteboards require login, load slowly on modest laptops, and drain battery. Drawing with a trackpad or cheap mouse feels jittery and jagged in standard paint tools.
- **How Lawh Helps**: Instant startup, dark mode that doesn't blind him at 2 AM, velocity-smoothed mouse pen that turns quick gestures into readable handwritten notes, and easy local file organization alongside his course code.

### Persona B: "Maya" — The Technical Content Creator & Explainer
- **Profile**: A developer creating technical tutorials, coding videos, or explaining architectural concepts on YouTube or in team demos.
- **Pain Points**: Heavy enterprise whiteboards show UI clutter (share buttons, user avatars, comment badges) during screen recordings. Formal vector tools look rigid and take too long to arrange while talking.
- **How Lawh Helps**: The sketchy hand-drawn aesthetic feels warm and conversational on video. The UI chrome fades cleanly, leaving maximum canvas real estate for live demonstration. Clean PNG export lets her drop diagrams directly into slide decks.

### Persona C: "Tariq" — The Solo Software Architect
- **Profile**: A senior developer designing a new software component, database schema, or state machine before typing out Rust or backend code.
- **Pain Points**: Complex modeling tools (UML software, Visio) force too much upfront structure. Markdown-only notes lack spatial clarity.
- **How Lawh Helps**: Freeform spatial brainstorming. He can drop rough boxes, connect them with arrows, jot handwritten notes, and iterate without wrestling with auto-formatting constraints.

---

## 2. Core Use Cases & Scenarios

### Use Case 1: The "Feynman Technique" Study Session
1. Ziad opens Lawh in under half a second.
2. He selects the hand-drawn rectangle tool and types/draws the components of a B-Tree or Raft consensus node.
3. Using the freehand pen with mouse smoothing, he arrows between nodes, writing step-by-step numbers to explain the algorithm to himself as if teaching a beginner.
4. When done, he saves `raft-election.lawh` locally and exports a transparent PNG to paste into his Obsidian study vault.

### Use Case 2: Visual Code Review & Bug Post-Mortem
1. Maya encounters a subtle race condition in async code.
2. She opens Lawh, sketches two timelines with the line tool, drops boxes representing concurrent thread execution, and circles the contention point with a red freehand pen.
3. She hits `Ctrl+E` (Export PNG) and drops the resulting image into the GitHub Pull Request discussion, making the race condition immediately obvious to reviewers.

### Use Case 3: Offline Brainstorming While Traveling
1. Tariq is on a flight or train without Wi-Fi.
2. He launches Lawh without worrying about offline cache sync errors or login popups.
3. He pans across an infinite canvas mapping out microservice boundaries and API contracts, saving versions locally with standard `Ctrl+S`.
