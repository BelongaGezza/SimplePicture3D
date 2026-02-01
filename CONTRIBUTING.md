# Contributing to SimplePicture3D

Thank you for your interest in contributing to SimplePicture3D! We welcome contributions from the community and are excited to work with you.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [How to Contribute](#how-to-contribute)
5. [Coding Standards](#coding-standards)
6. [Testing Guidelines](#testing-guidelines)
7. [Pull Request Process](#pull-request-process)
8. [Security & Dependencies](#security--dependencies)
9. [Community](#community)

---

## Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to follow. By participating, you agree to:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards other community members

We do not tolerate harassment, discrimination, or disrespectful behavior of any kind.

---

## Getting Started

### Rust onboarding (backend contributors)

To contribute to the Rust backend (mesh, file I/O, Tauri commands), we recommend completing **The Rust Book** chapters 1–5 before your first code change:

| Chapter | Topic | Why it matters here |
|---------|--------|----------------------|
| 1 | Getting Started | `cargo`, `rustup`, project layout |
| 2 | Programming a Guessing Game | `let`, types, crates, `Result` |
| 3 | Common Programming Concepts | Variables, functions, control flow |
| 4 | Understanding Ownership | Ownership, borrowing, slices (critical for mesh/bytes) |
| 5 | Using Structs and Enums | Data modeling, `Option`, `Result` |

**Resource:** [The Rust Programming Language](https://doc.rust-lang.org/book/) (doc.rust-lang.org/book).

After chapters 1–5 you should be able to read and modify code in `src-tauri/src/` (e.g. `lib.rs`, `file_io.rs`) and run `cargo test` and `cargo build`. Later chapters (error handling, generics, traits) are useful as you touch more of the backend.

### Prerequisites

Before you begin, ensure you have:

- **Rust 1.70+** - Install via [rustup](https://rustup.rs/)
- **Node.js 18+** - Download from [nodejs.org](https://nodejs.org/)
- **Python 3.9+** - Download from [python.org](https://www.python.org/)
- **Git** - Install from [git-scm.com](https://git-scm.com/)
- **Cursor IDE** (recommended) or VS Code

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/SimplePicture3D.git
   cd SimplePicture3D
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/BelongaGezza/SimplePicture3D.git
   ```

---

## Development Setup

Required tools and versions are listed in [README.md](README.md#development-setup): Rust 1.70+, Node.js 18+, npm 9+, Python 3.9+ (when using the Python AI backend), and Git.

**Basic setup:**

```bash
# 1. Rust backend
cd src-tauri
cargo build
cd ..

# 2. Node / frontend
npm install

# 3. Python (optional until python/requirements.txt exists)
cd python
python -m venv venv
# Windows: venv\Scripts\activate   |   macOS/Linux: source venv/bin/activate
pip install -r requirements.txt
cd ..

# 4. Run app
npm run tauri dev
```

To verify everything works: from the project root run `cargo build` in `src-tauri`, then `npm run build`, then `npm run tauri dev`. See README for full verification commands.

**Rust logging:** The backend uses `env_logger`. Set `RUST_LOG` (e.g. `RUST_LOG=debug` or `RUST_LOG=simplepicture3d=info`) when running the app; see README.

---

## How to Contribute

### Finding Work

- **Good First Issues**: Look for issues labeled [`good first issue`](https://github.com/BelongaGezza/SimplePicture3D/labels/good%20first%20issue)
- **Help Wanted**: Check [`help wanted`](https://github.com/BelongaGezza/SimplePicture3D/labels/help%20wanted) for tasks needing assistance
- **Roadmap**: Review [TODO.md](todo.md) for planned features
- **Bugs**: Browse open [issues](https://github.com/BelongaGezza/SimplePicture3D/issues)

### Before You Start

1. **Check existing work**: Search issues and PRs to avoid duplication
2. **Discuss major changes**: Open an issue for discussion before significant work
3. **Claim the issue**: Comment on the issue to let others know you're working on it

### Types of Contributions

We welcome:

- 🐛 **Bug fixes** - Help us squash bugs
- ✨ **New features** - Implement items from the roadmap
- 📝 **Documentation** - Improve guides, comments, or README
- 🧪 **Tests** - Increase code coverage
- 🎨 **UI/UX improvements** - Enhance user experience
- 🔧 **Tooling** - Improve build scripts, CI/CD
- 🌍 **Translations** - Help make SimplePicture3D multilingual (future)

---

## Coding Standards

### Rust

- **Style**: Use `rustfmt` (run `cargo fmt`)
- **Linting**: Pass `clippy` checks (`cargo clippy`)
- **Conventions**:
  - Use descriptive variable names
  - Write doc comments for public APIs (`///`)
  - Minimize `unsafe` code, justify when necessary
  - Handle errors with `Result` and `anyhow`

**Example:**
```rust
/// Converts a depth map to a point cloud mesh.
///
/// # Arguments
/// * `depth_map` - 2D array of depth values (0.0 to 1.0)
/// * `depth_range` - Target depth in millimeters (min, max)
///
/// # Returns
/// A `Result` containing the mesh or an error.
pub fn generate_mesh(
    depth_map: &Array2<f32>,
    depth_range: (f32, f32),
) -> Result<Mesh> {
    // Implementation
}
```

### Python

- **Style**: Use `black` formatter (`black .`)
- **Linting**: Pass `flake8` or `ruff` checks
- **Type hints**: Use type annotations for function signatures
- **Docstrings**: Follow NumPy style

**Example:**
```python
def estimate_depth(image: np.ndarray, model_name: str = "depth-anything-v2") -> np.ndarray:
    """
    Estimate depth map from RGB image using specified model.

    Parameters
    ----------
    image : np.ndarray
        Input image as RGB array (H, W, 3)
    model_name : str, optional
        Name of depth estimation model (default: "depth-anything-v2")

    Returns
    -------
    np.ndarray
        Depth map as normalized float array (H, W)
    """
    # Implementation
```

### TypeScript/Svelte

- **Style**: Use Prettier (`npm run format`)
- **Linting**: Pass ESLint (`npm run lint`)
- **Conventions**:
  - Use TypeScript for type safety
  - Keep components small and focused
  - Use reactive declarations in Svelte (`$:`)

**Example:**
```typescript
interface DepthControlsProps {
  depthRange: [number, number];
  onRangeChange: (range: [number, number]) => void;
}

export function DepthControls({ depthRange, onRangeChange }: DepthControlsProps) {
  // Implementation
}
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat: add curves tool for depth adjustment`
- `fix: resolve mesh generation crash on empty images`
- `docs: update user guide with preset instructions`
- `test: add unit tests for STL exporter`
- `refactor: simplify depth map normalization logic`
- `chore: update dependencies`

---

## Testing Guidelines

### Running Tests

```bash
# Rust tests (from project root)
cargo test --manifest-path src-tauri/Cargo.toml

# Or from src-tauri:
cd src-tauri && cargo test

# Python tests (when python/ has tests and venv is active)
cd python && pytest

# Frontend tests (when configured)
npm test
```

See [README](README.md#testing) and [CLAUDE.md](CLAUDE.md) for the canonical list of testing commands.

### Code coverage

Coverage can be generated locally. CI does not yet upload coverage reports.

**Rust (cargo-tarpaulin):**

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --manifest-path src-tauri/Cargo.toml --out Stdout
# Or output to HTML: --out Html --output-dir coverage
```

**Python (pytest-cov):**

```bash
pip install pytest-cov
cd python && pytest --cov=. --cov-report=term-missing
# Or HTML: --cov-report=html
```

**Coverage goals (per PRD/todo.md):** Rust >80%, Python >70%, Frontend >60%.

### Writing Tests

- **Unit tests**: Test individual functions in isolation
- **Integration tests**: Test component interactions
- **E2E tests**: Test full user workflows (coming soon)

**Coverage goals:**
- Rust: >80%
- Python: >70%
- Frontend: >60%

**Example (Rust):**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_depth_map() {
        let input = array![[0.0, 0.5], [1.0, 0.25]];
        let result = normalize_depth_map(&input, (2.0, 10.0));
        assert_eq!(result[[0, 0]], 2.0);
        assert_eq!(result[[1, 0]], 10.0);
    }
}
```

---

## Security & Dependencies

Before **adding a new dependency** (Rust crate, npm package, or Python package) or before **release**, follow the [Security Checklist](docs/security-checklist.md). It covers license checks, `cargo audit` / `npm audit` / `pip-audit`, and release sign-off steps.

---

## Pull Request Process

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 2. Make Your Changes

- Write clean, well-documented code
- Add tests for new functionality
- Update documentation if needed

### 3. Test Locally

```bash
# Run all tests
cargo test
pytest
npm test

# Check formatting
cargo fmt -- --check
black --check python/
npm run lint
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "feat: add your feature description"
```

### 5. Push to Your Fork

```bash
git push origin feature/your-feature-name
```

### 6. Open a Pull Request

- Go to the [SimplePicture3D repository](https://github.com/BelongaGezza/SimplePicture3D)
- Click "New Pull Request"
- Select your branch
- Fill out the PR template (coming soon)

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass locally
- [ ] New tests added for new functionality
- [ ] Documentation updated (if applicable)
- [ ] Commit messages follow Conventional Commits
- [ ] No merge conflicts with `main` branch

### Review Process

1. **Automated checks**: CI will run tests and linting
2. **Code review**: Maintainers will review your code
3. **Feedback**: Address any requested changes
4. **Approval**: Once approved, a maintainer will merge

**Be patient**: Reviews may take a few days. Feel free to ping if no response after a week.

---

## Community

### Communication Channels

- **GitHub Issues**: [Bug reports and feature requests](https://github.com/BelongaGezza/SimplePicture3D/issues)
- **GitHub Discussions**: [Questions and ideas](https://github.com/BelongaGezza/SimplePicture3D/discussions)
- **Discord**: Coming soon
- **Email**: Contact maintainer (see README)

### Getting Help

- Read the [User Guide](docs/user-guide.md) (coming soon)
- Check [Developer Guide](docs/developer-guide.md) (coming soon)
- Search [existing issues](https://github.com/BelongaGezza/SimplePicture3D/issues)
- Ask in [Discussions](https://github.com/BelongaGezza/SimplePicture3D/discussions)

### Recognition

Contributors are recognized in:
- [README.md](README.md) acknowledgments
- Release notes for significant contributions
- Project credits (in-app, future)

---

## License

By contributing to SimplePicture3D, you agree that your contributions will be licensed under the [MIT License](LICENSE).

You confirm that:
- You have the right to license your contributions
- Your contributions are your original work or properly attributed
- You understand the MIT License terms

---

## Questions?

If you have questions about contributing, please:
1. Check this guide thoroughly
2. Search existing GitHub Discussions
3. Open a new Discussion with the `question` label

Thank you for contributing to SimplePicture3D! 🎉

---

**Happy coding!** 🚀
