# Getting Started with Edison Note

Welcome to Edison Note! This guide will help you install and start using the most powerful open-source note-taking application.

## 📦 Installation

### System Requirements

Before installing Edison Note, ensure your system meets these requirements:

- **Windows**: Windows 10 (1903+) or Windows 11
- **macOS**: macOS 10.15 (Catalina) or later
- **Linux**: Ubuntu 18.04+, Fedora 32+, or equivalent

### Option 1: Download Pre-built Binary (Recommended)

1. Visit the [Edison Note Releases](https://github.com/prequired/enote/releases) page
2. Download the appropriate file for your platform:
   - **Windows**: `edison-note-windows.exe`
   - **macOS**: `edison-note-macos.dmg`
   - **Linux**: `edison-note-linux.AppImage`

#### Windows Installation
1. Download `edison-note-windows.exe`
2. Run the executable
3. Follow the installation wizard
4. Launch Edison Note from the Start Menu

#### macOS Installation
1. Download `edison-note-macos.dmg`
2. Open the DMG file
3. Drag Edison Note to your Applications folder
4. Launch from Applications (you may need to allow the app in Security & Privacy settings)

#### Linux Installation
1. Download `edison-note-linux.AppImage`
2. Make it executable: `chmod +x edison-note-linux.AppImage`
3. Run the AppImage: `./edison-note-linux.AppImage`

### Option 2: Build from Source

If you prefer to build from source or want the latest development features:

#### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Node.js 16+ ([Install Node.js](https://nodejs.org/))
- Git

#### Build Steps
```bash
# Clone the repository
git clone https://github.com/prequired/enote.git
cd enote

# Install dependencies (Linux only)
# For Ubuntu 22.04+ and newer distributions:
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libappindicator3-dev librsvg2-dev patchelf libayatana-appindicator3-dev
# For older Ubuntu versions (20.04 and earlier):
# sudo apt-get install libwebkit2gtk-4.0-dev libgtk-3-dev libappindicator3-dev librsvg2-dev patchelf

# Build and run
cargo build --release
cargo run
```

## 🚀 First Launch

When you first launch Edison Note:

1. **Welcome Screen**: You'll see a welcome message and quick tour option
2. **Data Location**: Edison Note creates a folder at `~/EdisonNote` for your notes
3. **Default Theme**: The app starts with the light theme using Edison Tech colors

## 📝 Your First Note

Let's create your first note:

### Step 1: Create a New Note
- Click the **"📝 New Note"** button in the toolbar, or
- Use the keyboard shortcut `Ctrl+N` (Windows/Linux) or `Cmd+N` (macOS)

### Step 2: Add a Title
- The note title is automatically generated from your first line
- Type something like: `# My First Note`

### Step 3: Write Content
Edison Note supports full Markdown syntax:

```markdown
# My First Note

This is my first note in Edison Note! 

## Features I want to try:
- [ ] Bi-directional linking
- [ ] AI enhancement
- [ ] Graph view
- [ ] Tags and organization

## Interesting facts:
Edison Note is built with:
- **Rust** for performance
- **Tauri** for cross-platform support
- **egui** for the user interface

I can also add [[Another Note]] links!
```

### Step 4: Save Your Note
- Notes auto-save every 30 seconds
- Manual save: `Ctrl+S` (Windows/Linux) or `Cmd+S` (macOS)
- You'll see a "Saved" indicator in the status bar

## 🔗 Creating Your Second Note and Linking

### Bi-directional Linking
1. In your first note, type `[[Another Note]]`
2. This creates a link to a note called "Another Note"
3. Click the link to create and open the new note
4. In "Another Note", you'll see a backlink to "My First Note" in the right sidebar

### Organizing with Folders and Tags
1. **Folders**: Drag notes into folders in the left sidebar
2. **Tags**: Add tags to your notes like `#work` or `#personal`
3. **Search**: Use the search bar to find notes by content, title, or tags

## ✨ Try AI Enhancement

Edison Note's AI features can help improve your notes:

1. Write some text in your note
2. Select the text or place your cursor in a paragraph
3. Click the **✨ Enhance** button in the toolbar
4. Choose enhancement options:
   - **Clarity**: Improve readability
   - **Structure**: Add proper formatting
   - **Tags**: Suggest relevant tags
   - **Grammar**: Fix spelling and grammar

## 🎨 Customization

### Appearance Settings
Access Settings via the gear icon in the toolbar:

- **Theme**: Switch between light and dark modes
- **Font Size**: Adjust text size (12-24px)
- **Colors**: Customize accent colors
- **Layout**: Toggle sidebar visibility

### AI Configuration
In Settings > Enhance:
- **Local AI**: Uses your computer (privacy-focused, works offline)
- **Cloud AI**: Optional premium features (requires API key)
- **Enhancement Types**: Enable/disable specific AI features

## ⌨️ Essential Keyboard Shortcuts

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Note | `Ctrl+N` | `Cmd+N` |
| Save Note | `Ctrl+S` | `Cmd+S` |
| Search | `Ctrl+F` | `Cmd+F` |
| Command Palette | `Ctrl+Shift+P` | `Cmd+Shift+P` |
| AI Enhance | `Ctrl+E` | `Cmd+E` |
| Graph View | `Ctrl+G` | `Cmd+G` |
| Toggle Sidebar | `Ctrl+\` | `Cmd+\` |

## 🔍 Exploring More Features

Now that you have the basics down, explore these advanced features:

### Graph View
- Press `Ctrl+G` to see your notes as a connected graph
- Click nodes to navigate between notes
- Visualize relationships in your knowledge base

### Command Palette
- Press `Ctrl+Shift+P` for quick access to all features
- Type to search for commands
- Great for keyboard-centric workflow

### Advanced Search
- **Regex**: Use `/pattern/` for complex searches
- **Tags**: Search `tag:work` for tagged notes
- **Date ranges**: Use `date:2024-01-01..2024-12-31`
- **Content/Title**: Search `title:meeting` or `content:agenda`

### Plugin System
- Browse available plugins in Settings > Plugins
- Install community-created extensions
- Extend Edison Note's functionality

## 🆘 Getting Help

If you run into any issues:

1. **Check the Wiki**: Browse our [comprehensive documentation](https://github.com/prequired/enote/wiki)
2. **Search Issues**: Look through [existing GitHub issues](https://github.com/prequired/enote/issues)
3. **Join Discord**: Ask questions in our [community chat](https://discord.gg/edison-tech)
4. **Report Bugs**: [Create a new issue](https://github.com/prequired/enote/issues/new/choose) if needed

## 🎯 Next Steps

Ready to dive deeper? Check out these guides:

- **[User Guide](User-Guide)**: Complete feature documentation
- **[AI Enhancement Guide](AI-Enhancement)**: Master the AI features
- **[Keyboard Shortcuts](Keyboard-Shortcuts)**: Become more efficient
- **[Plugins](Plugins)**: Extend Edison Note with plugins

---

**Congratulations! You're now ready to start your note-taking journey with Edison Note. Happy writing! 📝**