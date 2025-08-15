# Contributing to csvmd

Thank you for your interest in contributing to csvmd! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Standards](#code-standards)
- [Testing](#testing)
- [Making Changes](#making-changes)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Issue Reporting](#issue-reporting)
- [Release Process](#release-process)

## Getting Started

csvmd is a Rust CLI tool that converts CSV files to Markdown tables. The project values:

- **Code quality**: Clean, well-tested, and documented code
- **Performance**: Efficient processing of both small and large CSV files
- **Reliability**: Comprehensive error handling and edge case coverage
- **Usability**: Intuitive CLI interface and clear documentation

## Development Setup

### Prerequisites

1. **Install Rust**: Visit [rustup.rs](https://rustup.rs/) to install Rust and Cargo
   - csvmd requires Rust 1.70 or later (Rust 2021 edition)
   - The stable toolchain is recommended for development
2. **Git**: Ensure you have Git installed for version control

### Local Setup

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/csvmd.git
   cd csvmd
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests to ensure everything works:
   ```bash
   cargo test
   ```
5. Try the CLI tool:
   ```bash
   cargo run -- --help
   ```

### Project Structure

- `src/lib.rs` - Core library with conversion logic
- `src/main.rs` - CLI interface using clap
- `src/error.rs` - Custom error types
- `tests/` - Integration and edge case tests
- `README.md` - User documentation
- `Cargo.toml` - Project configuration and dependencies

## Code Standards

### Formatting

**Always run `cargo fmt` before committing code.** The project uses rustfmt for consistent code formatting.

```bash
# Format your code
cargo fmt

# Check formatting without making changes
cargo fmt --check
```

### Linting

Use clippy to catch common mistakes and improve code quality:

```bash
cargo clippy
```

Fix any clippy warnings before submitting your pull request.

### Code Style Guidelines

- Use descriptive variable and function names
- Add documentation comments (`///`) for public functions and types
- Keep functions focused and reasonably sized
- Use `Result<T>` for operations that can fail
- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)

### Error Handling

- Use the custom `CsvMdError` types defined in `src/error.rs`
- Provide helpful error messages that guide users toward solutions
- Handle edge cases gracefully (empty files, malformed CSV, etc.)

## Testing

The project has comprehensive test coverage across multiple levels:

### Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests (in src/lib.rs)
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run specific test
cargo test test_csv_with_pipes

# Run tests with output
cargo test -- --nocapture

# Review and approve snapshot test changes (when using insta)
cargo insta review
```

### Test Types

1. **Unit Tests** (`src/lib.rs`): Test core conversion logic, configuration options, and error conditions
2. **Integration Tests** (`tests/integration_tests.rs`): Test full CLI functionality using temporary files
3. **Edge Case Tests** (`tests/edge_cases.rs`): Test unusual inputs, malformed data, and boundary conditions

### Writing Tests

When adding new features or fixing bugs:

1. **Add unit tests** for new functions or logic changes
2. **Add integration tests** for new CLI options or behaviors
3. **Add edge case tests** for unusual inputs or error conditions
4. **Use snapshot testing** with `insta` for complex output validation:
   ```rust
   insta::assert_snapshot!(result);
   ```
   - Use `cargo insta review` to review and approve snapshot changes
   - Include snapshot files (`.snap`) in your commits
5. **Use temporary files** for file-based tests:
   ```rust
   use tempfile::NamedTempFile;
   let mut temp_file = NamedTempFile::new().unwrap();
   ```

### Test Guidelines

- Test both success and failure cases
- Include tests for different CSV features (quotes, newlines, special characters)
- Test CLI options and combinations
- Ensure tests are deterministic and don't depend on external state
- Use descriptive test names that explain what is being tested

## Making Changes

### Before You Start

1. Check existing issues to see if your change is already planned
2. For significant changes, open an issue to discuss the approach first
3. Ensure your development environment is set up correctly

### Development Workflow

1. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes, following the code standards
3. Add or update tests for your changes
4. Run the full test suite:
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy
   ```
5. Test your changes manually with various CSV inputs
6. Commit your changes with clear, descriptive messages
7. Push to your fork and create a pull request

### Performance Considerations

- For large file handling, prefer the streaming approach (`csv_to_markdown_streaming`)
- Use appropriate string capacity pre-allocation when building output
- Profile your changes if they affect the main conversion logic
- Consider memory usage for large CSV files

## Pull Request Guidelines

### Before Submitting

- [ ] All tests pass (`cargo test`)
- [ ] Code is properly formatted (`cargo fmt --check`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated if needed
- [ ] Changes are covered by tests

### PR Description

Include in your pull request:

1. **Summary**: Brief description of what the PR does
2. **Motivation**: Why this change is needed
3. **Changes**: List of specific changes made
4. **Testing**: How you tested the changes
5. **Breaking Changes**: Any breaking changes (should be rare)

### Example PR Template

```markdown
## Summary
Add support for custom header alignment in streaming mode

## Motivation
Users requested the ability to use center/right alignment with the --stream flag

## Changes
- Extended streaming functions to accept HeaderAlignment parameter
- Updated CLI to pass alignment config to streaming mode
- Added tests for streaming + alignment combinations

## Testing
- Added unit tests for streaming with different alignments
- Added integration tests for CLI --stream --align combinations
- Manually tested with large CSV files

## Breaking Changes
None - this is a backward-compatible addition
```

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

1. **csvmd version**: Run `csvmd --version`
2. **Operating system** and version
3. **Input CSV**: Minimal example that reproduces the issue
4. **Expected behavior**: What you expected to happen
5. **Actual behavior**: What actually happened
6. **Steps to reproduce**: Exact commands used

### Feature Requests

For feature requests, please include:

1. **Use case**: Why you need this feature
2. **Proposed solution**: How you think it should work
3. **Alternatives**: Other approaches you've considered
4. **Examples**: Sample input/output if applicable

### Security Issues

For security vulnerabilities, please email the maintainer directly rather than opening a public issue.

## Release Process

*This section is primarily for maintainers*

### Version Numbering

csvmd follows [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes (backward compatible)

### Release Steps

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if it exists)
3. Create git tag: `git tag -a v1.x.x -m "Release v1.x.x"`
4. Push tag: `git push origin v1.x.x`
5. GitHub Actions will automatically:
   - Build binaries for all platforms
   - Create GitHub release
   - Publish to crates.io

### Binary Distribution

The CI system automatically creates binaries for:
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64, universal)
- Windows (x86_64)

Binaries are signed and notarized for macOS.

## Getting Help

- **Questions**: Open a GitHub issue with the "question" label
- **Discussions**: Use GitHub Discussions for general topics
- **Chat**: Check if there's a community chat (Discord, Slack, etc.)

## Code of Conduct

Be respectful and inclusive in all interactions. We want csvmd to be welcoming to contributors from all backgrounds and experience levels.

---

Thank you for contributing to csvmd! Your contributions help make CSV-to-Markdown conversion better for everyone.