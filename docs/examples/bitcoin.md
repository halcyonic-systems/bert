# Example: Bitcoin Network

A Deep Systems Analysis of the Bitcoin network using the BERT 8-tuple framework `S = <C, N, E, G, B, T, H, dt>`.

This model ships as a bundled example in BERT v0.4.0+ and serves as the **reference model** throughout the [system-language-spec.md](../system-language-spec.md). It decomposes Bitcoin into four core subsystems across three hierarchical levels, with fully specified boundary interfaces, external flows, and internal coordination. The analysis is grounded in the working paper ["A Deep Systems Analysis of Bitcoin"](https://www.researchhub.com/paper/9310259/a-deep-systems-analysis-of-bitcoin) (Thornton, 2025).

**Model file**: `assets/models/examples/bitcoin.json`

## System Definition

| Field | Value |
|-------|-------|
| **Name** | Bitcoin |
| **Complexity** | Complex (adaptable and evolvable) |
| **Environment (E)** | Global Financial Network |
| **Time unit (dt)** | Second |
| **Boundary porosity** | 0.0 (cryptographically precise -- a transaction is valid or it is not) |
| **Perceptive fuzziness** | 0.0 (consensus rules are deterministic) |

## 8-Tuple Mapping

Mapped against the formal structure from [mobus-reference.md](../mobus-reference.md):

| Symbol | Element | Bitcoin instantiation |
|--------|---------|-----------------------|
| **C** | Components | Mining, Validating, Protocol, Development (4 primary subsystems) + 3 interface subsystems (Software Wallet, GitHub Interface, Node RPC Interface) |
| **N** | Network | 5 internal flows: Protocol Updates, Mempool Transactions, Mined Blocks, Protocol Rules & Parameters, Block & State Updates, plus interface relay flows |
| **E** | Environment | Global Financial Network -- Users, Power Grid, GitHub (sources); Environment/heat, Users (sinks) |
| **G** | External interactions | Transaction requests (import), electricity (import), code contributions (import), confirmed transactions (export), waste heat (export) |
| **B** | Boundary | Bitcoin Network Boundary with 5 interfaces governing all flow crossing |
| **T** | Transformation | Protocols on each interface: JSON-RPC (wallet, RPC), AC-Grid (power), Git-PR (contributions), Thermal-radiation (heat) |
| **H** | Hierarchy | 3-level decomposition: Level 0 (Bitcoin), Level 1 (4 subsystems), Level 2 (10 sub-subsystems) |
| **dt** | Time scale | Second |

## Components (C) -- Level 1

### Mining (C0.1)

Secures the network through proof-of-work. Miners validate transactions, compete to create new blocks, and introduce new bitcoins into circulation according to the halving schedule. Decomposes into:

- **Hash Production** (C0.1.0) -- Physical ASIC infrastructure producing computational work. Consumes electricity, produces hash attempts and waste heat.
- **Block Assembly** (C0.1.1) -- Selects transactions from mempool, constructs block templates, coordinates reward distribution. Makes economic decisions (fee optimization, empty vs. full block tradeoffs). Archetype: **Economy** (see [archetypes.md](../archetypes.md)).

Internal flow: **Valid Proof** (Message/Signal) -- successfully computed hash meeting difficulty target, flowing from Hash Production to Block Assembly for block finalization.

### Validating (C0.2)

Distributed network of full nodes maintaining and verifying the blockchain. Validates transactions and blocks, relays information, enforces consensus rules. Decomposes into:

- **Mempool** (C0.2.0) -- Pool of unconfirmed transactions. Validates against consensus rules and local policy, maintains fee-sorted ordering, handles eviction at capacity.
- **Block Processor** (C0.2.1) -- Validates incoming blocks, updates local chain view, checks proof-of-work, manages chain reorganizations. Archetype: **Governance** (see [archetypes.md](../archetypes.md)).

### Protocol (C0.5)

Core rules engine enabling decentralized coordination without central authority. Defines how transactions and blocks are structured, validated, and propagated. Decomposes into:

- **Consensus Rules** (C0.5.0) -- Deterministic validity conditions: difficulty adjustment, block weight limits, script verification, coinbase maturity, halving schedule. Changes require network-wide coordination (soft/hard forks). Archetype: **Governance**.
- **Network Layer** (C0.5.1) -- Peer-to-peer communication: peer discovery, gossip protocol, compact block relay, message serialization.
- **Chain State** (C0.5.2) -- Maintains the UTXO set, block index, and chain tip. Handles reorgs and tracks cumulative work for chain selection.

### Development (C0.4)

Decentralized group of engineers, cryptographers, and scientists maintaining Bitcoin Core and the BIP process. Decomposes into:

- **Protocol Research** (C0.4.0) -- Problem identification, solution design, BIP authoring. Archetype: **Agent**.
- **Code Implementation** (C0.4.1) -- Translates designs into working software (patches, tests, documentation). Archetype: **Agent**.
- **Review & Governance** (C0.4.2) -- Peer review, security audit, merge decisions. No formal voting -- rough consensus through sustained technical discussion. Archetype: **Governance**.

### Interface Subsystems

Three subsystems sit on the boundary, mediating between external entities and internal components. Each has a `parent_interface` linking it to a specific boundary interface (see [system-language-spec.md](../system-language-spec.md) for the interface subsystem pattern):

| Interface subsystem | Boundary interface | Protocol | Direction |
|--------------------|--------------------|----------|-----------|
| Software Wallet (C0.52) | I0.52 | JSON-RPC | Import (transaction requests from Users) |
| GitHub Interface (C0.51) | I0.51 | Git-PR | Import (code contributions from GitHub) |
| Node RPC Interface (C0.53) | I0.53 | JSON-RPC | Export (confirmed transactions to Users) |

## Environment and External Interactions (E, G)

### Sources

| Source | Substance type | Flow | Parameters |
|--------|---------------|------|------------|
| **Users** | Message/Transaction | Transaction Requests | Request acceptance rate (%), total pending value (BTC), avg transaction size (bytes) |
| **Power Grid** | Energy/Electricity | Electricity | Daily consumption (TWh), efficiency (J/hash), geographic distribution (%), energy source mix (%), power density (W/TH/s) |
| **GitHub** | Message/Contribution | Contributors | PR rate (PRs/month), merge rate, BIP submission rate, active developers (unique/month) |

### Sinks

| Sink | Substance type | Flow | Parameters |
|------|---------------|------|------------|
| **Environment** | Energy/Thermal | Waste Heat | Thermal radiation from mining and infrastructure |
| **Users** | Message/Data | Confirmed Transactions | Block size (MB), transactions/block, throughput (s), total value (BTC), fee rate (sats/vbyte), block propagation (s) |

Flow substance types follow the three-type taxonomy in [mobus-reference.md](../mobus-reference.md): the Bitcoin system processes **Messages** (transactions, blocks, code contributions), consumes **Energy** (electricity), and exports **Energy** as waste (heat). No Material flows -- Bitcoin is a purely digital system.

## Network (N) -- Internal Flows

| Flow | From | To | Type | Substance | Usability |
|------|------|----|------|-----------|-----------|
| Protocol Updates | Development | Protocol | Flow | Message/Code | Resource |
| Mempool Transactions | Validating | Mining | Flow | Message/Transaction | Product |
| Mined Blocks | Mining | Protocol | Flow | Message/Data | Product |
| Protocol Rules & Parameters | Protocol | Mining | **Force** | Message/Consensus-Rules | Product |
| Block & State Updates | Protocol | Validating | Flow | Message/Data | Product |
| Code Contributions | GitHub Interface | Development | Flow | Message/Contribution | Resource |
| User Transaction Submissions | Software Wallet | Validating | Flow | Message/Transaction | Resource |
| Confirmation Broadcast | Validating | Node RPC Interface | Flow | Message/Signal | Product |

Note the distinction between Flow and **Force**: Protocol Rules & Parameters is typed as a Force because it constrains Mining's behavior rather than carrying a consumable substance. Forces represent governance and regulatory interactions -- see [simulation.md](../simulation.md) for how Forces affect agent behavior differently from Flows in simulation.

## Boundary (B) and Transformation (T)

The Bitcoin Network Boundary has five interfaces, each with a specific protocol (the T element):

| Interface | Protocol | Type | Connected entity |
|-----------|----------|------|-----------------|
| Heat Dissipation (I0.0) | Thermal-radiation | Export | Environment (sink) |
| Transformers (I0.3) | AC-Grid | Import | Power Grid (source) |
| GitHub Interface (I0.51) | Git-PR | Import | GitHub (source) |
| Software Wallet (I0.52) | JSON-RPC | Import | Users (source) |
| Node RPC Interface (I0.53) | JSON-RPC | Export | Users (sink) |

Porosity = 0.0 and perceptive fuzziness = 0.0 reflect Bitcoin's cryptographic precision: the boundary is perfectly sharp. A transaction either satisfies consensus rules or it does not. There is no ambiguity about what is inside the system versus outside.

## Hierarchy (H) -- Decomposition Summary

```
Level 0: Bitcoin (S0)
  Level 1:
    Mining (C0.1)
      Level 2: Hash Production (C0.1.0), Block Assembly (C0.1.1)
    Validating (C0.2)
      Level 2: Mempool (C0.2.0), Block Processor (C0.2.1)
    Development (C0.4)
      Level 2: Protocol Research (C0.4.0), Code Implementation (C0.4.1), Review & Governance (C0.4.2)
    Protocol (C0.5)
      Level 2: Consensus Rules (C0.5.0), Network Layer (C0.5.1), Chain State (C0.5.2)
    [Interface subsystems: Software Wallet (C0.52), GitHub Interface (C0.51), Node RPC Interface (C0.53)]
```

This is a 3-level recursive decomposition following DSA methodology (see [mobus-reference.md](../mobus-reference.md)). Each Level 2 subsystem could be further decomposed -- for instance, Hash Production could break down into individual ASIC clusters, cooling systems, and power delivery units.

## Systems Science Insights

### Emergent decentralization

Decentralization and censorship resistance are not properties of any single subsystem. They emerge from the interaction pattern: Mining provides security, Validating enforces rules, Protocol defines consensus, Development evolves the system. Remove any one and the emergent property degrades. This is a concrete instance of Bertalanffy's principle that system-level properties arise from component interactions, not component properties.

### Force vs. Flow distinction

The Protocol Rules & Parameters interaction is typed as a **Force**, not a Flow. This captures a real asymmetry: consensus rules do not get "consumed" by Mining the way mempool transactions do. They constrain behavior without being depleted. This distinction matters for [simulation](../simulation.md) -- Forces modify agent decision functions rather than filling resource buffers.

### Archetype distribution

The model uses three archetypes from the [archetype framework](../archetypes.md):
- **Economy** (Block Assembly) -- resource allocation and optimization under constraints
- **Governance** (Block Processor, Consensus Rules, Review & Governance) -- rule enforcement and legitimacy
- **Agent** (Protocol Research, Code Implementation) -- autonomous decision-making and creative production

This distribution tells a story: Bitcoin's core loop is governance-heavy (three Governance subsystems vs. one Economy), reflecting a system where rule enforcement dominates resource allocation.

### Boundary as cryptographic membrane

Zero porosity and zero perceptive fuzziness are unusual in natural systems but characteristic of cryptographic protocols. The boundary is not porous or fuzzy -- it is a mathematical predicate. This makes Bitcoin an ideal test case for BERT's boundary model because the boundary properties can be stated precisely.

## Research Foundation

Based on: Thornton, S. (2025). ["A Deep Systems Analysis of Bitcoin."](https://www.researchhub.com/paper/9310259/a-deep-systems-analysis-of-bitcoin) Working paper applying DSA methodology to understand Bitcoin as a complex adaptive system operating at the intersection of physical, digital, and social realms.

## Future Directions

- **Ethereum Ecosystem Model** -- Analysis of Ethereum's multi-layer architecture including execution layer, consensus layer, and DeFi protocols.
- **DAO Governance Systems** -- Organizational structures in decentralized autonomous organizations with voting mechanisms and treasury management.

These systems offer unique value for systems science: purely digital structure, transparent state changes, rapid governance evolution, and natural experiments in system design theory.

## Try It Yourself

1. **Load model**: Open `assets/models/examples/bitcoin.json` in BERT's Model Browser.
2. **Trace the main loop**: Follow Transaction Requests from Users through Software Wallet, into Validating, through Mining, into Protocol, and back out as Confirmed Transactions through Node RPC Interface.
3. **Inspect the Force**: Click Protocol Rules & Parameters to see how consensus constraints flow differently from data flows.
4. **Zoom into Mining**: Explore the Level 2 decomposition -- Hash Production and Block Assembly -- and the Valid Proof flow between them.
5. **Check archetypes**: Compare the Governance-typed subsystems with the Economy and Agent subsystems to see how functional roles distribute across the system.
