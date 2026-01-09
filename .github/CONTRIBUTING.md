# Contributing to b58uuid-rs

Thank you for your interest in contributing to b58uuid-rs! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow. Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue using the bug report template. Include:

- A clear description of the bug
- Steps to reproduce the issue
- Expected vs actual behavior
- Your environment (Rust version, OS, etc.)
- Code examples demonstrating the issue

### Suggesting Features

Feature requests are welcome! Please create an issue using the feature request template and include:

- A clear description of the feature
- The use case it would enable
- Example usage code
- Any alternatives you've considered

### Submitting Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following the coding standards below
3. **Add tests** for any new functionality
4. **Ensure all tests pass**: `cargo test`
5. **Run clippy**: `cargo clippy -- -D warnings`
6. **Format your code**: `cargo fmt`
7. **Update documentation** if needed
8. **Submit a pull request** with a clear description of your changes

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/b58uuid-rs.git
cd b58uuid-rs

# Build the project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## Coding Standards

### Style Guidelines

- Follow Rust standard style guidelines
- Use `cargo fmt` to format all code
- Use `cargo clippy` to catch common mistakes
- Write clear, self-documenting code with appropriate comments
- Keep functions focused and reasonably sized

### Testing Requirements

- All new functionality must include tests
- Maintain or improve code coverage
- Tests should be clear and well-documented
- Include both unit tests and integration tests where appropriate

### Documentation

- Document all public APIs with doc comments
- Include examples in doc comments
- Update README.md if adding new features
- Keep documentation clear and concise

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Reference issue numbers when applicable
- Keep the first line under 72 characters

Example:
```
Add support for custom Base58 alphabets

- Implement configurable alphabet
- Add tests for custom alphabets
- Update documentation

Fixes #123
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests with coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

### Writing Tests

- Place unit tests in the same file as the code they test
- Place integration tests in the `tests/` directory
- Use descriptive test names that explain what is being tested
- Test both success and failure cases
- Test edge cases and boundary conditions

## Performance Considerations

- Profile code before optimizing
- Document any performance-critical sections
- Include benchmarks for performance-sensitive code
- Avoid premature optimization

## Questions?

If you have questions about contributing, feel free to:

- Open a discussion on GitHub
- Ask in an issue
- Reach out to the maintainers

Thank you for contributing to b58uuid-rs!
