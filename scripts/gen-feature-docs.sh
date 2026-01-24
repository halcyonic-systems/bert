#!/bin/bash
# Feature Documentation Generator
# Generates standardized documentation for feature branches

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
echo -e "${BLUE}║      BERT Feature Documentation Gen      ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════╝${NC}"

# Function to display help message
show_help() {
    echo -e "${GREEN}Usage:${NC}"
    echo -e "  $0 <feature-name> [options]"
    echo -e ""
    echo -e "${GREEN}Options:${NC}"
    echo -e "  ${YELLOW}-b, --branch${NC}      Specify the branch name (default: current branch)"
    echo -e "  ${YELLOW}-o, --output${NC}      Output directory (default: docs/features/)"
    echo -e "  ${YELLOW}-h, --help${NC}        Show this help message"
    echo -e ""
    echo -e "${GREEN}Examples:${NC}"
    echo -e "  $0 \"LLM Chat Integration\""
    echo -e "  $0 \"Screenshot Export\" -b feature/screenshot-export"
}

# Default values
BRANCH=$(git symbolic-ref --short HEAD 2>/dev/null || echo "unknown")
OUTPUT_DIR="docs/features"
FEATURE_NAME=""
CURRENT_DATE=$(date +"%Y-%m-%d")
CONTRIBUTORS="[Your Name]"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        -b|--branch)
            BRANCH="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            if [[ -z "$FEATURE_NAME" ]]; then
                FEATURE_NAME="$1"
                shift
            else
                echo -e "${RED}Unknown argument: $1${NC}"
                show_help
                exit 1
            fi
            ;;
    esac
done

# Check if feature name is provided
if [[ -z "$FEATURE_NAME" ]]; then
    echo -e "${RED}Error: Feature name is required${NC}"
    show_help
    exit 1
fi

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Convert feature name to filename format
FILENAME=$(echo "$FEATURE_NAME" | tr '[:upper:] ' '[:lower:]-')
FILEPATH="$OUTPUT_DIR/$FILENAME.md"

# Check if a documentation file already exists
if [[ -f "$FILEPATH" ]]; then
    echo -e "${YELLOW}Warning: Documentation file already exists at $FILEPATH${NC}"
    read -p "Do you want to overwrite it? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${RED}Aborted.${NC}"
        exit 0
    fi
fi

# Get list of modified and added files
MODIFIED_FILES=$(git diff --name-status $(git merge-base HEAD origin/main) HEAD | grep -E "^[AM]" | sed 's/^[AM]\s\+//' | grep "\.rs$" | sort)

# Generate component lists
COMPONENTS_ADDED=""
COMPONENTS_MODIFIED=""

while IFS= read -r file; do
    # Check if file exists in main branch
    if git cat-file -e origin/main:"$file" 2>/dev/null; then
        # File exists in main, so it's modified
        COMPONENTS_MODIFIED+="- \`$file\`: [Describe changes]\n"
    else
        # File doesn't exist in main, so it's added
        COMPONENTS_ADDED+="- \`$file\`: [Describe purpose]\n"
    fi
done <<< "$MODIFIED_FILES"

# Generate documentation from template
cat > "$FILEPATH" << EOF
# Feature: $FEATURE_NAME

## Overview

**Feature Name**: $FEATURE_NAME
**Branch**: $BRANCH
**Status**: In Progress
**Date Created**: $CURRENT_DATE
**Date Completed**: —
**Contributors**: $CONTRIBUTORS

## Description

[Provide a brief description of what this feature does and why it's valuable]

## Implemented Functionality

- [List specific capabilities implemented]
- [Be specific about what users can now do]
- [Include any limitations or constraints]

## Technical Implementation

### Components Added

${COMPONENTS_ADDED:-[No new components added]}

### Components Modified

${COMPONENTS_MODIFIED:-[No existing components modified]}

### Architecture Decisions

[Brief explanation of key architectural decisions, patterns used, and their rationale]

## Usage Examples

\`\`\`rust
// Simple code example showing how to use the feature
let example = Feature::new();
example.demonstrate();
\`\`\`

## Testing Strategy

[Describe how this feature has been tested]

## Future Improvements

- [Potential enhancements identified during implementation]
- [Known limitations that could be addressed]
- [Ideas for extending the feature]

## Documentation Updates Required

**Before merging, ensure user-facing docs are updated:**

- [ ] Update relevant \`gitbook/\` pages (see below)
- [ ] Add to release notes if user-visible change
- [ ] Update \`gitbook/for-researchers/\` if new systems concept
- [ ] Add example to \`gitbook/examples/\` if applicable

**Gitbook pages to update:**
- [List specific pages, e.g., \`gitbook/for-researchers/system-archetypes.md\`]

## Related Documentation

- [Links to related features or documentation]
- [References to external resources or dependencies]
- [Design documents or discussions]

---

_Feature doc created $CURRENT_DATE. Update **Status** to "Complete" and **Date Completed** when merged._
EOF

echo -e "${GREEN}Feature documentation generated at $FILEPATH${NC}"
echo -e "${YELLOW}Important:${NC} Please review and update the documentation with accurate descriptions."
echo -e "To edit: ${BLUE}code $FILEPATH${NC}"

exit 0