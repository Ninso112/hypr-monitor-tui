# Contributing to hypr-monitor-tui

Thank you for your interest in contributing to hypr-monitor-tui.

## Code of Conduct

Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

1. Check existing issues to avoid duplicates.
2. Use the bug report template.
3. Include: hypr-monitor-tui version, Hyprland version, steps to reproduce, expected vs actual behavior, terminal and OS.

### Suggesting Features

1. Check existing issues and roadmap.
2. Use the feature request template.
3. Describe the use case and benefit.

### Code Contributions

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/amazing-feature`).
3. Make your changes.
4. Run tests and lints:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```
5. Commit with clear messages.
6. Push to your fork and open a Pull Request.

## Development Guidelines

- Follow Rust conventions and idioms.
- Use `rustfmt` for formatting.
- All code must pass `clippy` without warnings.
- Write documentation for public APIs.
- Comment complex logic.

### Commit Messages

```
feat: add monitor rotation support
fix: resolve crash when no monitors detected
docs: update installation instructions
refactor: simplify IPC handling
test: add config generation tests
```

## Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # Application state
├── events.rs        # Event handling
├── ui/              # User interface
├── hyprland/        # Hyprland integration
└── config/          # Configuration management
```

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
