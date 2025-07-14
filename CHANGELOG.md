# Changelog

All notable changes to Edison Note will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project setup and core architecture
- Basic note-taking functionality with markdown support
- AI enhancement features with local and cloud processing
- Bi-directional linking system
- Graph view for note relationships
- Advanced search with regex and date range support
- Cross-platform desktop application (Windows, macOS, Linux)
- WCAG 2.1 AA accessibility compliance
- Plugin system architecture
- Collaborative editing foundation
- CI/CD pipeline with GitHub Actions

### Changed
- N/A (Initial release)

### Deprecated
- N/A (Initial release)

### Removed
- N/A (Initial release)

### Fixed
- N/A (Initial release)

### Security
- Implemented local-first data storage
- Added opt-in encrypted sync capabilities
- Security audit pipeline with cargo-audit

## [0.1.0] - 2024-XX-XX

### Added
- Initial release of Edison Note
- Core note-taking functionality
- Markdown editor with live preview
- Rich text toolbar
- File-based storage in ~/EdisonNote
- Basic UI with three-column layout
- Open Blue and Community Green theming
- MIT license and open-source foundation

---

## Release Notes Template

When creating a new release, use this template:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features and capabilities

### Changed
- Changes to existing functionality

### Deprecated
- Features that will be removed in future versions

### Removed
- Features removed in this version

### Fixed
- Bug fixes and corrections

### Security
- Security improvements and patches
```

## Version Numbering

Edison Note follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version when you make incompatible API changes
- **MINOR** version when you add functionality in a backwards compatible manner
- **PATCH** version when you make backwards compatible bug fixes

### Pre-release Identifiers

- **alpha**: Early development, unstable features
- **beta**: Feature-complete, testing phase
- **rc**: Release candidate, final testing

Examples: `1.0.0-alpha.1`, `1.0.0-beta.2`, `1.0.0-rc.1`