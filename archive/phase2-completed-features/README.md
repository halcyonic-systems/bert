# BERT Features Documentation

This directory contains documentation for individual features implemented in BERT. Each feature has its own markdown file with standardized documentation.

## Available Features

<!-- This section will be populated as features are documented -->

## Adding Feature Documentation

Feature documentation should be generated using the `gen-feature-docs.sh` script:

```bash
# From the repository root
./scripts/gen-feature-docs.sh "Feature Name"
```

This will create a new markdown file in this directory using the standardized template.

## Documentation Format

Each feature documentation file follows a standard format including:

- Feature overview and status
- Description and implemented functionality
- Technical implementation details
- Usage examples
- Testing strategy
- Future improvements
- Related documentation

## Updating Documentation

Feature documentation should be updated in these key scenarios:

1. When a feature is first implemented
2. When significant changes are made to a feature
3. Before a feature is merged to main
4. When user-facing behavior changes

## Integration with Development Workflow

To integrate feature documentation into your development workflow:

1. Create feature documentation at the start of development
2. Update it as the feature evolves
3. Review documentation as part of code reviews
4. Keep documentation updated after feature release

## Related Documentation

- [Contributing Guide](../contributing/contributing.md)
- [Feature Template](../contributing/feature-template.md)
- [Documentation Standards](../contributing/documentation-implementation-analysis.md)