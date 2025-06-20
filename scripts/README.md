# BERT Utility Scripts

This directory contains utility scripts for development, building, and maintenance of the BERT project.

## Available Scripts

### bert.sh

A comprehensive helper script for common BERT development tasks:

```bash
# Show help
./scripts/bert.sh help

# Setup development environment
./scripts/bert.sh setup

# Run in development mode
./scripts/bert.sh dev

# Build for production
./scripts/bert.sh build

# Clean build artifacts
./scripts/bert.sh clean

# Run linting tools
./scripts/bert.sh lint

# Generate documentation
./scripts/bert.sh docs

# Run tests
./scripts/bert.sh test
```

## Adding Scripts

When adding scripts to this directory:

1. Use clear, descriptive names
2. Add proper documentation and usage instructions
3. Ensure scripts are platform-agnostic when possible
4. Include error handling

## Script Organization

Organize scripts into subdirectories based on their purpose:

- `build/` - Build-related scripts
- `dev/` - Development environment scripts
- `test/` - Testing automation scripts
- `docs/` - Documentation generation scripts

## Running Scripts

For consistency, all scripts should be runnable from the repository root:

```bash
# Main helper script
./scripts/bert.sh [command]

# Other scripts
./scripts/build/release.sh
```