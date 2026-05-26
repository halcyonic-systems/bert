# BERT Documentation

## For Users

| Doc | What it covers |
|-----|---------------|
| [Getting Started](getting-started.md) | Install, first model, controls reference |
| [Generator Pipeline](generator.md) | NL-to-model: extraction, repair, deterministic compilation |
| [Simulation](simulation.md) | Mesa bridge, process primitives, step cycle, dashboard |
| [Archetypes](archetypes.md) | HCGS classification, agent behavioral types, decision guide |

## Examples

| Doc | System type |
|-----|------------|
| [Cell](examples/cell.md) | Biological |
| [Ecosystem](examples/ecosystem.md) | Ecological |
| [Organization](examples/organization.md) | Social |
| [Solar Panel](examples/solar-panel.md) | Technological |
| [LLM](examples/llm.md) | Meta-system |
| [Bitcoin](examples/bitcoin.md) | Cryptoeconomic |

These are also available as interactive models in BERT's Model Browser.

## Core References

| Doc | What it covers |
|-----|---------------|
| [System Language Spec](system-language-spec.md) | Formal SL specification v0.1 (flagship) |
| [Schema Reference](bert-schema-reference.md) | JSON schema, validation layers, ID conventions |
| [Mobus Reference](mobus-reference.md) | 8-tuple quick reference |
| [TypeDB Schema](bert-typedb-schema.md) | TypeDB graph projection schema |
| [TypeQL Patterns](typedb-query-patterns.md) | Cross-model TypeQL query examples |

## Theory

| Doc | What it covers |
|-----|---------------|
| [Process Primitives](process-primitives.md) | 9+1 primitives with T-functions and substance constraints |
| [H-Element Theory](h-element-theory.md) | History → knowledge connection, implementation sketches |
| [Simulation–Linear Algebra Bridge](simulation-linalg-bridge.md) | Maps every sim concept to its linear algebra dual |
| [Lifecycle Dynamics](lifecycle-dynamics.md) | System lifecycle phases (post-v0.4 theory) |

## Development

| Doc | What it covers |
|-----|---------------|
| [BUILD.md](BUILD.md) | Build commands, release workflow, CI/CD |
| [CLAUDE.md](CLAUDE.md) | AI assistant context, architecture patterns, gotchas |
| [Documentation Guidelines](documentation-guidelines.md) | Rust doc templates for modules/functions/types |
| [Feature Specs](features/) | Agent system, chat, process primitive visualization |

## Audiences

- **Users**: Getting Started → Examples → Simulation
- **Developers**: BUILD.md → CONTRIBUTING.md → Feature Specs
- **Researchers**: System Language Spec → Mobus Reference → Archetypes → Theory
