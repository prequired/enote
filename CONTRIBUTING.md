# Contributing to Edison Note

Thank you for your interest in contributing to Edison Note! We welcome contributions from developers of all skill levels and backgrounds. This document provides guidelines for contributing to our open-source note-taking application.

## 🌟 Our Vision

Edison Note aims to be the leading open-source note-taking application that prioritizes:
- **User Privacy**: Local-first approach with optional cloud features
- **Accessibility**: WCAG 2.1 AA compliance and inclusive design
- **Performance**: Built with Rust for speed and reliability
- **Innovation**: AI-enhanced features that respect user privacy
- **Community**: Open development and transparent decision-making

## 📋 Ways to Contribute

### 🐛 Bug Reports
- Search existing issues before creating a new one
- Use the bug report template
- Include steps to reproduce, expected vs actual behavior
- Provide system information (OS, version, etc.)

### 💡 Feature Requests
- Check if the feature aligns with our roadmap
- Use the feature request template
- Explain the use case and potential impact
- Consider proposing implementation approaches

### 🔧 Code Contributions
- Documentation improvements
- Bug fixes
- New features
- Performance optimizations
- UI/UX enhancements
- Test coverage improvements

### 📚 Documentation
- README improvements
- Code documentation
- User guides
- Developer tutorials
- API documentation

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- Git
- Familiarity with Rust, Tauri, and egui (helpful but not required)

### Setup Development Environment

1. **Fork and Clone**
```bash
git clone https://github.com/your-username/edison-note.git
cd edison-note
```

2. **Install Dependencies**
```bash
cargo build
```

3. **Run Tests**
```bash
cargo test
```

4. **Start Development Server**
```bash
cargo run
```

### Project Structure
```
src/
├── main.rs              # Application entry point
├── ui/                  # User interface components
├── features/           # Core functionality
├── ai/                 # AI enhancement system
├── storage/            # Data persistence
└── utils/              # Utility functions
```

## 🔄 Development Workflow

### 1. Create a Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number
```

### 2. Make Changes
- Follow Rust best practices and idioms
- Write clear, self-documenting code
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes
```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Test the application
cargo run
```

### 4. Commit Your Changes
```bash
git add .
git commit -m "feat: add new note linking feature"
```

**Commit Message Format:**
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `style:` Code formatting
- `refactor:` Code refactoring
- `test:` Adding or modifying tests
- `chore:` Maintenance tasks

### 5. Create Pull Request
- Use the pull request template
- Reference related issues
- Provide clear description of changes
- Include screenshots for UI changes

## 📏 Code Standards

### Rust Code Style
- Follow `rustfmt` formatting
- Use `clippy` for linting
- Write idiomatic Rust code
- Prefer explicit over implicit when it improves readability

### Architecture Principles
- **Modular Design**: Keep components loosely coupled
- **Error Handling**: Use `Result<T, E>` consistently
- **Performance**: Avoid unnecessary allocations
- **Safety**: Leverage Rust's ownership system

### UI/UX Guidelines
- Follow Edison Tech design system
- Ensure WCAG 2.1 AA compliance
- Test with keyboard navigation
- Consider screen reader compatibility
- Maintain consistent spacing (8px grid)

### Testing
- Write unit tests for core logic
- Include integration tests for features
- Test accessibility features
- Verify cross-platform compatibility

## 🎨 Design Contributions

### Visual Design
- Follow the Edison Tech brand colors:
  - Open Blue (#0078D4)
  - Community Green (#00CC6A)
  - Clean White (#FFFFFF)
  - Slate Gray (#4A4A4A)

### Accessibility
- Ensure keyboard navigation works
- Test with screen readers
- Verify color contrast ratios
- Support high contrast mode
- Include ARIA labels where needed

## 🤖 AI Feature Guidelines

### Privacy First
- Default to local AI processing
- Make cloud features opt-in
- Clear data usage policies
- No tracking without consent

### Enhancement Quality
- Test AI suggestions thoroughly
- Provide confidence scores
- Allow users to reject suggestions
- Maintain original content integrity

## 🧪 Testing Guidelines

### Test Categories
- **Unit Tests**: Test individual functions
- **Integration Tests**: Test feature workflows
- **UI Tests**: Test user interactions
- **Performance Tests**: Benchmark critical paths
- **Accessibility Tests**: Verify WCAG compliance

### Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_creation() {
        let note = Note::new("Test Title".to_string());
        assert_eq!(note.title, "Test Title");
        assert!(!note.content.is_empty());
    }
}
```

## 📖 Documentation Standards

### Code Documentation
- Document all public APIs
- Include examples in doc comments
- Explain complex algorithms
- Document safety requirements

### User Documentation
- Clear, concise instructions
- Include screenshots
- Cover common use cases
- Provide troubleshooting guides

## 🔍 Review Process

### Before Submitting
- [ ] Tests pass locally
- [ ] Code follows style guidelines
- [ ] Documentation is updated
- [ ] Accessibility considerations addressed
- [ ] Performance impact considered

### Review Criteria
- **Functionality**: Does it work as intended?
- **Code Quality**: Is it well-written and maintainable?
- **Performance**: Does it impact app performance?
- **Security**: Are there security implications?
- **Accessibility**: Is it accessible to all users?
- **Documentation**: Is it properly documented?

## 🚀 Release Process

### Version Numbers
We follow [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- Major: Breaking changes
- Minor: New features (backwards compatible)
- Patch: Bug fixes

### Release Schedule
- Patch releases: As needed for critical bugs
- Minor releases: Monthly
- Major releases: Every 6-12 months

## 💬 Communication

### Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Discord**: Real-time chat and community
- **Email**: security@edison-tech.org for security issues

### Community Guidelines
- Be respectful and inclusive
- Help newcomers get started
- Share knowledge and learn together
- Focus on constructive feedback

## 🏆 Recognition

### Contributors
All contributors are recognized in:
- CONTRIBUTORS.md file
- Release notes
- About dialog in the application

### Special Recognition
- **Core Contributors**: Regular, significant contributions
- **Maintainers**: Commit access and review privileges
- **Community Champions**: Help others and build community

## 📜 License

By contributing to Edison Note, you agree that your contributions will be licensed under the MIT License.

## ❓ Questions?

- Check existing documentation
- Search GitHub issues
- Ask in GitHub Discussions
- Join our Discord community

Thank you for contributing to Edison Note and helping build the future of open-source note-taking! 🎉