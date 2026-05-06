# Info Processing Suite — BERT JSON Prototyping Guide

A scoped plan for creating 9 BERT models of computational instruments — one for each section of the Instruments essay, plus a composite case — demonstrating that the same 40 typed primitives used to model Bitcoin, Ethereum, Cosmos Hub, and Solana also formally specify the AI systems that operate *on* those systems.

## Why These 9

The Instruments essay (halcyonic.systems/instruments) organizes computational tools by what people think they do: Recognition, Prediction, Generation, Reasoning, Decision, Discovery. Each section shows a class of tools, then names the ceiling — the question none of them ask.

These 9 models answer the question *about* those tools. Each one is itself an information processing system. Each one has components, boundaries, flows, interfaces, and mechanisms — the same CESM structure as any governance system or blockchain protocol. The typed lens doesn't just describe them. It reveals architectural facts that prose descriptions obscure.

**Existing**: `llm.json` — Large Language Model (§03 Generation)

**To build** (in order):

| #   | System                       | Essay §         | Why it resonates                                                                                  |
| --- | ---------------------------- | --------------- | ------------------------------------------------------------------------------------------------- |
| 1   | Vision Transformer           | §01 Recognition | "Seeing" is feature extraction — the decomposition shows no governance subsystem at all           |
| 2   | State Space Model (Mamba)    | §02 Prediction  | Architecturally alien to transformers, captures temporal dependencies without understanding cause |
| 3   | Diffusion Model              | §03 Generation  | The other major generative paradigm — noise reversal, not token prediction                        |
| 4   | Reinforcement Learning Agent | §05 Decision    | RLHF is how LLMs get aligned — every AI safety researcher lives here                              |
| 5   | Graph Neural Network         | §04 Reasoning   | Computes over structure — modeling it AS structure is the meta-move                               |
| 6   | Neuro-Symbolic System        | §04 Reasoning   | The instrument closest to specification — where learning meets authored rules                     |
| 7   | Causal Discovery Algorithm   | §06 Discovery   | The instrument closest to *authoring* structure — and where it stops short                        |
| 8   | RAG System                   | §07 Composite   | Not a single instrument but a composed system of instruments — Facets IS one                      |

Together with the LLM, the suite covers every essay section: Recognition, Prediction, Generation (×2), Reasoning (×2), Decision, Discovery, and Composition.

---

## 1. Vision Transformer (ViT / DINOv2 / I-JEPA)

**Essay reference**: §01 — "Recognition tells you what patterns exist in the data. It cannot tell you what kind of system produced those patterns."

**Why this one**: Vision is the capability people encounter first — "it can see." The Vision Transformer replaced CNNs as the dominant recognition architecture by treating images as sequences of patches and learning relational structure via attention. Every multimodal AI system (GPT-4V, Gemini, Claude's vision) has a ViT-descendant at its perceptual front end. Modeling it reveals: pure perception, no governance. That absence IS the essay's point.

### Likely Decomposition

**Environment**: Visual perception infrastructure — image data pipeline, compute cluster, downstream task consumers.

**Sources**: Raw image (pixel grid), task labels (for supervised training), augmentation parameters (cropping, flipping, color jitter)

**Sinks**: Feature embeddings (high-dimensional representation), classification output (if task head attached), computational energy

**Subsystems** (5):

| Subsystem | Role | Archetype | Why this classification |
|-----------|------|-----------|------------------------|
| Patch Embedding Layer | Divides image into fixed-size patches, projects each to a vector | Economy | Allocates representational capacity — decides how to partition the visual field |
| Positional Encoder | Adds spatial information to patch embeddings — where each patch came from | Economy | Allocates spatial meaning — without this, the model sees a bag of patches with no layout |
| Self-Attention Stack | Learns which patches are relevant to which other patches via multi-head attention | **Agent** | The active computation — decides what visual relationships matter |
| Layer Normalization / MLP Block | Stabilizes and transforms representations between attention layers | Economy | Allocates gradient flow — the metabolic regulation of the processing pipeline |
| Classification / Feature Head | Maps final representation to task output (class label, embedding, detection) | **Agent** | Decides the output — the recognition verdict |

### What the Typed Lens Reveals

**No governance subsystem.** This is the structural fingerprint of pure perception. The ViT has agents (attention, classification) and economy (embedding, normalization), but nothing that validates, constrains, or enforces. It recognizes whatever patterns exist in the data. It cannot say whether those patterns are meaningful, spurious, or dangerous.

Compare to the RL Agent (which has the reward model as governance) or the Neuro-Symbolic system (which has the rule engine). The ViT's architectural *lack* of governance is what makes it a recognition system rather than a reasoning or decision system. The typed decomposition makes this absence explicit and nameable — not just "it can't reason" but "there is no governance subsystem in the architecture."

This also explains why adversarial examples work: with no governance subsystem to validate whether the recognized pattern is plausible, the attention mechanism can be steered by imperceptible perturbations. The system has no internal check on its own perceptions.

### Key Interfaces to Specify

- Patch Embedding → Positional Encoder: raw patch vectors awaiting spatial annotation
- Positional Encoder → Self-Attention Stack: spatially-annotated patch representations
- Self-Attention Stack (internal): layer-to-layer residual connections + normalized representations
- Self-Attention Stack → Feature Head: final-layer [CLS] token or pooled patch representations
- Feature Head → Output: task-specific prediction or embedding vector

### Generation Notes

The simplest architecture in the suite — use it as a warm-up. The key modeling decision: how to handle the [CLS] token (a special "summary" token prepended to the patch sequence). It's not a subsystem — it's a flow artifact that accumulates information across attention layers. Model it as a flow, not a component.

---

## 2. State Space Model (Mamba / S4 / S5)

**Essay reference**: §02 — "Predicts if nothing changes. Cannot tell you what would happen if you intervened."

**Why this one**: State space models are the most architecturally radical alternative to transformers. Where transformers compute pairwise attention (quadratic in sequence length), SSMs evolve a hidden state through structured linear dynamics (linear in sequence length). Mamba added input-dependent selection, making it competitive with transformers on language tasks. The decomposition reveals: the state evolution IS the model's memory, but it's learned, not authored — it tracks temporal patterns without knowing what causes them.

### Likely Decomposition

**Environment**: Sequential data processing infrastructure — time-series feeds, language streams, compute cluster.

**Sources**: Input sequence (tokens, sensor readings, time-series values), initial state (zero or learned), task specification

**Sinks**: Output sequence (predictions, classifications per timestep), final hidden state, computational energy

**Subsystems** (5):

| Subsystem | Role | Archetype | Why this classification |
|-----------|------|-----------|------------------------|
| Input Projection | Maps raw input to model dimension — the front door | Economy | Allocates: decides what information enters the state evolution |
| State Evolution Engine | Evolves hidden state via structured matrices (A, B, C, D) — the core SSM | **Agent** | The active processor — this IS the model. It decides how state changes over time |
| Selective Gating Mechanism | Input-dependent filtering — Mamba's key innovation. Decides what to remember and what to forget at each step | **Governance** | Validates what enters the state — not all input gets through. The gate constrains. |
| Normalization / Residual Stack | Stabilizes multi-layer state evolution, manages gradient flow | Economy | Metabolic regulation — same role as in the ViT |
| Output Projection | Maps hidden state to task-specific output at each timestep | **Agent** | Decides the prediction — the forecasting verdict |

### What the Typed Lens Reveals

The Selective Gating Mechanism as **Governance** is the key difference from the ViT. The ViT has no governance — it processes everything. Mamba's selection mechanism actively filters: at each timestep, it decides what information is worth incorporating into the state and what to discard. This is validation — it constrains what the state evolution engine sees.

This is why Mamba competes with transformers on language despite having no attention mechanism. Attention asks "what's relevant to what?" (an Agent question). Selection asks "should this get through?" (a Governance question). Different architectural answers to the same problem — and the typed decomposition makes the difference structural.

The state evolution engine itself is fascinating: it's a dynamical system with learned parameters (A, B, C, D matrices). The state carries temporal information across arbitrary distances. But the dynamics are learned from data — no one authored a mechanistic claim about what the state *means*. It predicts regime shifts it cannot explain. That's §02's ceiling, made architectural.

### Key Interfaces to Specify

- Input Projection → Selective Gate: projected input for gating decision
- Input Projection → State Evolution: projected input for state update
- Selective Gate → State Evolution: binary/soft gate signal determining what passes
- State Evolution (internal): state-to-state transitions across timesteps via structured matrices
- State Evolution → Output Projection: hidden state at each timestep for prediction

### Generation Notes

The state evolution matrices (A, B, C, D) are the architectural heart — consider whether to model them as a subsystem or as parameters of the State Evolution Engine. Recommendation: parameters, not a subsystem. They're learned weights, not a processing unit. The distinction matters — they're closer to the LLM's weight matrices than to a functional component.

The temporal structure is richer here than in any other model in the suite. Set `time_constant` to reflect the multi-scale temporal dynamics: the gate operates at "token" scale (each input), the state evolution at "sequence" scale (accumulated over the full input), and training at "epoch" scale.

---

## 3. Reinforcement Learning Agent (PPO/RLHF)

**Essay reference**: §05 — "Optimization without specification is a powerful engine with no destination."

**Why this one**: RLHF (Reinforcement Learning from Human Feedback) is the mechanism that turns a raw LLM into an aligned assistant. Every alignment researcher, every safety team, every policy paper on AI governance references this architecture. Modeling it with typed structure reveals where the *governance* actually lives.

### Likely Decomposition

**Environment**: Training infrastructure — simulator or real environment providing state observations and accepting actions.

**Sources**: State observations (environment percepts), reward signal (scalar feedback), human preference data (for RLHF variant)

**Sinks**: Action output (policy decisions), training loss (gradient updates), computational energy (GPU heat)

**Subsystems** (5):

| Subsystem | Role | Archetype | Why this classification |
|-----------|------|-----------|------------------------|
| Policy Network | Maps states to action probabilities | **Agent** | This is the deciding subsystem — it chooses |
| Value Estimator | Predicts expected cumulative reward from any state | Economy | Allocates attention by estimating value — the "is this worth pursuing" signal |
| Reward Model | Encodes human preferences as a scalar signal (RLHF) | **Governance** | Validates and constrains — it doesn't decide what to do, it judges what was done |
| Experience Replay Buffer | Stores and samples past (state, action, reward) tuples | Economy | Allocates training signal across time — prioritized replay IS resource allocation |
| Environment Interface | Translates between agent actions and environment states | Agent | Mediates interaction — the sensorimotor boundary |

### What the Typed Lens Reveals

The reward model is classified as **Governance**, not Agent. This is non-obvious and important. In the RLHF literature, the reward model is often described as "teaching the model what humans want." But structurally, it validates — it doesn't decide. It constrains the policy network's action space. It enforces alignment. That's governance. The architectural implication: if your reward model is poorly specified, you haven't made a bad decision — you've created a governance failure. Different diagnosis, different fix.

The value estimator is **Economy** — it allocates by estimating. It answers "how much is this state worth?" which is a resource allocation question, not a decision question. The policy network consumes that estimate to make decisions. Separating these two functions (valuation vs. decision) via archetype is exactly the kind of structural clarity that gets lost in monolithic descriptions.

### Key Interfaces to Specify

- Policy → Environment Interface: action encoding protocol, action space constraints
- Environment Interface → Policy: state observation encoding, partial observability handling
- Reward Model → Policy: scalar reward signal, but also gradient flow during RLHF training
- Value Estimator → Policy: advantage estimates (how much better is this action than average?)
- Replay Buffer → Policy + Value: mini-batch sampling, priority weighting

### Generation Notes

Use `bert-json-creation` skill. Reference `llm.json` for structural template. Key differences from LLM model:
- The LLM has no governance subsystem (no internal validator). The RL agent does — the reward model.
- The LLM's flows are primarily Message type. The RL agent has Message (observations), Energy (compute), AND a feedback loop (reward → policy update) that the LLM lacks at inference time.
- Environment sources/sinks are more active — the RL agent changes its environment, the LLM doesn't.

---

## 4. Diffusion Model (Stable Diffusion / DDPM)

**Essay reference**: §03 — "The difference between 'a plausible bridge' and 'a bridge that carries this load across this span.'"

**Why this one**: Diffusion models are architecturally alien to transformers. They reverse a noise process rather than predicting next tokens. The decomposition reveals a fundamentally different information processing pattern — iterative refinement within a latent space, where the boundary between "noise" and "signal" shifts at every timestep. AI researchers who work on both paradigms will immediately see that the typed structure captures architectural differences that prose hand-waves.

### Likely Decomposition

**Environment**: Image/media generation infrastructure — training data distribution, compute cluster, user prompt interface.

**Sources**: Text prompt (natural language conditioning), noise sample (random initialization from Gaussian), training images (distribution to learn)

**Sinks**: Generated image (denoised output), computational energy, latent representations (for downstream use)

**Subsystems** (5):

| Subsystem | Role | Archetype | Why this classification |
|-----------|------|-----------|------------------------|
| Noise Scheduler | Controls the forward/reverse noise process — how much noise at each timestep | **Governance** | Sets the rules of the denoising game — doesn't generate, it regulates the process |
| U-Net Denoiser | Predicts and removes noise at each timestep | **Agent** | The active processor — it makes the denoising decisions |
| Text Encoder (CLIP) | Converts text prompts to conditioning vectors | Economy | Translates between modalities — allocates semantic meaning to guide generation |
| Latent Space (VAE Encoder) | Compresses pixel space to lower-dimensional latent space | Economy | Dimension reduction IS allocation — deciding what information to keep |
| Image Decoder (VAE Decoder) | Reconstructs full-resolution image from latent representation | Agent | Executes the final transformation — latent → pixel |

### What the Typed Lens Reveals

The noise scheduler as **Governance** is the key insight. It doesn't generate anything. It controls the process by which generation happens — setting the variance schedule, determining how much noise to add/remove at each step, defining when the process terminates. This is regulatory, not creative. The U-Net does the creative work (denoising) within the constraints the scheduler sets.

This decomposition also reveals that diffusion models have a *temporal boundary structure* the LLM lacks. At timestep t=1000 (pure noise), the boundary between signal and noise is at maximum opacity. At t=0 (clean image), it's fully transparent. The boundary itself is a dynamic typed entity. This is hard to express in prose but natural in BERT's boundary/porosity fields.

### Key Interfaces to Specify

- Noise Scheduler → U-Net: timestep embedding, noise level specification
- Text Encoder → U-Net: cross-attention conditioning vectors
- VAE Encoder → U-Net: latent space representation (reduced dimensionality)
- U-Net → VAE Decoder: denoised latent at t=0
- Noise Scheduler → Latent Space: forward process noise injection (training)

### Generation Notes

The iterative timestep structure is unusual. Consider setting `time_constant` differently across subsystems — the scheduler operates at the "epoch" scale (full denoising trajectory), the U-Net at the "step" scale (single denoising operation). This temporal hierarchy is a feature of the typed model, not a detail to flatten.

---

## 5. Graph Neural Network (GCN/GAT/GraphSAGE)

**Essay reference**: §04 — "Learns what the network looks like. Cannot say what the network is for."

**Why this one**: The meta-move. A GNN computes over graph structure — it propagates information through nodes and edges to learn representations. Modeling a GNN with BERT means using typed structure to specify a system that itself operates on structure. AI researchers in the graph ML community will find this irresistible: the tool that learns structure, formally specified AS structure.

Also directly relevant to your network science thesis — GNNs are the computational instruments your thesis work sits alongside.

### Likely Decomposition

**Environment**: Graph computation infrastructure — input graph data, feature matrices, GPU/TPU cluster.

**Sources**: Input graph (adjacency structure), node features (attribute vectors), edge features (relationship attributes), task labels (for supervised settings)

**Sinks**: Node embeddings (learned representations), graph-level prediction (classification/regression output), computational energy

**Subsystems** (5):

| Subsystem | Role | Archetype | Why this classification |
|-----------|------|-----------|------------------------|
| Node Embedding Layer | Initial feature transformation — maps raw node attributes to learned representations | Economy | Allocates representational capacity — decides what dimensions to use |
| Message Passing Stack | Propagates information between neighbors via learned aggregation | **Agent** | The active computation — decides what information flows where |
| Attention/Aggregation Module | Weights neighbor messages by learned importance (GAT variant) | **Governance** | Validates which messages matter — constrains the message passing |
| Readout/Pooling Layer | Aggregates node-level representations into graph-level representation | Economy | Allocates: reduces many node representations to one graph representation |
| Task Head | Maps final representations to task-specific output (classification, regression, link prediction) | **Agent** | Decides the output — the prediction subsystem |

### What the Typed Lens Reveals

The message passing protocol IS a typed interface. In a GNN, each layer defines: what message each node sends (a function of its features), how messages are aggregated (sum, mean, max, attention-weighted), and how the node updates its representation. These three operations — message, aggregate, update — are the GNN's interface protocol.

BERT's interface specification forces you to make this explicit. The protocol field on the Message Passing → Attention interface must state: "1. Node features projected to message space → 2. Messages sent along edges → 3. Attention scores computed for each edge → 4. Messages weighted by attention → 5. Aggregated messages update node state." That protocol IS the GNN architecture. Different GNN variants (GCN, GAT, GraphSAGE) differ precisely in their interface protocols, not in their subsystem structure.

This means the typed model can express the *family* of GNN architectures as variants of the same subsystem structure with different interface protocols. That's a structural insight the GNN literature doesn't make explicit.

### Key Interfaces to Specify

- Node Embedding → Message Passing: initial node representations + graph structure (adjacency)
- Message Passing → Attention: raw neighbor messages for weighting
- Attention → Message Passing: weighted/filtered messages (feedback loop — attention constrains message flow)
- Message Passing → Readout: final node embeddings after K layers
- Readout → Task Head: graph-level representation vector

### Generation Notes

The feedback loop between Message Passing and Attention is architecturally interesting — it's a governance feedback loop within the processing stack. This parallels the reward model's feedback to the policy network in the RL agent. Flag this structural parallel in the description.

---

## 6. Neuro-Symbolic System (LNN/DeepProbLog/NeSy)

**Essay reference**: §04 — "Only as good as the rules it's given. The human decides where the boundary between learning and assertion falls."

**Why this one**: This is the instrument that most directly confronts the thesis of the Instruments essay. A neuro-symbolic system combines pattern recognition (neural) with logical constraint enforcement (symbolic). It is the most ambitious bridge between learning and specification. The decomposition reveals where the boundary falls — and that boundary is an *authored commitment*, not a learned parameter.

For AI researchers working on neurosymbolic AI, safe AI, or interpretable AI, this model makes the argument viscerally: the symbolic rules you embed ARE specifications in the CESM sense. The neural components learn. The symbolic components assert. BERT's typed structure captures this duality natively.

### Likely Decomposition

**Environment**: Hybrid computation infrastructure — differentiable logic engine, neural network backend, knowledge base, training data.

**Sources**: Training data (raw observations), formal knowledge base (logical rules, ontology), task specification (what to predict/classify)

**Sinks**: Predictions (neural output constrained by logic), explanation traces (logical derivation paths), rule violations (constraint failures)

**Subsystems** (5):

| Subsystem | Role | Archetype | Why this classification |
|-----------|------|-----------|------------------------|
| Neural Perception Module | Pattern recognition from raw data — the learning side | **Agent** | Decides what patterns exist — the inductive subsystem |
| Symbolic Rule Engine | Enforces logical constraints on neural outputs | **Governance** | Validates and constrains — the deductive subsystem. This IS specification. |
| Neural-Symbolic Interface | Translates between continuous neural representations and discrete logical symbols | Economy | Allocates between two representation regimes — the bridge |
| Knowledge Base Manager | Stores, retrieves, and updates formal rules and ontological commitments | Economy | Allocates knowledge — manages the specification repository |
| Explanation Generator | Produces human-readable derivation traces showing why a conclusion was reached | Agent | Decides how to present reasoning — the interpretability subsystem |

### What the Typed Lens Reveals

The Symbolic Rule Engine's archetype is **Governance** — and this is where the Instruments essay's entire argument becomes structural. The essay says the gap is that none of the instruments can author specifications. The neuro-symbolic system is the only one that *embeds* specifications (as logical rules). But it embeds them as a governance subsystem — as constraints on the neural agent, not as the neural agent's own capacity.

This means even the most specification-aware instrument still treats specification as a *constraint from outside*, not as an *authored commitment from within*. The rules come from a human. The Rule Engine enforces them. The neural module learns within them. The boundary between "what the system learns" and "what the human asserts" is exactly the boundary between Agent and Governance subsystems.

That boundary is the thesis of the Instruments essay, made structural.

### Key Interfaces to Specify

- Neural Module → NeSy Interface: continuous activation vectors (soft predictions)
- NeSy Interface → Rule Engine: discretized logical propositions for constraint checking
- Rule Engine → NeSy Interface: constraint satisfaction results (pass/violate/partial)
- NeSy Interface → Neural Module: gradient-compatible constraint signals (differentiable logic)
- Knowledge Base → Rule Engine: active rule set, ontological commitments
- Neural Module + Rule Engine → Explanation Generator: activation traces + derivation paths

### Generation Notes

The Neural-Symbolic Interface is the most architecturally critical subsystem — it mediates between continuous and discrete representations. Its interface protocols must be specified with special care, because this is where most NeSy systems fail in practice. The typed model makes this explicit: the protocol must specify how continuous activations are discretized (thresholding? probabilistic? fuzzy logic?) and how logical constraints flow back as differentiable gradients. These are design decisions the NeSy literature often leaves implicit.

---

## 7. Causal Discovery Algorithm (PC / FCI / NOTEARS)

**Essay reference**: §06 — "Proposes candidates. Cannot commit to them."

**Why this one**: Causal discovery is the instrument that comes closest to doing what BERT does — proposing structure. It infers candidate causal graphs from observational data using conditional independence tests. It's the computational instrument most sympathetic to the specification project. And the decomposition reveals exactly where it stops short: it proposes hypotheses, it doesn't commit to them. The human still has to sign the drawing.

For AI researchers working on causal ML, this model reframes their tool as an information processing system and reveals that the critical subsystem — the one that converts a candidate graph into a committed specification — is *absent from the architecture*. That absence is the essay's thesis.

### Likely Decomposition

**Environment**: Observational data infrastructure — datasets, statistical computing environment, domain expert interface.

**Sources**: Observational data (multivariate samples), prior knowledge (forbidden/required edges, tiered ordering), significance thresholds (alpha levels)

**Sinks**: Candidate causal graph (DAG or CPDAG), edge confidence scores, independence test results, undetermined edges (ambiguities)

**Subsystems** (5):

| Subsystem | Role | Archetype | Why this classification |
|-----------|------|-----------|------------------------|
| Independence Test Engine | Computes conditional independence between variable pairs (partial correlation, kernel-based, etc.) | **Agent** | The active computation — decides whether two variables are independent given a conditioning set |
| Constraint Propagation Module | Applies orientation rules (colliders, Meek rules) to direct edges based on independence results | **Governance** | Validates and constrains — enforces logical consistency of the emerging graph structure |
| Search Strategy | Controls the order in which variable pairs and conditioning sets are tested (skeleton-first, score-based, hybrid) | Economy | Allocates computational budget — decides where to look next |
| Prior Knowledge Integrator | Incorporates domain constraints (forbidden edges, required edges, tiered variable ordering) | **Governance** | Validates against external specification — the one place where authored knowledge enters |
| Graph Assembler | Combines test results, orientation rules, and priors into a candidate graph (CPDAG/PAG) | **Agent** | Decides the output — assembles the structural hypothesis |

### What the Typed Lens Reveals

Two governance subsystems — and this is what makes causal discovery the most structurally interesting instrument. The Constraint Propagation Module enforces *internal* consistency (logical rules about how a DAG must be structured). The Prior Knowledge Integrator enforces *external* constraints (what the domain expert asserts). These are different kinds of governance: one validates against mathematical rules, the other against authored commitments.

But here's the key: the Prior Knowledge Integrator is the only subsystem in the entire architecture that accepts *specification* as input. The forbidden/required edges, the tiered ordering — these are authored claims about the system. They enter through a governance interface. And they are *optional*. Most causal discovery runs proceed without them.

This means causal discovery has the *interface* for specification but doesn't *require* it. The architecture supports authored commitment but doesn't demand it. That's a more nuanced version of the essay's claim: it's not that these instruments *can't* use specification. It's that they're architecturally designed to work *without* it. The default path skips the Prior Knowledge Integrator entirely.

The output — a CPDAG (completed partially directed acyclic graph) — is explicitly a *set of hypotheses*, not a commitment. Multiple DAGs are consistent with the same CPDAG. The human must choose among them. The architecture produces candidates; the modeler authors the specification. That gap is the essay's §06 ceiling, made structural.

### Key Interfaces to Specify

- Search Strategy → Independence Test Engine: next variable pair + conditioning set to test
- Independence Test Engine → Constraint Propagation: test result (independent/dependent, p-value)
- Constraint Propagation → Graph Assembler: oriented edges, undetermined edges, conflicts
- Prior Knowledge Integrator → Constraint Propagation: hard constraints (forbidden/required edges)
- Prior Knowledge Integrator → Search Strategy: tiered ordering (test higher tiers first)
- Graph Assembler → Output: candidate CPDAG with confidence annotations

### Generation Notes

The dual governance structure (internal consistency + external specification) is unique in the suite. No other model has two governance subsystems with different validation logics. Emphasize this in the description — it's the structural signature of an instrument that *could* accept specification but doesn't insist on it.

The output type is also unique: not a prediction, not a decision, but a *hypothesis space*. The Graph Assembler's output is explicitly ambiguous — multiple causal structures are consistent with it. Model this in the sink description. This is the formal representation of "proposes candidates, cannot commit."

---

## 8. Retrieval-Augmented Generation (RAG) System

**Essay reference**: §07 Composite — spans §03 (Generation) and §01 (Recognition). Not a single instrument but a *composed system* of instruments. Maps to the essay's §07 ("The Question None of Them Ask") because RAG is what happens when instruments get wired together without anyone specifying the composed system.

**Why this one**: RAG is the most widely deployed AI architecture in production (2024-2026). Every enterprise AI deployment is some form of RAG. More importantly for your argument: **Facets IS a RAG system** — 554 vector passages + 734 KG triples organized by 8-tuple classification. Modeling RAG with BERT demonstrates that your deployed system is a typed instance of the generic architecture. AI researchers will see the direct applicability.

The composite case is structurally distinct from all other models in the suite: each subsystem is itself a complex system that could have its own BERT model. The LLM generator IS the system modeled in `llm.json`. The retriever could be a ViT if you're doing multimodal RAG. RAG is a *system of systems* — and the typed interfaces between them are the load-bearing specifications that most deployments leave implicit.

### Likely Decomposition

**Environment**: Knowledge management infrastructure — document corpus, user query interface, vector database, generation API.

**Sources**: User query (natural language question), document corpus (knowledge base), retrieval index (pre-computed embeddings)

**Sinks**: Generated response (grounded answer), retrieved context (passages surfaced), query logs (usage telemetry)

**Subsystems** (5):

| Subsystem | Role | Archetype | Why this classification |
|-----------|------|-----------|------------------------|
| Query Encoder | Converts natural language query to embedding vector | Economy | Translates between spaces — allocates meaning to a fixed-dimensional representation |
| Vector Retriever | Searches embedding space for relevant passages | **Agent** | Decides what's relevant — the active search/selection subsystem |
| Reranker | Scores and filters retrieved passages for relevance | **Governance** | Validates retriever output — doesn't retrieve, it judges what was retrieved |
| Context Assembler | Constructs the prompt from query + retrieved passages | Economy | Allocates context window budget — decides what fits and in what order |
| Generator (LLM) | Produces grounded response conditioned on assembled context | **Agent** | The deciding/generating subsystem — produces the output |

### What the Typed Lens Reveals

RAG systems fail at the *seams* — the interfaces between subsystems. The retriever returns passages the generator can't use. The reranker optimizes for relevance but the context assembler truncates the best passages. The generator hallucinates despite having correct context because the context was assembled in the wrong order.

Every one of these failure modes is an **interface specification failure**. The typed model makes this explicit: the protocol between Retriever → Reranker specifies what metadata accompanies each passage (score, source, chunk boundaries). The protocol between Context Assembler → Generator specifies ordering, truncation strategy, and citation markers. When these protocols are implicit (as they are in most RAG deployments), failures are mysterious. When they're typed, failures are diagnosable.

The Facets connection: Facets adds an additional subsystem — a **Knowledge Graph Router** that classifies queries by 8-tuple dimension before retrieval. This is a governance subsystem that the generic RAG architecture lacks. Modeling both the generic RAG and Facets-as-RAG shows exactly what typed structure adds.

### Key Interfaces to Specify

- Query Encoder → Vector Retriever: embedding vector + search parameters (top-k, similarity threshold)
- Vector Retriever → Reranker: candidate passages with scores, source metadata, chunk boundaries
- Reranker → Context Assembler: filtered/reordered passages with relevance scores
- Context Assembler → Generator: assembled prompt with citation markers, token budget remaining
- Generator → Output: response with grounding annotations (which passages supported which claims)

### Generation Notes

Consider creating two variants: `rag.json` (generic architecture) and `facets-rag.json` (Facets-specific, with KG Router as additional governance subsystem and 8-tuple classification on the retrieval path). The diff between them IS the argument for typed structure.

---

## The Suite as an Argument

Once all 9 models exist:

**The BERT examples directory contains**:
- bitcoin.json, ethereum.json, cosmos-hub.json, solana.json (economic/governance systems)
- llm.json, vision-transformer.json, state-space-model.json, rl-agent.json, diffusion-model.json, gnn.json, neuro-symbolic.json, causal-discovery.json, rag.json (computational instruments)

**13 models. Same 40 primitives.**

**The argument writes itself**: the same typed structure — Source, Sink, System, Boundary, Interface, Flow, the three archetypes — describes both the systems being modeled AND the instruments doing the modeling. Bitcoin and the GNN that analyzes Bitcoin's transaction graph use the same specification language. Ethereum's governance mechanism and the RL agent trained to optimize gas fees share the same typed structure.

This is not a metaphor. It's a formal claim: information processing systems are information processing systems regardless of substrate. The typed structure is substrate-independent. And that's exactly what the Instruments essay argues in prose — the suite proves it in structure.

### For AI Researchers Specifically

The suite makes five claims they care about:

1. **Pure perception has no governance**. The Vision Transformer model has agents and economy but zero governance subsystems. This structural absence explains adversarial vulnerability and is the architectural reason recognition alone is insufficient for safe deployment.

2. **Alignment is a governance specification problem, not just a training problem**. The RL agent model shows the reward model is a governance subsystem. If it's poorly specified, you have a governance failure. This reframes alignment debates from "how do we train better reward models" to "how do we specify governance structures for decision-making systems."

3. **Composability requires typed interfaces**. The RAG model shows that composite AI systems fail at the seams. The interfaces between retriever, reranker, assembler, and generator are the load-bearing specifications. Most production RAG systems leave these implicit. Facets doesn't.

4. **The learning/specification boundary is an architectural choice, not a philosophical debate**. The neuro-symbolic model shows this boundary as the interface between Agent and Governance subsystems. It's a design decision with typed consequences, not an abstract question about the nature of intelligence.

5. **Causal discovery has the interface for specification but doesn't require it**. The causal discovery model has dual governance (internal consistency + external priors) but the external specification interface is optional. The architecture is designed to work without authored commitment. That's the gap — not a missing capability, but a missing *default*.

---

## Execution Plan

### Order of Operations

Build in essay order — each model maps to its section, and complexity builds naturally:

1. **Vision Transformer** (§01) — simplest architecture, warm-up. Establishes the "no governance" baseline.
2. **State Space Model** (§02) — introduces selective gating as governance. First non-transformer.
3. **Diffusion Model** (§03) — introduces temporal boundary dynamics. Architecturally alien.
4. **RL Agent** (§05) — introduces reward model as governance. The alignment connection.
5. **GNN** (§04) — introduces the governance feedback loop (attention constrains message passing). Meta.
6. **Neuro-Symbolic** (§04) — the learning/specification boundary as typed structure.
7. **Causal Discovery** (§06) — dual governance, optional specification interface. The closest to BERT.
8. **RAG System** (§07) — the capstone. Composition of instruments, typed interfaces as load-bearing specs.

### For Each Model

1. **Research** (30 min): Read 1-2 canonical papers or architecture docs to ground the decomposition. You know these systems — this is about precision, not learning.
2. **Decompose** (15 min): Sketch subsystems, archetypes, key interfaces on paper or in a scratch note. Validate against the Instruments essay section.
3. **Generate** (20 min): `Use bert-json-creation to create [system] model` — provide the decomposition sketch and llm.json as structural reference.
4. **Validate** (15 min): Load in BERT, verify subsystem relationships render correctly, check interface protocols are complete.
5. **Annotate** (10 min): Add the "What the Typed Lens Reveals" insight to the model description. This is what makes each model argumentative, not just descriptive.

**Estimated total**: ~12-14 hours across focused sessions for all 8 new models.

### Quality Gates

Each model must pass:
- [ ] All subsystems have archetype classification with written justification
- [ ] All interfaces have complete protocol specifications (numbered steps)
- [ ] At least one non-obvious structural insight in the system description
- [ ] Environment sources/sinks reflect the actual deployment context, not an abstraction
- [ ] Time constants differ meaningfully across subsystems (not all "Second")

### Optional Extension: Facets-RAG Variant

After the generic RAG model, create `facets-rag.json` as a variant with the Knowledge Graph Router governance subsystem. The diff between `rag.json` and `facets-rag.json` IS the demo for what typed structure adds to a production system. This is the version you show people.

---

## Connection to Instruments Essay

Each model can be cross-referenced to its essay section. Consider adding a `references` or `essay_section` field to the JSON metadata (or just the description) so that when someone browses the BERT examples, they can trace from the formal model to the prose argument and back.

The essay argues in language. The suite argues in structure. Together they make the case that the info processing lens isn't a metaphor — it's a formal framework that applies to its own instruments.

---

## Next: Paired Implementations

Once the JSON suite exists, the next move is pairing each specification with a minimal running implementation — real code you can point to and trace back to the typed model.

**The goal**: for each JSON, a small but real codebase (or pointer to an existing one) where you can say "subsystem 3 maps to this class, interface 2 maps to this function call, the governance boundary lives *here* in the code." Specification and implementation, side by side, with explicit traceability.

**Low-hanging fruit** (existing implementations to annotate):

| JSON | Implementation | Work required |
|------|---------------|---------------|
| `rag.json` | Facets (deployed on Railway) | Annotation only — map JSON subsystems to Facets modules. Already built. |
| `llm.json` | Any open-weight model (LLaMA, Mistral) | Annotation — map JSON subsystems to model architecture code. No building. |

**Medium effort** (minimal implementations to write):

| JSON | Implementation | Scope |
|------|---------------|-------|
| `rl-agent.json` | Minimal PPO agent (Python, ~200 lines) | CartPole or similar. Annotate: policy network, value estimator, reward model as separate classes matching JSON subsystems. |
| `gnn.json` | Minimal GCN (PyTorch Geometric, ~100 lines) | Node classification on a small graph. Annotate message passing, attention, readout as separate modules. |
| `vision-transformer.json` | Minimal ViT (PyTorch, ~150 lines) | CIFAR-10 or similar. Patch embedding, attention stack, classification head as separate modules. |

**Higher effort** (but highest payoff):

| JSON | Implementation | Why it's worth it |
|------|---------------|-------------------|
| `causal-discovery.json` | PC algorithm with prior knowledge interface (~300 lines) | The dual governance structure (constraint propagation + prior knowledge) is the most novel architectural claim. A running implementation where you can toggle the prior knowledge integrator on/off and watch the output change is the most compelling demo in the suite. |
| `neuro-symbolic.json` | DeepProbLog or LNN minimal example | The learning/specification boundary made executable. Toggle the rule engine on/off, watch the neural module's outputs change. |

**The punchline artifact**: a README or interactive page in the BERT examples directory where each row is a system with three links: the essay section (prose argument), the JSON (formal specification), and the implementation (running proof). Three representations of the same claim — language, structure, code.

**Not recommended to pair** (diminishing returns):

| JSON | Why skip |
|------|----------|
| `diffusion-model.json` | Stable Diffusion implementations are massive. A minimal DDPM trainer exists (~200 lines) but the architectural insight (noise scheduler as governance) is harder to make visible in code than in the JSON. |
| `state-space-model.json` | Mamba's implementation is tightly optimized CUDA code. A pedagogical version exists but loses the architectural clarity. The JSON is more instructive than the code here. |

Build the JSONs first. Annotate Facets and LLaMA as the two free pairings. Then decide which minimal implementations earn their build time based on what lands with people when you show them the specs.
