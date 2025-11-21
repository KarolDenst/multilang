# Development Standards

This document outlines the standards and best practices for contributing to this project.

## General Principles
- **Keep Nodes Simple**: AST nodes should focus on their specific logic. Complex parsing logic should be handled by the parser or helper methods.
- **Use `ParsedChildren`**: When implementing `from_children`, always use the `ParsedChildren` helper to extract child nodes safely.
- **Test Driven Development**: Write tests for new features *before* or *alongside* implementation. Ensure all tests pass before merging.

## Documentation
- **Update Documentation**: Always update the `README.md` and relevant files in `docs/` after implementing a new feature or changing existing behavior.

## Code Style
- Follow standard Rust formatting (use `cargo fmt`).
- Use descriptive variable and function names.
- Add comments for complex logic, especially in the parser and grammar definitions.
