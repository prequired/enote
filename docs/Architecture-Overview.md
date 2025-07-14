# Architecture Overview

This document provides a comprehensive overview of Edison Note's architecture, designed for developers who want to understand or contribute to the codebase.

## 🏗️ High-Level Architecture

Edison Note follows a modular, layered architecture built on modern Rust ecosystem tools:

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interface (egui)                    │
├─────────────────────────────────────────────────────────────┤
│              Tauri Application Framework                    │
├─────────────────────────────────────────────────────────────┤
│  Features Layer  │  AI Layer  │  Storage Layer │ Utils      │
│  ┌─────────────┐ │ ┌────────┐ │ ┌────────────┐ │ ┌────────┐ │
│  │ Note Mgmt   │ │ │ Local  │ │ │  SQLite    │ │ │ Export │ │
│  │ Linking     │ │ │ Cloud  │ │ │  File I/O  │ │ │ Utils  │ │
│  │ Search      │ │ │ Enhance│ │ │  Sync      │ │ │ Markdown│ │
│  │ Collab      │ │ └────────┘ │ └────────────┘ │ └────────┘ │
│  └─────────────┘ │            │                │            │
├─────────────────────────────────────────────────────────────┤
│                      Operating System                       │
└─────────────────────────────────────────────────────────────┘
```

## 📁 Directory Structure

```
src/
├── main.rs              # Application entry point and initialization
├── ui/                  # User interface components
│   ├── mod.rs          # Module exports and UI coordination
│   ├── editor.rs       # Markdown editor with syntax highlighting
│   ├── sidebar.rs      # Left and right sidebar components
│   ├── toolbar.rs      # Top toolbar with actions
│   ├── settings.rs     # Settings panel and configuration UI
│   ├── graph_view.rs   # Graph visualization using petgraph
│   └── accessibility.rs# WCAG 2.1 AA compliance and accesskit
├── features/           # Core application features
│   ├── mod.rs          # Feature coordination and state management
│   ├── note_manager.rs # CRUD operations for notes
│   ├── linking.rs      # Bi-directional linking implementation
│   ├── search.rs       # Advanced search with regex and filters
│   └── collaboration.rs# Real-time collaborative editing
├── ai/                 # AI enhancement system
│   ├── mod.rs          # AI coordinator and configuration
│   ├── local_ai.rs     # Local AI processing (rust-bert)
│   ├── cloud_ai.rs     # Cloud AI integration (xAI Grok)
│   └── enhancer.rs     # Enhancement orchestration
├── storage/            # Data persistence and management
│   └── mod.rs          # Database operations and file handling
└── utils/              # Shared utilities
    ├── file_utils.rs   # File system operations
    ├── markdown.rs     # Markdown processing and parsing
    └── export.rs       # Export functionality (HTML, PDF, JSON)
```

## 🧱 Core Components

### 1. Application Framework (Tauri)

**Purpose**: Cross-platform desktop application framework
**Location**: Integrated throughout the application
**Key Features**:
- Native OS integration
- Secure communication between frontend and backend
- Auto-updater support
- Platform-specific functionality

```rust
// main.rs - Application initialization
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize application state
            // Set up database connections
            // Configure AI services
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. User Interface Layer (egui)

**Purpose**: Immediate-mode GUI for responsive interface
**Location**: `src/ui/`
**Key Components**:

#### Editor Component (`editor.rs`)
```rust
pub struct MarkdownEditor {
    content: String,
    syntax_highlighter: SyntaxHighlighter,
    cursor_position: usize,
    scroll_position: f32,
}

impl MarkdownEditor {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Render markdown editor with syntax highlighting
        // Handle keyboard input and shortcuts
        // Manage cursor and selection
    }
}
```

#### Sidebar Components (`sidebar.rs`)
```rust
pub struct LeftSidebar {
    search_query: String,
    selected_folder: Option<FolderId>,
    note_list: Vec<Note>,
}

pub struct RightSidebar {
    backlinks: Vec<NoteLink>,
    metadata: NoteMetadata,
    plugin_panels: Vec<PluginPanel>,
}
```

### 3. Features Layer

#### Note Management (`note_manager.rs`)
```rust
pub struct NoteManager {
    storage: Arc<Storage>,
    link_manager: Arc<LinkManager>,
    search_index: SearchIndex,
}

impl NoteManager {
    pub async fn create_note(&self, title: String) -> Result<Note, Error> {
        // Create note in database
        // Update search index
        // Notify link manager
    }
    
    pub async fn save_note(&self, note: &Note) -> Result<(), Error> {
        // Persist to database
        // Update indexes
        // Trigger auto-linking
    }
}
```

#### Linking System (`linking.rs`)
```rust
pub struct LinkManager {
    storage: Arc<Storage>,
    graph: petgraph::Graph<NoteId, LinkType>,
}

impl LinkManager {
    pub fn create_link(&mut self, from: NoteId, to: NoteId) -> Result<(), Error> {
        // Add edge to graph
        // Update database
        // Notify UI components
    }
    
    pub fn get_backlinks(&self, note_id: NoteId) -> Vec<NoteLink> {
        // Query graph for incoming links
        // Return formatted link information
    }
}
```

#### Search Engine (`search.rs`)
```rust
pub struct SearchEngine {
    index: tantivy::Index,
    query_parser: QueryParser,
}

impl SearchEngine {
    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>, Error> {
        // Parse search query (regex, tags, dates)
        // Execute search against index
        // Return ranked results
    }
}
```

### 4. AI Enhancement System

#### AI Coordinator (`ai/mod.rs`)
```rust
pub struct AIManager {
    local_ai: Option<LocalAI>,
    cloud_ai: Option<CloudAI>,
    config: AIConfig,
}

impl AIManager {
    pub async fn enhance_text(&self, text: &str, enhancement_type: EnhancementType) -> Result<String, Error> {
        match self.config.preferred_provider {
            AIProvider::Local => self.local_ai.enhance(text, enhancement_type).await,
            AIProvider::Cloud => self.cloud_ai.enhance(text, enhancement_type).await,
        }
    }
}
```

#### Local AI Processing (`local_ai.rs`)
```rust
use rust_bert::pipelines::generation::GPT2Generator;

pub struct LocalAI {
    generator: GPT2Generator,
    enhancement_prompts: HashMap<EnhancementType, String>,
}

impl LocalAI {
    pub async fn enhance(&self, text: &str, enhancement_type: EnhancementType) -> Result<String, Error> {
        let prompt = self.build_prompt(text, enhancement_type);
        let output = self.generator.generate(Some(&[prompt]), None);
        Ok(self.parse_output(output))
    }
}
```

### 5. Storage Layer

#### Database Operations (`storage/mod.rs`)
```rust
use rusqlite::{Connection, Result};

pub struct Storage {
    conn: Arc<Mutex<Connection>>,
    file_manager: FileManager,
}

impl Storage {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        self.init_schema(&conn)?;
        Ok(Storage {
            conn: Arc::new(Mutex::new(conn)),
            file_manager: FileManager::new(),
        })
    }
    
    pub fn save_note(&self, note: &Note) -> Result<()> {
        // Save to SQLite database
        // Update full-text search index
        // Persist markdown to file system
    }
}
```

## 🔄 Data Flow

### Note Creation Flow
```
User Input → UI Layer → Note Manager → Storage Layer → Database/Files
                                   ↓
                              Link Manager → Graph Update
                                   ↓
                              Search Engine → Index Update
```

### AI Enhancement Flow
```
User Selection → UI Layer → AI Manager → Local/Cloud AI → Enhanced Text
                                                       ↓
                              Note Manager ← Enhanced Content
                                   ↓
                              Storage Layer → Save Updated Note
```

### Search Flow
```
Search Query → UI Layer → Search Engine → Index Query → Results
                                       ↓
                                  Note Manager → Note Details
```

## 🎨 UI Architecture

### Theme System
```rust
pub struct Theme {
    pub primary_color: Color32,      // Open Blue (#0078D4)
    pub accent_color: Color32,       // Community Green (#00CC6A)
    pub background_color: Color32,   // Clean White (#FFFFFF)
    pub text_color: Color32,         // Slate Gray (#4A4A4A)
    pub font_size: f32,
    pub animations_enabled: bool,
}
```

### Layout Management
```rust
pub struct AppLayout {
    left_sidebar_visible: bool,
    right_sidebar_visible: bool,
    sidebar_width: f32,
    main_content_tabs: Vec<NoteTab>,
}
```

## 🔌 Plugin Architecture

### Plugin System Design
```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn init(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    fn render_ui(&mut self, ui: &mut egui::Ui, context: &PluginContext);
    fn handle_event(&mut self, event: PluginEvent) -> Result<(), PluginError>;
}
```

### Plugin Loading
```rust
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    plugin_directories: Vec<PathBuf>,
}

impl PluginManager {
    pub fn load_plugins(&mut self) -> Result<(), Error> {
        // Scan plugin directories
        // Load dynamic libraries
        // Initialize plugin instances
    }
}
```

## ⚡ Performance Considerations

### Memory Management
- **Efficient Note Loading**: Only load visible notes into memory
- **Lazy Rendering**: Use egui's efficient immediate-mode rendering
- **Caching Strategy**: Cache frequently accessed notes and search results

### Database Optimization
```rust
// Indexed queries for fast search
CREATE INDEX idx_notes_title ON notes(title);
CREATE INDEX idx_notes_content_fts ON notes_fts(content);
CREATE INDEX idx_links_from_to ON links(from_note_id, to_note_id);
```

### Async Operations
```rust
// Non-blocking file operations
pub async fn save_note_async(&self, note: Note) -> Result<(), Error> {
    tokio::task::spawn_blocking(move || {
        // Heavy I/O operations on thread pool
    }).await?
}
```

## 🔒 Security Architecture

### Data Protection
- **Local-First**: All data stored locally by default
- **Encryption**: Optional encrypted sync using rustls
- **Sandboxing**: Tauri's security model restricts system access

### Privacy Measures
```rust
pub struct PrivacyConfig {
    telemetry_enabled: bool,
    crash_reporting: bool,
    usage_analytics: bool,
    cloud_sync_enabled: bool,
}
```

## 🧪 Testing Architecture

### Test Organization
```
tests/
├── unit/               # Component-level tests
│   ├── note_manager_tests.rs
│   ├── linking_tests.rs
│   └── ai_tests.rs
├── integration/        # Feature-level tests
│   ├── note_workflow_tests.rs
│   └── search_tests.rs
└── performance/        # Performance benchmarks
    ├── startup_time.rs
    └── search_performance.rs
```

### Testing Strategy
- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Test feature workflows end-to-end
- **Performance Tests**: Benchmark critical operations
- **UI Tests**: Automated accessibility and interaction testing

## 🚀 Build and Deployment

### Build Process
```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release

# Cross-compilation for different platforms
cargo build --target x86_64-pc-windows-msvc
cargo build --target x86_64-apple-darwin
cargo build --target x86_64-unknown-linux-gnu
```

### CI/CD Pipeline
- **GitHub Actions**: Automated testing and building
- **Cross-Platform**: Builds for Windows, macOS, and Linux
- **Security**: Automated vulnerability scanning
- **Release**: Automated release creation with binaries

## 📈 Scalability Considerations

### Future Enhancements
- **Cloud Sync**: End-to-end encrypted synchronization
- **Mobile Apps**: React Native companion apps
- **Web Interface**: Optional web-based access
- **Enterprise Features**: Team collaboration and management

### Performance Targets
- **Startup Time**: < 2 seconds
- **Note Loading**: < 500ms for 1,000+ notes
- **Search Response**: < 100ms for typical queries
- **Memory Usage**: < 200MB for normal operation

---

This architecture provides a solid foundation for Edison Note's current features while allowing for future expansion and community contributions.