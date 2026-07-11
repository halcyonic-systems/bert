package LanguageTransformer {

    // ===== substance categories -> item defs =====
    item def Message;

    part def LanguageTransformer {
        doc /* Generation instrument (§03): produces next-token probability distributions conditioned on ... */
        // --- ports (BERT boundary interfaces) ---
        in  port tokenizationInterface : Message;   // I0.50
        out port generationInterface : Message;   // I0.51
        // --- subsystems (BERT internal components) ---
        part encoding : Encoding;   // [BERT-only] archetype=Economy
        part transformerCore : TransformerCore;   // [BERT-only] archetype=Economy
        part decoding : Decoding;   // [BERT-only] archetype=Economy
        // interface processors fold into ports above: C0.50, C0.51
        // --- internal flows ---
        flow of Message from encoding to transformerCore;   // F0.2 Token Embeddings [usability=Resource]
        flow of Message from transformerCore to decoding;   // F0.3 Contextualized Representations [usability=Resource]
    }

    // ===== component definitions =====
    part def Encoding {
        doc /* Tokenization and embedding: converts raw text to dense vector sequences in a lea... */
        // [BERT-only] adaptable=True evolveable=False
    }
    part def TransformerCore {
        doc /* Stacked self-attention and feed-forward layers: builds contextualized representa... */
        // [BERT-only] adaptable=True evolveable=False
    }
    part def Decoding {
        doc /* Logit projection and sampling: converts contextualized representations to probab... */
        // [BERT-only] adaptable=True evolveable=False
    }

    // ===== environment context =====
    part environment {
        part promptInput;   // Src-1.0 Source
        part generatedText;   // Snk-1.0 Sink
        part languageTransformer : LanguageTransformer;
        flow of Message from promptInput to languageTransformer.tokenizationInterface;   // F-1.0 Raw Text [usability=Resource]
        flow of Message from languageTransformer.generationInterface to generatedText;   // F-1.1 Generated Language [usability=Product]
    }

    // ===== [BERT-only] semantics with NO SysML v2 home (dropped/annotated) =====
    // structural fields present on every system but unrepresentable:
    //   member_autonomy, time_constant, boundary.porosity, boundary.perceptive_fuzziness
    //   Interaction type=Force (influence-without-transfer): 0 in this model
    //   usability (Resource/Product/Waste/Disruption) on every flow -> see inline comments
}
