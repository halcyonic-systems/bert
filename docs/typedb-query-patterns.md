# TypeDB Query Patterns for BERT Models

Canonical reference for cross-model TypeQL queries against BERT models transpiled into TypeDB. Companion to [`bert-typedb-schema.md`](./bert-typedb-schema.md) (which defines the schema) — this document covers what to *do* with the typed graph once models are loaded.

## Prerequisites

- TypeDB 3.x server running on `localhost:1729` (start: `typedb server`)
- `bert-typedb` binary built: `cargo build -p bert-typedb --release` (from bert repo root)
- One or more BERT JSON models transpiled into a database, e.g.:
  ```sh
  ./target/release/bert-typedb assets/models/examples/bitcoin.json --db bert-cross-model-demo
  ./target/release/bert-typedb assets/models/examples/ethereum.json --db bert-cross-model-demo --skip-schema
  ```
  Each model lives under its own `model_name` namespace prefix (`bitcoin:`, `ethereum:`, etc.) — no collisions.

## Running Queries

The TypeDB 3.x console requires a script file for non-trivial queries (the `--command "transaction read"` flag mode doesn't expose the `query` sub-command). The pattern:

```sh
cat > /tmp/q.tql <<'EOF'
transaction read <db-name>
    match
      <pattern>;
    fetch {
      <projection>
    };

    close
exit
EOF

typedb console \
  --address http://localhost:1729 \
  --tls-disabled --diagnostics-disable \
  --username admin --password password \
  --script /tmp/q.tql 2>&1 | grep -E '"(name|id|archetype)":'
```

The `grep` at the end extracts just the JSON answer fields and skips the streaming-protocol noise. Adjust the field list per query.

## Pattern Catalog

### 1. Single-Model Queries (sanity check after transpile)

Confirm a model loaded correctly by listing all its systems:

```typeql
match
  $s isa system, has bert_id $id, has display_name $name;
  $id like "^bitcoin:.*";
fetch { "id": $id, "name": $name };
```

Substitute the model name prefix to inspect any chain.

### 2. Cross-Model Archetype Distribution

Find every system across all loaded models grouped by HCGS archetype (per Mobus Ch. 10–13):

```typeql
match
  $s isa system, has archetype $a, has bert_id $id, has display_name $name;
fetch {
  "id": $id,
  "name": $name,
  "archetype": $a
};
```

Empirically demonstrated on `bert-cross-model-demo` (4 blockchains): returns 17 archetyped systems — 5 Agent, 4 Economy, 8 Governance — across bitcoin/ethereum/cosmos/solana. Reveals which systems share HCGS classification across radically different domains.

### 3. Force Interaction Discovery (cross-domain structural pattern)

Find every Force interaction (per SL §1.7 — influence without substance transfer):

```typeql
match
  $i isa interaction,
  has interaction_type "Force",
  has bert_id $id,
  has display_name $name;
fetch {
  "id": $id,
  "name": $name
};
```

Empirically on the 4-blockchain database, returns one Force per chain, all named some variant of "Rules & Parameters". Combined with the LLM model (in `llm-redesign-test*` databases): the LLM's Trained Weights → Transformer Stack Force exhibits the same structural pattern as the blockchain Protocol → Validating Forces. This is the cross-domain knowledgebase-constrains-computation pattern (Mobus parallel: DNA → ribosome protein production).

### 4. Level-Filtered Queries

Find level-1 subsystems with a specific archetype across all chains:

```typeql
match
  $s isa system,
  has archetype "Economy",
  has system_level 1,
  has bert_id $id,
  has display_name $name;
fetch { "id": $id, "name": $name };
```

Useful for comparing "what each model considers an Economy at the immediate sub-S0 level."

### 5. Substance-Type Filtering

Find all interactions of a particular substance sub_type:

```typeql
match
  $i isa interaction,
  has substance_sub_type "Consensus-Rules",
  has bert_id $id,
  has display_name $name;
fetch { "id": $id, "name": $name };
```

Example sub_types from the SL §1.7 controlled vocabulary:
- `Consensus-Rules` — protocol-level constraint (blockchain Force pattern)
- `Training-Objective` — loss-function constraint (LLM Force pattern, new vocab term)
- `Data`, `Signal`, `Transaction`, `Code`, `Contribution` — common Message sub_types
- `Electricity`, `Thermal`, `Bandwidth` — Energy sub_types

### 6. Known-Limitation Patterns (use sparingly)

Some patterns aren't yet supported because the corresponding TypeDB inference functions exist as schema declarations but have not been exercised:

- **`cross_model_equivalents()`** — declared in schema, intended to use `is_same_as_id` linkage to identify equivalent external entities across models. Not yet wired up; running queries that depend on it will return empty results until the inference layer is exercised.
- **Containment traversal** (subsystems-of-subsystems): TypeDB supports recursive traversal but BERT-specific patterns (e.g. "all leaf-level subsystems under S0") are not yet idiomatized in this catalog.

## Workflow Tips

- **Always use a fresh `--db` for experimental queries** — don't pollute the canonical `bert-cross-model-demo` with one-off models.
- **Re-run the same query against multiple databases** to compare model populations — useful when iterating on a model and wanting to confirm the change reflects in the typed graph.
- **`grep` the JSON keys, not values** — TypeDB's streaming output interleaves protocol messages with the JSON answers; filtering by `'"<key>":'` cleanly extracts only the document fields.
- **Free-form TypeQL works in the console interactively too** — but the script-file pattern is the only one that survives copy-paste and is reliable across re-runs.

## When to Add a Pattern Here

Add a new section when:
- A query returns insight that warranted a session conversation
- The query exercises a TypeDB feature (inference, recursion, aggregation) not yet documented
- A pattern is needed for a recurring task (model validation, cross-model regression check, batch inspection)

Don't add ephemeral one-off queries — those belong in session reference files.

## Related Docs

- [`bert-typedb-schema.md`](./bert-typedb-schema.md) — TypeDB schema definitions, attributes, relations
- [`bert-schema-reference.md`](./bert-schema-reference.md) — canonical BERT JSON schema (input format)
- [`system-language-spec.md`](./system-language-spec.md) — SL spec v0.1 (semantics of Force, sub_type vocabularies, etc.)
- `.claude/skills/bert-query/SKILL.md` (in halcyonic vault) — Claude-invocable wrapper that automates this workflow
