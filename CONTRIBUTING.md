# Contributing to CLI Task Timer

Thank you for your interest in contributing to CLI Task Timer! We welcome contributions from everyone and appreciate your help in making this project better.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Reporting Issues](#reporting-issues)
- [Feature Requests](#feature-requests)

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:

   ```bash
   git clone https://github.com/YOUR_USERNAME/CLI_Task_Timer.git
   cd CLI_Task_Timer
   ```

3. **Add the upstream** remote:

   ```bash
   git remote add upstream https://github.com/shubhendud-turing/CLI_Task_Timer.git
   ```

## Development Setup

### Prerequisites

- **Rust**: This project requires Rust 1.90 or later
- **Git**: For version control

### Installing Rust

If you don't have Rust installed, you can install it using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building the Project

```bash
# Clone and navigate to the project
git clone https://github.com/YOUR_USERNAME/CLI_Task_Timer.git
cd CLI_Task_Timer

# Build the project
cargo build

# Build for release (optimized)
cargo build --release
```

### Installing for Local Testing

#### Option 1: Install in Current Directory

Install the `tt` binary locally for testing without affecting your global installation:

```bash
# Install from current directory
cargo install --path . --bin tt

# The binary will be installed to ~/.cargo/bin/tt
# Make sure ~/.cargo/bin is in your PATH
```

#### Option 2: Use Cargo Run for Testing

Test the application directly without installation:

```bash
# Run with cargo (recommended for development)
cargo run --bin tt -- --help
cargo run --bin tt -- start "Test task"
cargo run --bin tt -- pause
cargo run --bin tt -- status
cargo run --bin tt -- list

# Examples of testing different scenarios
cargo run --bin tt -- start "Feature development"
cargo run --bin tt -- pause
cargo run --bin tt -- resume
cargo run --bin tt -- complete
```

#### Option 3: Use Built Binary Directly

```bash
# After building, use the binary directly
./target/debug/tt --help
./target/release/tt start "Direct binary test"
```

### Testing

#### Running All Tests

```bash
# Run all unit tests
cargo test --bin tt

# Run tests with output (useful for debugging)
cargo test --bin tt -- --nocapture

# Run only unit tests (excludes integration tests)
cargo test --bins

# Run integration tests (if any)
cargo test --test integration_tests
```

#### Running Specific Tests

```bash
# Run tests for a specific module
cargo test task::tests
cargo test cli::tests
cargo test display::tests
cargo test workflows

# Run a specific test function
cargo test test_task_pause
cargo test test_complete_workflow_start_and_pause

# Run tests matching a pattern
cargo test workflow
```

#### Test Categories

The project includes several types of tests:

1. **Unit Tests** (in each module):

   - Task creation and state management
   - CLI argument parsing
   - Display formatting functions
   - Error handling

2. **Integration Tests** (in `tests/` directory):

   - End-to-end CLI command testing
   - Full application workflow testing

3. **Workflow Tests** (in `src/workflows.rs`):
   - Complete task lifecycle testing
   - Multi-task scenario validation
   - Real-time duration measurement

#### Performance Testing

```bash
# Build optimized release version
cargo build --release

# Time the startup performance
time ./target/release/tt --version

# Test with larger workloads
./target/release/tt start "Performance test"
# ... simulate work ...
./target/release/tt pause
```

### Development Workflow

#### Testing During Development

```bash
# 1. Make changes to source code
# 2. Run relevant tests
cargo test --bin tt

# 3. Test CLI functionality manually
cargo run --bin tt -- start "Testing changes"

# 4. Build and test release version
cargo build --release
./target/release/tt --help

# 5. Install locally for broader testing
cargo install --path . --bin tt --force
tt start "Integration testing"
```

#### Continuous Testing

For continuous testing during development, you can use `cargo watch`:

```bash
# Install cargo-watch if not already installed
cargo install cargo-watch

# Run tests automatically on file changes
cargo watch -x "test --bin tt"

# Run tests and build on changes
cargo watch -x "test --bin tt" -x "build --release"
```

## How to Contribute

### Types of Contributions

We welcome several types of contributions:

- **Bug fixes**
- **New features**
- **Documentation improvements**
- **Performance improvements**
- **Code refactoring**
- **Tests**

### Workflow

1. **Check existing issues** to see if your contribution idea already exists
2. **Create an issue** for discussion if you're planning a significant change
3. **Create a branch** for your work:

   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

4. **Make your changes**
5. **Test your changes**
6. **Commit your changes** with a clear message
7. **Push to your fork**
8. **Create a Pull Request**

## Coding Standards

### Rust Style Guidelines

This project follows the official Rust style guidelines and uses strict linting rules:

- **Use `rustfmt`** for code formatting:

  ```bash
  cargo fmt
  ```

- **Use `clippy`** for linting (all clippy warnings are denied):

  ```bash
  cargo clippy
  ```

- **Follow Rust naming conventions**:
  - `snake_case` for variables and functions
  - `PascalCase` for types and structs
  - `SCREAMING_SNAKE_CASE` for constants

### Code Quality

- Write **clear, self-documenting code**
- Add **comments** for complex logic
- Use **meaningful variable and function names**
- Keep functions **small and focused**
- Avoid **unsafe code** (denied by project lints)
- **Handle errors appropriately**

### Commit Messages

Use clear and descriptive commit messages:

```text
type(scope): brief description

Longer description if necessary

- List any breaking changes
- Reference related issues (#123)
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Examples**:

```text
feat(timer): add pause functionality to task timer
fix(cli): resolve crash when no arguments provided
docs(readme): update installation instructions
test(timer): add unit tests for timer pause/resume
```

## Test Guidelines

### Writing Tests

- Write **unit tests** for all new functionality
- Write **integration tests** for CLI behavior
- Ensure **good test coverage**
- Use **descriptive test names**

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(expected_result, result);
    }
}
```

### Test Execution

```bash
# Run all tests
cargo test

# Run tests in a specific module
cargo test module_name

# Run with verbose output
cargo test -- --nocapture
```

## Pull Request Process

### Before Submitting

1. **Sync with upstream**:

   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

2. **Rebase your branch** (if necessary):

   ```bash
   git checkout your-branch
   git rebase main
   ```

3. **Run all checks**:

   ```bash
   cargo fmt --check
   cargo clippy
   cargo test
   cargo build --release
   ```

### Pull Request Requirements

- **Clear title** describing the change
- **Detailed description** explaining:
  - What changes were made
  - Why they were made
  - How to test them
- **Link related issues** using keywords (e.g., "Fixes #123")
- **All tests pass**
- **Code follows project standards**
- **Documentation updated** if necessary

### Review Process

- **All PRs require review** before merging
- **Address feedback** promptly and respectfully
- **Keep PRs focused** - one feature/fix per PR
- **Be patient** - maintainers volunteer their time

## Reporting Issues

### Bug Reports

When reporting bugs, please include:

- **Clear title** and description
- **Steps to reproduce** the issue
- **Expected vs actual behavior**
- **Environment information**:
  - OS and version
  - Rust version (`rustc --version`)
  - CLI Task Timer version
- **Error messages** or stack traces
- **Minimal test case** if possible

### Security Issues

For security vulnerabilities, please **DO NOT** open a public issue. Instead, email the maintainers directly.

## Feature Requests

When requesting features:

- **Check existing issues** first
- **Provide clear use case** and rationale
- **Describe the expected behavior**
- **Consider implementation complexity**
- **Be open to discussion** about alternatives

## Development Tips

### Useful Commands

```bash
# Format code
cargo fmt

# Check for linting issues
cargo clippy

# Check compilation without building
cargo check

# Build documentation
cargo doc --open

# Run with specific log level
RUST_LOG=debug cargo run
```

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Code Review**: Ask questions in PR comments

## Recognition

Contributors will be recognized in:

- **CHANGELOG.md** for significant contributions
- **README.md** contributors section
- **Git history** with proper attribution

## License

By contributing to CLI Task Timer, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to CLI Task Timer! 🎉
