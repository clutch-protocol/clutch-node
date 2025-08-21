# Contributing to Clutch Node

Thank you for your interest in contributing to Clutch Node! This document provides guidelines and information for contributors.

## 🌟 Vision

Clutch Protocol aims to revolutionize ride-sharing through decentralization, reducing fees from 15-25% to 5-8%, enabling instant payouts, and empowering users via community governance.

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- Docker & Docker Compose
- Git

### Development Setup
1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/clutch-node.git
   cd clutch-node
   ```
3. Set up development environment:
   ```bash
   cargo build
   cargo test
   ```

## 📋 How to Contribute

### 1. Issues
- Check existing issues before creating new ones
- Use issue templates when available
- Provide clear descriptions and reproduction steps
- Label issues appropriately

### 2. Pull Requests
- Create feature branches: `git checkout -b feature/your-feature-name`
- Follow the PR template
- Ensure all tests pass
- Keep commits atomic and well-documented
- Reference related issues in PR description

### 3. Code Standards

#### Rust Guidelines
- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add tests for new functionality
- Document public APIs with `///` comments

#### Security Focus
- Client-side signing only
- No private key exposure
- Validate all inputs
- Follow blockchain security best practices

### 4. Commit Messages
Follow conventional commits format:
```
type(scope): description

Examples:
feat(consensus): implement Aura consensus mechanism
fix(api): resolve transaction validation bug
docs(readme): update installation instructions
```

## 🧪 Testing

### Running Tests
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# All tests with coverage
cargo test --all-features
```

### Test Guidelines
- Write tests for all new features
- Maintain test coverage above 80%
- Test both success and error cases
- Use descriptive test names

## 📚 Documentation

- Update README.md for user-facing changes
- Add inline documentation for complex code
- Update API documentation
- Include examples in documentation

## 🔒 Security

### Reporting Security Issues
- **DO NOT** create public issues for security vulnerabilities
- Email: mehran.mazhar@gmail.com
- Include detailed reproduction steps
- Allow time for investigation before disclosure

### Security Guidelines
- Never commit private keys or secrets
- Use secure coding practices
- Follow cryptographic best practices
- Validate all external inputs

## 🏗️ Architecture Principles

### Decentralization First
- No single points of failure
- Minimize centralized dependencies
- Enable peer-to-peer interactions

### Scalability
- Design for high throughput
- Consider network effects
- Plan for horizontal scaling

### Transparency
- All transactions on-chain
- Open source everything
- Clear audit trails

## 📅 Development Process

### Roadmap Alignment
Current focus areas (MVP by September 12, 2025):
- Consensus mechanism (Aura)
- Transaction validation
- Node networking
- Fee distribution (90% drivers, 5% nodes, 5% developers)

### Review Process
1. Automated CI/CD checks
2. Code review by maintainers
3. Security review for sensitive changes
4. Community feedback for major features

## 🤝 Community

### Communication
- GitHub Discussions for general questions
- Issues for bugs and feature requests
- Email for security concerns

### Code of Conduct
We follow a strict code of conduct:
- Be respectful and inclusive
- Focus on constructive feedback
- Zero tolerance for harassment
- Prioritize project goals over personal preferences

## 🎯 Getting Help

### Resources
- [README.md](./README.md) - Project overview
- [Documentation](./docs/) - Detailed guides
- [Examples](./examples/) - Code samples

### Contact
- **Maintainer:** Mehran Mazhar
- **Email:** mehran.mazhar@gmail.com
- **GitHub:** [@MehranMazhar](https://github.com/MehranMazhar)

## 🏆 Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Invited to community calls
- Eligible for future governance tokens

## 📄 License

By contributing, you agree that your contributions will be licensed under the same license as the project (Apache 2.0).

---

**Remember:** We're building the future of decentralized transportation. Every contribution matters! 🚗⛓️