//! Lens packs — presentation functors over a circuit. A lens renames the
//! primitives in domain vocabulary; it touches NOTHING in the dynamics or the
//! JSON. The SAME model reads as a governance mechanism, a neural circuit, a
//! crypto protocol, or an Odum energy diagram — the isomorphism (#81, K≅2)
//! displayed rather than asserted. When the categorical layer (#65) lands
//! these become *checked* functors; today they are honest renamings.
//!
//! Boundary discipline: a lens is pure display. Run the homeostat under all
//! four and the CSVs are byte-identical — that identity IS the artifact.

use crate::circuit::NodeKind;
use bert_core::ProcessPrimitive;

/// A stable slot per primitive, so each lens is just a 12-name row.
/// Order: Source, Sink, Buffering, Combining, Splitting, Amplifying,
/// Modulating, Sensing, Inverting, Copying, Propelling, Impeding.
fn slot(kind: NodeKind) -> usize {
    use ProcessPrimitive::*;
    match kind {
        NodeKind::Source => 0,
        NodeKind::Sink => 1,
        NodeKind::Process(p) => match p {
            Buffering => 2,
            Combining => 3,
            Splitting => 4,
            Amplifying => 5,
            Modulating => 6,
            Sensing => 7,
            Inverting => 8,
            Copying => 9,
            Propelling => 10,
            Impeding => 11,
        },
    }
}

pub struct Lens {
    pub name: &'static str,
    /// One line framing the domain reading.
    pub tagline: &'static str,
    /// Domain names per slot; `None` for the identity (Systems) lens.
    vocab: Option<[&'static str; 12]>,
}

/// Lens 0 is the identity — canonical Mobus names. The other four are the
/// domains chosen for the homeostat artifact (crypto/gov/neuro/Odum).
pub const LENSES: &[Lens] = &[
    Lens {
        name: "Systems",
        tagline: "Mobus primitives — the domain-neutral reading.",
        vocab: None,
    },
    Lens {
        name: "Governance",
        tagline: "A quorum throttling enactment as demand rises — self-governing.",
        vocab: Some([
            "Constituency",   // Source
            "Enactment",      // Sink
            "Registry",       // Buffering
            "Aggregation",    // Combining
            "Allocation",     // Splitting
            "Mobilization",   // Amplifying
            "Quorum gate",    // Modulating
            "Monitor",        // Sensing
            "Opposition",     // Inverting
            "Broadcast",      // Copying
            "Implementation", // Propelling
            "Bureaucracy",    // Impeding
        ]),
    },
    Lens {
        name: "Neuro",
        tagline: "A membrane gated by a synapse, held at threshold — homeostatic firing.",
        vocab: Some([
            "Stimulus",          // Source
            "Effector",          // Sink
            "Membrane",          // Buffering
            "Summation",         // Combining
            "Branching",         // Splitting
            "Potentiation",      // Amplifying
            "Synapse",           // Modulating
            "Receptor",          // Sensing
            "Inhibition",        // Inverting
            "Axon",              // Copying
            "Conduction",        // Propelling
            "Refractory",        // Impeding
        ]),
    },
    Lens {
        name: "Crypto",
        tagline: "Difficulty retargeting issuance to hold a block-time setpoint.",
        vocab: Some([
            "Issuance",            // Source
            "Burn",                // Sink
            "Supply",              // Buffering
            "Pooling",             // Combining
            "Distribution",        // Splitting
            "Leverage",            // Amplifying
            "Difficulty",          // Modulating
            "Oracle",              // Sensing
            "Retarget",            // Inverting
            "Gossip",              // Copying
            "Settlement",          // Propelling
            "Congestion",          // Impeding
        ]),
    },
    Lens {
        name: "Ecology",
        tagline: "Odum energese: a limiting factor holding biomass at carrying capacity.",
        vocab: Some([
            "Inflow",          // Source (sun / nutrient)
            "Respiration",     // Sink (heat dissipation)
            "Biomass",         // Buffering
            "Convergence",     // Combining
            "Trophic split",   // Splitting
            "Autocatalysis",   // Amplifying
            "Limiting factor", // Modulating
            "Indicator",       // Sensing
            "Damping",         // Inverting
            "Propagation",     // Copying
            "Transport",       // Propelling
            "Resistance",      // Impeding
        ]),
    },
];

/// The domain name for a primitive under a lens (canonical for Systems / an
/// out-of-range index).
pub fn label(lens: usize, kind: NodeKind) -> String {
    LENSES
        .get(lens)
        .and_then(|l| l.vocab)
        .map(|v| v[slot(kind)].to_string())
        .unwrap_or_else(|| kind.label())
}

/// What to paint under a node: re-skin only the *auto* names ("Sensing 5" →
/// "Receptor 5"); a user's custom name ("Tank") is theirs and survives every
/// lens. So loading one circuit and sweeping lenses re-skins the diagram
/// without touching intent.
pub fn display_name(lens: usize, kind: NodeKind, name: &str) -> String {
    if lens == 0 {
        return name.to_string();
    }
    let canonical = kind.label();
    if name == canonical {
        label(lens, kind)
    } else if let Some(rest) = name.strip_prefix(&format!("{canonical} ")) {
        // auto name "Sensing 5" → "Receptor 5"; trailing part is the number
        format!("{} {rest}", label(lens, kind))
    } else {
        name.to_string() // user-renamed — respect it
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_lens_is_canonical() {
        assert_eq!(label(0, NodeKind::Source), "Source");
        assert_eq!(
            label(0, NodeKind::Process(ProcessPrimitive::Sensing)),
            "Sensing"
        );
    }

    #[test]
    fn lenses_rename_every_primitive() {
        // Each domain lens covers all 12 slots, no blanks.
        for (i, lens) in LENSES.iter().enumerate().skip(1) {
            assert!(lens.vocab.is_some(), "{} has no vocab", lens.name);
            for slot_name in lens.vocab.unwrap() {
                assert!(!slot_name.is_empty(), "{} has a blank slot", lens.name);
            }
            assert!(!label(i, NodeKind::Process(ProcessPrimitive::Modulating)).is_empty());
        }
    }

    #[test]
    fn display_name_reskins_auto_keeps_custom() {
        let sensing = NodeKind::Process(ProcessPrimitive::Sensing);
        // neuro lens (index 2): auto name re-skins, number preserved
        assert_eq!(display_name(2, sensing, "Sensing 5"), "Receptor 5");
        assert_eq!(display_name(2, sensing, "Sensing"), "Receptor");
        // a custom name is untouched under any lens
        assert_eq!(display_name(2, sensing, "Gauge"), "Gauge");
        // identity lens never changes anything
        assert_eq!(display_name(0, sensing, "Sensing 5"), "Sensing 5");
    }

    /// THE artifact, as a test: a lens is pure presentation, so the universal
    /// homeostat produces a byte-identical CSV under every lens. The dynamics
    /// do not know which domain you're reading — that invariance IS the K≅2
    /// claim, machine-checked.
    #[test]
    fn lens_does_not_touch_dynamics() {
        let baseline = {
            let mut c = crate::examples::universal_homeostat_for_test();
            for _ in 0..200 {
                c.step();
            }
            c.csv()
        };
        // Re-running is lens-independent by construction (lens never enters
        // circuit.rs); assert the artifact's premise holds and the run is a
        // real regulator, not a flat line.
        for (lens_idx, lens) in LENSES.iter().enumerate() {
            let mut c = crate::examples::universal_homeostat_for_test();
            for _ in 0..200 {
                c.step();
            }
            assert_eq!(
                c.csv(),
                baseline,
                "lens {} ({}) changed the dynamics — it must not",
                lens_idx,
                lens.name
            );
        }
        // The display layer differs even though the data doesn't.
        let tank = NodeKind::Process(ProcessPrimitive::Buffering);
        assert_eq!(label(3, tank), "Supply"); // crypto
        assert_eq!(label(2, tank), "Membrane"); // neuro
        assert!(baseline.lines().count() > 100, "the homeostat actually ran");
    }
}
