# Edison Note

An open-source, cross-platform desktop note-taking application built with Rust, Tauri, and egui. Edison Note combines the power of markdown editing with AI-enhanced features, bi-directional linking, and a polished user interface that rivals Obsidian and Notion.

![Edison Note Logo](assets/logo.png)

## ✨ Features

### 📝 Core Note-Taking
- **Markdown Editor**: Rich markdown editing with syntax highlighting
- **Live Preview**: Real-time markdown preview with seamless editing
- **Rich Text Toolbar**: Easy formatting with intuitive toolbar buttons
- **Multimedia Support**: Embed images, PDFs, videos, and links

### 🔗 Organization & Linking
- **Bi-directional Linking**: Connect notes with `[[Note Title]]` syntax
- **Graph View**: Visualize note connections and relationships
- **Folders & Tags**: Organize notes hierarchically and with flexible tagging
- **Advanced Search**: Powerful search with regex, date ranges, and tag combinations
  - **Regex Search**: `/pattern/` for complex text matching
  - **Tag Combinations**: `tag:work AND tag:urgent` boolean logic
  - **Date Ranges**: `date:2024-01-01..2024-12-31` temporal filtering
  - **Field Searches**: `title:meeting content:agenda` targeted searches

### 🤖 AI Enhancement
- **Local AI Processing**: Privacy-first AI enhancements using local models (optimized <300ms)
- **Cloud AI Support**: Optional premium cloud AI (xAI Grok API)
- **Enhanced Modal**: Real-time preview, undo functionality, and progress indicators
- **Custom AI Prompts**: Template-based instructions (Meeting Notes, Technical Doc, Journal, Task List)
- **Enhancement Types**:
  - **Clarity**: Improve readability and flow
  - **Structure**: Add proper markdown formatting
  - **Tags**: Context-aware auto-suggestions with caching
  - **Summarization**: Generate content summaries
  - **Grammar**: Fix spelling and grammar issues
- **Configurable Settings**: Adjust AI sensitivity, style, and enhancement types

### 🎨 Modern UI Design
- **Edison Tech Branding**: Open Blue (#0078D4) and Community Green (#00CC6A)
- **Dynamic Theming**: Custom accent color picker with live preview
- **Responsive Layout**: Three-column grid with collapsible sidebars
- **Polished Aesthetics**: Rounded corners, shadows, and smooth 150ms animations
- **Dark/Light Themes**: Customizable appearance settings

### 🔒 Privacy & Security
- **Local-First**: All data stored locally in `~/EdisonNote`
- **Optional Cloud Sync**: Encrypted synchronization (opt-in)
- **No Tracking**: Anonymous usage only with explicit consent
- **Open Source**: MIT license, community-driven development

### ♿ Accessibility
- **WCAG 2.1 AA Compliant**: Full keyboard navigation and screen reader support
- **High Contrast Mode**: Enhanced visibility options
- **Scalable Fonts**: Adjustable text sizes (12-24px)
- **Keyboard Shortcuts**: Efficient navigation and editing

### 🚀 Advanced Features
- **Collaborative Editing**: Real-time multi-user editing with WebSocket support
- **Plugin Ecosystem**: Extensible with Rust dynamic libraries
- **Canvas View**: Visual whiteboard for note organization
- **Command Palette**: Quick actions with `Ctrl+Shift+P`
- **Version History**: Track note changes over time
- **Export Options**: HTML, PDF, plain text, and JSON export
- **Custom CSS**: Personalize the interface appearance
- **Performance Optimized**: <500ms note loading for 1,000+ notes

## 🛠️ Tech Stack

- **Rust**: Core application logic and performance
- **Tauri**: Cross-platform desktop framework
- **egui**: Immediate-mode GUI for responsive interface
- **rusqlite**: Local database for note storage and indexing
- **pulldown-cmark**: Markdown parsing and rendering
- **fuzzy-matcher**: Fast fuzzy search capabilities
- **reqwest**: HTTP client for cloud AI features
- **accesskit**: Accessibility support
- **tokio-tungstenite**: WebSocket for collaborative editing
- **operational-transform**: Conflict-free collaborative editing

## 📦 Installation

### Prerequisites

- Rust 1.70+ 
- Node.js 16+ (for Tauri)
- Git

### From Source

```bash
# Clone the repository
git clone https://github.com/prequired/enote.git
cd enote

# Install dependencies
cargo build

# Run the application
cargo run
```

### Binary Releases

Download the latest release for your platform:
- **Windows**: `edison-note-windows.exe`
- **macOS**: `edison-note-macos.dmg`
- **Linux**: `edison-note-linux.AppImage`

## 🚀 Quick Start

1. **Create Your First Note**: Click "📝 New Note" or use `Ctrl+N`
2. **Write in Markdown**: Use the editor with live preview
3. **Link Notes**: Create connections with `[[Other Note]]` syntax
4. **Enhance with AI**: Click the ✨ Enhance button for AI improvements
5. **Organize**: Use folders, tags, and the graph view
6. **Explore**: Try the command palette with `Ctrl+Shift+P`

## ⚡ Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| New Note | `Ctrl+N` |
| Save Note | `Ctrl+S` |
| Search | `Ctrl+F` |
| Command Palette | `Ctrl+Shift+P` |
| AI Enhance | `Ctrl+E` |
| Graph View | `Ctrl+G` |
| Toggle Sidebar | `Ctrl+\\` |
| Start Collaboration | `🤝 Collaborate` button |

## 🎨 Configuration

Edison Note stores configuration in:
- **Windows**: `%APPDATA%/EdisonNote/`
- **macOS**: `~/Library/Application Support/EdisonNote/`
- **Linux**: `~/.config/EdisonNote/`

### AI Configuration

To enable cloud AI features, set your API key:

```bash
export XAI_API_KEY="your-xai-api-key"
```

Or configure it in Settings > Enhance > Cloud AI.

## 🔧 Development

### Project Structure

```
src/
├── main.rs              # Application entry point
├── ui/                  # User interface components
│   ├── mod.rs          # UI module exports
│   ├── editor.rs       # Markdown editor
│   ├── sidebar.rs      # Left/right sidebars
│   ├── toolbar.rs      # Top toolbar
│   ├── settings.rs     # Settings panel
│   └── graph_view.rs   # Graph visualization
├── features/           # Core functionality
│   ├── mod.rs          # Feature exports
│   ├── note_manager.rs # Note management
│   ├── linking.rs      # Bi-directional linking
│   ├── search.rs       # Advanced search engine
│   └── collaboration.rs# Real-time collaborative editing
├── ai/                 # AI enhancement
│   ├── mod.rs          # AI module exports
│   ├── local_ai.rs     # Local AI processing
│   ├── cloud_ai.rs     # Cloud AI integration
│   └── enhancer.rs     # Enhancement coordinator
├── storage/            # Data persistence
│   └── mod.rs          # Database operations
└── utils/              # Utilities
    ├── file_utils.rs   # File operations
    ├── markdown.rs     # Markdown processing
    └── export.rs       # Export functionality
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

### Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🌟 Roadmap

### v1.0 (Current)
- ✅ Core note-taking functionality
- ✅ Bi-directional linking
- ✅ AI enhancement features with custom prompts
- ✅ Advanced search (regex, date ranges, tag combinations)
- ✅ Dynamic theming and UI polish
- ✅ Performance optimizations (<500ms note loading)
- ✅ Graph view
- ✅ Basic plugin system
- ✅ Collaborative editing (WebSocket-based)

### v1.1 (Next)
- 🔄 Cloud synchronization
- 🔄 Mobile companion app
- 🔄 Advanced plugin marketplace
- 🔄 Beta testing program
- 🔄 Advanced export formats

### v2.0 (Future)
- 🔄 Enterprise collaboration features
- 🔄 Advanced AI features (GPT integration)
- 🔄 Integration ecosystem
- 🔄 Mobile apps (iOS/Android)

## 🤝 Community

- **GitHub**: [prequired/enote](https://github.com/prequired/enote)
- **Discord**: [Join our community](https://discord.gg/edison-tech)
- **Documentation**: [docs.edison-tech.org](https://docs.edison-tech.org)
- **Blog**: [edison-tech.org/blog](https://edison-tech.org/blog)

## 💖 Acknowledgments

- **Obsidian**: Inspiration for linking and graph features
- **Notion**: UI/UX design inspiration
- **VS Code**: Polish and attention to detail
- **Rust Community**: Amazing ecosystem and support
- **Tauri Team**: Excellent cross-platform framework

## 🏆 Goals

Edison Note aims to become the leading open-source note-taking application by:

- **50,000 active users** within 1-2 years
- **1,000 GitHub contributors** building the ecosystem
- **Ethical practices** with user privacy and openness
- **Innovation** in AI-assisted note-taking
- **Community-driven** development and growth

---

**Built with ❤️ by the Edison Tech community**

[⭐ Star us on GitHub](https://github.com/prequired/enote) | [🚀 Download Edison Note](https://github.com/prequired/enote/releases)
