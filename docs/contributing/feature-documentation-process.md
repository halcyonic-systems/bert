# Feature Documentation Process

This document outlines the standardized process for documenting features in the BERT project. Following this process ensures consistent, high-quality documentation for all features.

## Documentation Workflow

### 1. Initial Documentation Generation

When starting work on a new feature:

```bash
# Generate feature documentation template
./scripts/gen-feature-docs.sh "Your Feature Name"
```

This creates a standardized markdown file in `docs/features/` with:
- Basic feature information
- Lists of added/modified components (auto-detected)
- Template sections for description, usage examples, etc.

### 2. Iterative Documentation

As you develop the feature:

1. Update the feature documentation with:
   - More detailed descriptions
   - Refined usage examples
   - Technical implementation details
   - Testing information

2. Commit documentation changes alongside code changes:
   ```bash
   git add docs/features/your-feature-name.md
   git commit -m "docs: update feature documentation for XYZ changes"
   ```

### 3. Pre-Merge Documentation Review

Before merging your feature branch:

1. Ensure the feature documentation is complete and accurate
2. Update the status to "Completed" or "Pending Review"
3. Include the documentation file in your pull request

## Documentation Automation Options

For even more automated documentation, consider:

1. **Pre-commit Hook**: Add a pre-commit hook that reminds you to update documentation
2. **PR Template**: Include a checkbox for feature documentation
3. **CI Integration**: Add a CI check that ensures feature documentation exists

## Example Feature Documentation Workflow

```bash
# Start a new feature branch
git checkout -b feature/amazing-feature

# Generate initial documentation
./scripts/gen-feature-docs.sh "Amazing Feature"

# Begin development...

# Update documentation as you go
code docs/features/amazing-feature.md

# Before creating a PR
./scripts/gen-feature-docs.sh "Amazing Feature" --update

# Create PR including both code and documentation
```

## Feature Documentation Standards

Feature documentation should:

1. **Be User-Focused**: Written primarily for users of the feature
2. **Be Technically Accurate**: Include correct implementation details
3. **Include Examples**: Show how to use the feature
4. **Discuss Limitations**: Be honest about any constraints
5. **Suggest Improvements**: Include ideas for future enhancements

## Feature Documentation Maintenance

After a feature is merged to main:

1. Keep the documentation updated as the feature evolves
2. Add cross-references to related features
3. Move outdated information to an "Historical Notes" section
4. Update usage examples as APIs change