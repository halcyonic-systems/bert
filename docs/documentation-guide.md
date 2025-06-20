# BERT Documentation Guide

This guide helps you navigate the various documentation resources available in the BERT project.

## Documentation Structure

BERT's documentation is organized into two distinct systems:

### 1. User Documentation (GitBook)

The primary user-facing documentation is maintained in the `gitbook/` directory and published to [bert.gitbook.io](https://bert.gitbook.io/bert-documentation). This documentation focuses on:

- How to use BERT
- Tutorials and examples
- Conceptual explanations
- System language principles

The GitBook content follows the GitBook publishing workflow and structure, allowing for a polished, user-friendly documentation website.

### 2. Developer Documentation (docs/)

The developer and contributor documentation is maintained in the `docs/` directory and focuses on:

- **Getting Started for Developers**: Quick start guides in [docs/getting-started/](getting-started/)
  - Controls guide
  - Tutorial walkthroughs
  - Code navigation

- **Architecture**: System design documents in [docs/architecture/](architecture/)
  - High-level architecture overview
  - Visual system architecture
  - Interaction system architecture
  
- **Contributing**: Development guidelines in [docs/contributing/](contributing/)
  - Contribution workflow
  - Coding standards
  - Documentation requirements
  - Directory organization guidance

- **Research**: Background materials 
  - System language specifications
  - Field status audits
  - Implementation research
  - *Note: Research materials will be added to a dedicated repository in the future*

### 3. Code Documentation

- **Rust Documentation**: Generated from code comments
  - Run `cargo doc --open` to generate and view
  - Follow guidelines in [docs/contributing/rust-documentation-guidelines.md](contributing/rust-documentation-guidelines.md)

## Documentation Locations

| Content Type | Location | Purpose | Audience |
|--------------|----------|---------|----------|
| Project Overview | README.md | Introduction and quick start | All users |
| User Guides | gitbook/ | Complete user documentation | End users |
| Developer Guides | docs/ | Implementation details | Contributors |
| API Documentation | Generated from code | Technical reference | Developers |

## Key Documentation Files

### For End Users

- **README.md**: Project overview and getting started
- **gitbook/getting-started/**: User tutorials and guides
- **gitbook/for-researchers/**: Systems science concepts

### For Developers

- **docs/contributing/contributing.md**: How to contribute to BERT
- **docs/contributing/directory-organization.md**: Repository structure guidelines
- **docs/architecture/comprehensive-architecture-overview.md**: Technical architecture

### For Researchers

- **System Language Specification**: Formal specification for the System Language framework *(coming soon)*
- **Deep Systems Analysis Methodology**: Documentation of the DSA approach *(coming soon)*
- *Note: These materials will be available in the forthcoming research repository*

## Documentation Systems

### GitBook (User Documentation)

The `gitbook/` directory contains content that is published to [bert.gitbook.io](https://bert.gitbook.io/bert-documentation) using the GitBook publishing system. This provides:

- A polished, searchable documentation website
- Navigation hierarchy through SUMMARY.md
- User-friendly reading experience
- Clear categorization of topics

**Do not move or restructure** the gitbook directory as it follows GitBook's expected format for publishing.

### docs/ (Developer Documentation)

The `docs/` directory contains documentation primarily for contributors and developers working on BERT. This documentation:

- Is directly accessible in the repository
- Follows a simpler structure
- Is more closely tied to the codebase
- Contains technical details not relevant to end users

## Documentation Standards

All documentation in the BERT project should:

1. **Be clear and concise**
2. **Use consistent terminology**
3. **Include examples where appropriate**
4. **Cross-reference related documents**
5. **Follow Markdown best practices**
6. **Respect the separation** between user (gitbook) and developer (docs) documentation

## Contributing to Documentation

We welcome documentation improvements! If you'd like to contribute:

1. Follow the guidelines in [docs/contributing/documentation-implementation-analysis.md](contributing/documentation-implementation-analysis.md)
2. Ensure you're maintaining the existing structure
3. Submit a pull request with your changes

## Future Documentation Plans

- Expanded tutorial library
- Video walkthroughs
- API documentation
- Interactive examples