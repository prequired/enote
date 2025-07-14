# Security Policy

## Supported Versions

We actively support the following versions of Edison Note with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

The Edison Note team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities to us through one of these methods:

1. **Email**: Send details to `security@edison-tech.org`
2. **GitHub Security Advisory**: Use GitHub's private vulnerability reporting feature
3. **Encrypted Communication**: Use our PGP key for sensitive reports

### What to Include

When reporting a vulnerability, please include:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

### Response Timeline

- **Acknowledgment**: We will acknowledge receipt of your vulnerability report within 2 business days
- **Initial Assessment**: We will provide an initial assessment within 5 business days
- **Updates**: We will keep you informed of our progress every 5 business days
- **Resolution**: We aim to resolve critical vulnerabilities within 30 days

### What to Expect

After you submit a report, here's what happens:

1. **Confirmation**: We'll confirm receipt and begin investigation
2. **Assessment**: We'll assess the severity and impact
3. **Fix Development**: We'll develop and test a fix
4. **Disclosure**: We'll coordinate disclosure with you
5. **Credit**: We'll credit you in our security advisory (if desired)

## Security Features

Edison Note implements several security measures:

### Data Protection
- **Local-First Storage**: Data is stored locally by default
- **Encryption at Rest**: Optional database encryption
- **Secure Sync**: End-to-end encryption for cloud synchronization
- **No Telemetry**: No data collection without explicit consent

### Application Security
- **Tauri Framework**: Leverages Tauri's security model
- **Rust Memory Safety**: Memory-safe implementation
- **Sandboxing**: Limited system access through Tauri
- **Code Signing**: Signed releases for authenticity

### Network Security
- **HTTPS Only**: All network communications use HTTPS
- **Certificate Pinning**: Protection against man-in-the-middle attacks
- **API Key Security**: Secure storage of cloud AI API keys
- **Rate Limiting**: Protection against abuse

### Privacy Protection
- **Minimal Data Collection**: Only necessary data is processed
- **Opt-in Analytics**: Usage analytics require explicit consent
- **Local AI Processing**: AI features work offline when possible
- **Data Anonymization**: Personal data is anonymized when transmitted

## Security Best Practices

### For Users
- Keep Edison Note updated to the latest version
- Use strong, unique passwords for cloud sync
- Enable automatic updates when available
- Report suspicious behavior immediately
- Review privacy settings regularly

### For Developers
- Follow secure coding practices
- Run security scans on dependencies
- Use the principle of least privilege
- Validate all input data
- Keep dependencies updated
- Review code for security issues

## Vulnerability Disclosure Policy

### Our Commitment
We are committed to:
- Investigating all legitimate reports
- Keeping you informed of our progress
- Crediting researchers who help improve our security
- Working with you to understand and resolve issues

### Responsible Disclosure
We ask that you:
- Give us reasonable time to investigate and fix issues
- Avoid privacy violations, destruction of data, or service disruption
- Don't access or modify other users' data
- Don't perform testing on our production systems

### Legal Safe Harbor
We will not pursue legal action against researchers who:
- Follow this responsible disclosure policy
- Act in good faith
- Don't violate the law or harm users

## Security Hall of Fame

We recognize security researchers who help make Edison Note safer:

<!-- Security researchers will be listed here -->

## Contact Information

- **Security Email**: security@edison-tech.org
- **General Contact**: community@edison-tech.org
- **GitHub**: [prequired/enote](https://github.com/prequired/enote)

## Additional Resources

- [Contributing Guidelines](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Privacy Policy](docs/Privacy-Policy.md)
- [Security Architecture](docs/Architecture-Overview.md#security-architecture)

---

Thank you for helping keep Edison Note and our users safe!