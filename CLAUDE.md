Claude Development Guide for Edison Note
This document guides further development and maintenance of Edison Note, an open-source, cross-platform desktop note-taking application built with Rust, Tauri, and egui, using Claude (Anthropic’s AI model) to assist in code generation, bug fixes, and feature enhancements. The app, designed to compete with Obsidian and Notion, features a modern, polished UI, AI integration with a configurable "Enhance" button, and a privacy-first architecture. It aligns with Edison Tech’s brand values—openness, accessibility, ethics, innovation, and community—targeting 50,000 active users and 1,000 GitHub contributors within 1–2 years. This guide includes a Claude prompt for ongoing development and a comprehensive test suite (not for GitHub) to ensure quality.
1. Project Summary
Edison Note is a native desktop app for note-taking, bridging everyday consumers (e.g., students, professionals) and power users (e.g., developers, researchers). It offers markdown-based editing, bi-directional linking, a plugin ecosystem, and AI-enhanced features, with a lightweight, responsive UI built using Tauri and egui. The app is performance-optimized (~10 MB footprint), privacy-focused (local-first storage), and accessible (WCAG 2.1 AA), with a modern design rivaling Obsidian’s minimalism and Notion’s fluidity.
Implemented Features

Core Note-Taking: Markdown editor with live preview, rich text toolbar, multimedia support, auto-save, file storage in ~/EdisonNote.
Organization: Bi-directional linking, Graph View (petgraph), folders, tags, fuzzy search.
AI Enhancement: "Enhance" button (sparkle icon) with local (rust-bert) and cloud (xAI Grok API) processing for Clarity, Structure, Tags, Summarization, Grammar; configurable in Settings.
Advanced Features: Plugin ecosystem, Canvas View, Command Palette (fuzzy-matcher), version history, HTML/JSON export.
UI Design: Three-column layout (Left Sidebar, Main Content, Right Sidebar), Open Blue (#0078D4) and Community Green (#00CC6A) palette, Inter font, 200ms animations, rounded corners, shadows.
Accessibility: WCAG 2.1 AA, accesskit, keyboard navigation, high-contrast mode, scalable fonts.
Privacy/Security: Local-first storage (rusqlite), opt-in encrypted sync (rustls), no default tracking.
Developer Experience: Modular Rust code, MIT license, GitHub documentation, contribution guidelines.

Project Structure
edison-note/
├── src/
│   ├── main.rs              # App initialization
│   ├── ui/                  # UI components & accessibility
│   ├── features/            # Note management & linking
│   ├── ai/                  # Enhancement system
│   ├── storage/             # Database operations
│   └── utils/               # File handling & export
├── Cargo.toml               # Dependencies & config
├── tauri.conf.json          # Tauri configuration
├── README.md                # Documentation
├── CONTRIBUTING.md          # Developer guidelines
└── LICENSE                  # MIT license

2. Claude Prompt for Ongoing Development
The following prompt instructs Claude to enhance or maintain Edison Note, building on the existing implementation, fixing bugs, adding features, or improving performance/UI, while preserving the app’s architecture and brand alignment.
Claude Prompt
You are tasked with maintaining and enhancing **Edison Note**, an open-source, cross-platform desktop note-taking app built with Rust, Tauri, and egui, competing with Obsidian and Notion. The app targets everyday consumers and power users, with a modern, polished UI (Open Blue #0078D4, Community Green #00CC6A, Inter font, 200ms animations, rounded corners) that avoids clunky aesthetics. It aligns with Edison Tech’s values: openness (MIT license, GitHub-hosted), accessibility (WCAG 2.1 AA), ethics (local-first, opt-in data), innovation (AI features), and community (1,000 GitHub contributors). The goal is to maintain 50,000 users and grow contributions.

### Existing Implementation
- **Tech Stack**: Rust, Tauri (cross-platform), egui (UI), `markdown-rs` (parsing), `rusqlite` (database), `rustls` (encryption), `reqwest` (HTTP), `petgraph` (Graph View), `fuzzy-matcher` (Command Palette), `rust-bert` (local AI), `accesskit` (accessibility).
- **UI**:
  - Three-column layout: Left Sidebar (Search, Notes, Folders, Tags), Main Content (Editor, Preview, Tabs), Right Sidebar (Metadata, Backlinks, Settings).
  - Top Bar: Native menu (File, Edit, View, Tools, Help), toolbar (New Note, Save, Edit/Preview, Search, Enhance [sparkle, 200ms pulse], Settings, Sync, Theme).
  - Status Bar: Sync status, word count, quick settings.
  - Design: Open Blue buttons, Community Green accents, Clean White backgrounds, Slate Gray text, 8px grid, animations (hover, slide-in).
- **Features**:
  - Core: Markdown editor, rich text toolbar, multimedia, auto-save, ~/EdisonNote storage.
  - Organization: Bi-directional linking (`[[Note Title]]`), Graph View, folders, tags, fuzzy search.
  - AI: Enhance button (Clarity, Structure, Tags, Summarization, Grammar), local (`rust-bert`) and cloud (xAI Grok API), configurable in Settings (Sensitivity, Style).
  - Advanced: Plugins (Rust dynamic libraries), Canvas View, Command Palette, version history, export (HTML/JSON).
  - Privacy: Local-first, opt-in sync, no tracking.
  - Accessibility: WCAG 2.1 AA, keyboard navigation, high-contrast mode.
- **Structure**: `src/main.rs`, `src/ui/`, `src/features/`, `src/ai/`, `src/storage/`, `src/utils/`, `Cargo.toml`, `tauri.conf.json`, `README.md`, `CONTRIBUTING.md`, `LICENSE`.

### Requirements for Claude
1. **Tasks**:
   - Fix bugs (e.g., UI rendering, AI processing, sync errors).
   - Enhance features (e.g., improve AI suggestions, add plugin templates).
   - Optimize performance (e.g., reduce note loading time, UI rendering).
   - Add new features (e.g., collaborative editing, advanced search filters).
   - Improve accessibility (e.g., enhance `accesskit` labels, test screen readers).
2. **Code Guidelines**:
   - Write idiomatic Rust, ensuring memory safety and performance.
   - Use Tauri for native integration, egui for UI components.
   - Maintain modular structure (`src/ui/`, `src/ai/`, etc.).
   - Include inline comments explaining functionality.
   - Preserve UI design: Open Blue (#0078D4), Community Green (#00CC6A), Inter font, 8px grid, 200ms animations.
3. **UI Enhancements**:
   - Ensure polished, non-clunky UX with smooth animations (e.g., 200ms button hover, sidebar slide).
   - Maintain WCAG 2.1 AA (contrast ≥ 4.5:1, `accesskit` labels, keyboard navigation).
   - Enhance micro-interactions (e.g., sparkle pulse on Enhance button, tab transitions).
4. **AI Enhancements**:
   - Improve `rust-bert` local AI accuracy (e.g., better tag suggestions).
   - Optimize xAI Grok API integration (`reqwest`) for cloud AI.
   - Enhance Settings > Enhance tab (e.g., add preview toggle, custom AI prompts).
5. **Deliverables**:
   - Updated Rust code for specified tasks, organized by module.
   - Updated `README.md` and `CONTRIBUTING.md` for GitHub.
   - Exclude test suite from output (provided separately).
   - Documentation for new/changed features.
6. **Constraints**:
   - Preserve existing architecture and brand values.
   - Ensure cross-platform compatibility (Windows, macOS, Linux).
   - Maintain privacy-first design (local storage, opt-in sync).
   - Validate code with `cargo test` and `cargo clippy`.

### Instructions for Claude
- Analyze the existing codebase structure and implementation details.
- Generate Rust code for requested enhancements, bug fixes, or optimizations.
- Update documentation (`README.md`, `CONTRIBUTING.md`) for community contributions.
- Ensure UI remains modern, polished, and accessible, with smooth animations.
- Output code in a single response, organized by module, with clear comments.
- Exclude test suite from output.

3. UI Design Details
The UI is designed to be top-tier, modern, and competitive, avoiding clunky or boring aesthetics, with inspiration from Obsidian’s minimalism, Notion’s fluidity, and VS Code’s polish.
3.1 Visual Identity

Colors:
Open Blue (#0078D4): Buttons, links, active states for trust and interactivity.
Community Green (#00CC6A): Accents (tags, highlights, hover effects) for vibrancy.
Clean White (#FFFFFF): Backgrounds for clarity.
Slate Gray (#4A4A4A): Text, ensuring WCAG 2.1 AA contrast (4.5:1).


Typography: Inter font (bundled or Roboto/Arial fallback), 14–18px, adjustable:
Headers: 18px, bold (note titles, settings tabs).
Body: 16px, regular (editor, menus).
Subtext: 14px, light (metadata, status bar).


Spacing: 8px grid system for padding/margins, ensuring spaciousness.
Micro-Interactions: 200ms animations (e.g., button hover scale 1.05x, sidebar slide-in, tab fade) for a lively feel.
Logo: Circuit/book icon with glow animation in title bar/About.

3.2 Design Principles

Minimalist yet Engaging: Clean layout with Community Green accents, inspired by Obsidian and Notion.
Consistent Design System: Atomic Design (atoms: buttons; molecules: toolbars; organisms: sidebars) for reusable components.
User-Centric Flow: Design Thinking ensures intuitive navigation (e.g., quick toolbar access, searchable palette).
Responsive: egui’s fluid layouts adapt to window sizes.
Accessible: WCAG 2.1 AA with accesskit, keyboard navigation, high-contrast mode.
Polished Aesthetics: Rounded corners (8px radius), soft shadows, hover effects for a premium look.
Dynamic Feedback: Real-time updates (e.g., save status, AI previews) avoid clunkiness.

3.3 UI Components

Menu Bar (Top Left): Native Tauri menus, Open Blue hover, 16px Inter font, 200ms fade-in, drop-shadow dropdowns.
Menus: File (New Note, Save, Import), Edit (Undo, Find), View (Toggle Edit/Preview, Graph View), Tools (Plugins, Enhance Settings), Help (Documentation, About).


Toolbar (Top Right): egui icons (32x32px): New Note (plus), Save (floppy disk), Edit/Preview (eye), Search (magnifying glass), Enhance (sparkle, 200ms pulse), Settings (gear), Sync (cloud), Theme (sun/moon). Open Blue fill, Community Green outline on hover.
Settings Panel (Right Sidebar): Tabbed (General, Appearance, Sync, Plugins, Enhance, Advanced, Privacy, Shortcuts). Open Blue tab headers, Community Green toggles, 200ms tab switch.
Enhance Tab: Toggles (Clarity, Structure, Tags, Summarization, Grammar), dropdowns (Model, Sensitivity, Style), Apply/Cancel buttons with hover scale.


Left Sidebar: Search Bar, Note List, Folders, Tags, Favorites, Trash, collapsible. Clean White background, Community Green tag highlights, 200ms slide-in.
Main Content: Editor (markdown, syntax highlighting), Preview, Split View, Tabbed Interface. Clean White background, Open Blue cursor, Community Green active tabs.
Right Sidebar: Metadata, Backlinks, Plugins, Settings, Outline, Version History, collapsible. Open Blue links, 200ms slide-out.
Status Bar: Sync status, word count, quick settings, Community Green accents, 200ms fade.

4. Feature Set

Core Note-Taking: Markdown editor (markdown-rs), rich text toolbar, multimedia, auto-save, ~/EdisonNote storage.
Organization: Bi-directional linking, Graph View (petgraph), folders, tags, fuzzy search (fuzzy-matcher).
AI Enhancement: Enhance button (sparkle icon) with local (rust-bert) and cloud (xAI Grok API) processing, configurable (Clarity, Structure, Tags, Summarization, Grammar).
Advanced: Plugins (Rust dynamic libraries), Canvas View, Command Palette, version history, export (HTML/JSON).
Privacy: Local-first (rusqlite), opt-in sync (rustls), no tracking.
Accessibility: WCAG 2.1 AA, accesskit, keyboard navigation, high-contrast mode.
Unique: Version history, custom CSS (Tauri webview).

5. Test Suite (Not for GitHub)
This internal test suite validates Edison Note’s functionality, performance, accessibility, and UI polish, ensuring it remains top-tier and competitive.
5.1 Unit Tests

Markdown Editor:
Test parsing: # Header → <h1>Header</h1> (markdown-rs).
Test syntax highlighting: **bold** displays bold in editor.
Test auto-save: Note saved to rusqlite and ~/EdisonNote every 30s.
Test multimedia: ![Image](path) renders image correctly.


Rich Text Toolbar:
Test actions: Bold button inserts **text**, list button inserts - Item.
Test image embedding: Drag-and-drop image adds ![Image](path).


Bi-Directional Linking:
Test linking: [[Note2]] in Note1 creates backlink in Note2 (rusqlite).
Test display: Note2 shows Note1 backlink in right sidebar.


Graph View:
Test petgraph: Links between Note1 and Note2 render as nodes/edges.
Test interaction: Clicking node opens note in Main Content.


AI Enhance:
Test local AI (rust-bert): “Plan meeting” → “# Meeting\n- Plan tasks\n#meeting”.
Test cloud AI (reqwest): Mock xAI Grok API response for “Plan meeting”.
Test modal: Accept/reject buttons update/revert note correctly.
Test configuration: Enable Clarity + Tags, verify output matches settings.


Settings:
Test toggles: Enable Clarity in Enhance tab updates rusqlite config.
Test font size: Change to 18px updates editor display.
Test theme: Switch to dark mode applies Slate Gray background.


Accessibility:
Test accesskit: Toolbar buttons have labels (e.g., label="Enhance Note").
Test keyboard: Tab through toolbar, editor, sidebars without errors.


Plugins:
Test loading: Mock calendar plugin renders in right sidebar.
Test API: Plugin executes without crashing.



5.2 Integration Tests

Note Workflow:
Create note → Add [[Note2]] → Save → View backlinks → Export HTML.
Verify: Note in rusqlite, backlinks in sidebar, HTML output correct.


AI Workflow:
Write note → Click Enhance → Configure Clarity + Tags → Accept.
Verify: Output in rusqlite, matches “# Title\n- Item\n#tag”.


Sync Workflow:
Enable sync → Save note → Check mock server (reqwest).
Verify: Note encrypted (rustls), synced correctly.


Plugin Workflow:
Install mock plugin → Activate in Settings → Use in right sidebar.
Verify: Plugin UI renders, interacts correctly.



5.3 Performance Tests

Startup Time: App loads in <2s (Tauri’s lightweight advantage).
Tool: criterion.
Verify: cargo run completes in <2s.


Note Loading: 1,000 notes load in <1s (rusqlite).
Test: Query 1,000 notes, measure time.


AI Processing: rust-bert processes 500-word note in <500ms.
Test: Run Clarity enhancement on sample note.


Graph View: 100 nodes render in <200ms (petgraph).
Test: Render graph with 100 linked notes.



5.4 Accessibility Tests

WCAG 2.1 AA:
Contrast: Open Blue (#0078D4) on Clean White (#FFFFFF) ≥ 4.5:1.
Keyboard: Navigate all UI elements (menu, toolbar, editor) via Tab/Enter.
Screen Reader: accesskit labels readable by NVDA/VoiceOver.


High-Contrast Mode: Verify UI renders with Slate Gray background.
Scalable Fonts: Test 12px to 20px in editor, ensure readability.

5.5 Usability Tests

User Flow: Simulate creating note, linking, applying AI Enhance, publishing.
Verify: No crashes, intuitive navigation.


Micro-Interactions: Verify 200ms animations (button hover, sidebar slide, Enhance pulse).
Error Handling: Test invalid markdown, sync failures, plugin errors.
Verify: User-friendly error messages in Status Bar.



5.6 Test Implementation

Tools: cargo test for unit/integration tests, criterion for performance, accesskit for accessibility.
Structure:
tests/unit/: Markdown, toolbar, AI, settings.
tests/integration/: Workflows (note, AI, sync, plugin).
tests/performance/: Startup, note loading, AI, Graph View.
tests/accessibility/: WCAG, keyboard, screen reader.


Execution: Run locally (cargo test --all), exclude from GitHub.

6. Implementation Details

Tech Stack: Rust, Tauri, egui, markdown-rs, rusqlite, rustls, reqwest, petgraph, fuzzy-matcher, rust-bert, accesskit.
Privacy: Local storage in ~/EdisonNote, opt-in sync (rustls), no tracking, privacy policy in README.md.
Community: GitHub repo with CONTRIBUTING.md, modular Rust for plugins/templates, GitHub Actions for CI/CD.
Visual Identity: Open Blue (#0078D4), Community Green (#00CC6A), Inter font, 8px grid, circuit/book logo.

7. Development Roadmap

Months 1–3: Fix bugs, optimize AI (rust-bert), enhance UI animations.
Months 4–6: Add collaborative editing, advanced search filters, update README.md.
Months 7–9: Expand plugin ecosystem, beta test for 5,000 users.
Months 10–12: Release v1.1 with new features, target 50,000 users.
Ongoing: Grow to 1,000 contributors, maintain performance.

8. Alignment with Edison Tech Brand

Openness: MIT-licensed, GitHub-hosted plugins.
Accessibility: WCAG 2.1 AA, rich text toolbar, templates.
Ethics: Local-first, opt-in sync, transparent policies.
Innovation: AI Enhance, Graph/Canvas views.
Community: Plugin API, GitHub engagement.

9. Competitive Positioning

Obsidian: Matches linking, Graph View, plugins; adds AI, richer UX.
Notion: Matches templates, collaboration; adds open-source, privacy.
Differentiation: Rust performance, local AI, freemium sync.

10. Next Steps

Run Claude Prompt: Use the prompt to fix bugs, enhance AI, or add features.
Run Test Suite: Validate with cargo test --all and criterion.
Engage Community: Promote v1.0 on X/Reddit (r/rust, r/productivity).
Monitor Feedback: Address user-reported issues via GitHub.

