#!/bin/bash
# BERT Development Utility Script
# Helps with common development tasks for the BERT project

# Set script to exit on error
set -e

# Define colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Print header
echo -e "${BLUE}╔══════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         BERT Development Helper          ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════╝${NC}"

# Function to display help message
show_help() {
    echo -e "${GREEN}Available commands:${NC}"
    echo -e "  ${YELLOW}setup${NC}      - Install dependencies and prepare development environment"
    echo -e "  ${YELLOW}dev${NC}        - Run BERT in development mode"
    echo -e "  ${YELLOW}build${NC}      - Build BERT for production"
    echo -e "  ${YELLOW}clean${NC}      - Clean build artifacts"
    echo -e "  ${YELLOW}lint${NC}       - Run linting tools"
    echo -e "  ${YELLOW}docs${NC}       - Generate documentation"
    echo -e "  ${YELLOW}test${NC}       - Run tests"
    echo -e "  ${YELLOW}feature${NC}    - Generate or update feature documentation"
    echo -e "  ${YELLOW}help${NC}       - Show this help message"
}

# Setup command
setup_command() {
    echo -e "${GREEN}Installing dependencies...${NC}"
    npm install
    rustup update
    echo -e "${GREEN}Setup complete!${NC}"
}

# Dev command
dev_command() {
    echo -e "${GREEN}Starting development server...${NC}"
    cargo tauri dev
}

# Build command
build_command() {
    echo -e "${GREEN}Building for production...${NC}"
    cargo tauri build
}

# Clean command
clean_command() {
    echo -e "${GREEN}Cleaning build artifacts...${NC}"
    cargo clean
    rm -rf dist
    echo -e "${GREEN}Clean complete!${NC}"
}

# Lint command
lint_command() {
    echo -e "${GREEN}Running linting tools...${NC}"
    cargo fmt --all -- --check
    cargo clippy -- -D warnings
    echo -e "${GREEN}Linting complete!${NC}"
}

# Docs command
docs_command() {
    echo -e "${GREEN}Generating documentation...${NC}"
    cargo doc --no-deps --open
}

# Test command
test_command() {
    echo -e "${GREEN}Running tests...${NC}"
    cargo test
}

# Feature documentation command
feature_command() {
    if [ $# -eq 0 ]; then
        echo -e "${RED}Error: Feature name is required${NC}"
        echo -e "Usage: $0 feature \"Feature Name\""
        exit 1
    fi
    
    echo -e "${GREEN}Generating feature documentation for: $1${NC}"
    ./scripts/gen-feature-docs.sh "$1" "${@:2}"
}

# Parse command line arguments
if [ $# -eq 0 ]; then
    show_help
    exit 0
fi

case "$1" in
    setup)
        setup_command
        ;;
    dev)
        dev_command
        ;;
    build)
        build_command
        ;;
    clean)
        clean_command
        ;;
    lint)
        lint_command
        ;;
    docs)
        docs_command
        ;;
    test)
        test_command
        ;;
    feature)
        feature_command "${@:2}"
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        show_help
        exit 1
        ;;
esac

exit 0