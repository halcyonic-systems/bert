//! Mode transitions on the kernel ladder: moving a [`WorldModel`] from the rung
//! its stamp names to another rung, either shedding out-of-mode structure into a
//! recoverable [`LossWitness`] (downgrade) or checking the target rung's
//! precondition and re-stamping (upgrade).
//!
//! The [`Mode`] poset (lib.rs, `Mode`) is a **meet-semilattice (tree-shaped); no
//! joins by design** — `Core` is the meet of every pair, `Structural` and `Full`
//! are parallel leaves with no join (the rungs are lenses, not a cumulative
//! tower; `Cybernetic` is deliberately absent). Transitions are defined on the
//! three Hasse edges — `Core`–`Structural`, `Core`–`Operational`,
//! `Operational`–`Full` — and every cross-move composes through the meet: e.g.
//! `Structural → Operational` descends `Structural → Core` then climbs
//! `Core → Operational`. The commuting law comes free, mirroring the Lean
//! `Kernel.toMobus_toBunge` factorisation through `toKlir`.
//!
//! Downgrade **strips, it does not restamp**: each out-of-mode field is reset to
//! its minimal witness (empty string / `None` / empty vec / zero / `Default`),
//! and the forgotten value is enumerated in the witness so that
//! `rebuild(downgrade(m).model, downgrade(m).loss)` reproduces `m` field-for-field.
//! Upgrade never edits: it checks the target edge's named hypothesis
//! (`Kernel.HasBond`, `Kernel.Irreflexive`) exactly as [`validate_mode`] does and
//! refuses with that name, leaving the required edits to the author.
//!
//! ## Pinned Lean/Rust correspondence (ratified with the §A5 memo)
//!
//! | Concern | Lean (`ViewGeneration.lean`) | Rust (this crate) |
//! |---|---|---|
//! | Mode poset | views generated off one `Kernel` | meet-semilattice, tree-shaped, no joins (A1) |
//! | Dependency | `dep : Set` | `Kernel.dep` documented multiset (parallel edges kept) |
//! | Structural precondition | `HasBond` needs semantic `Bonded` (`ActsOn`) | `check_bond` is syntactic — any interaction between distinct systems counts |
//! | Kernel invariance | `toMobus_toKlir = toKlir` | `WorldModel::kernel` byte-identical across every transition |
//! | Minimal witness | `PUnit` / `∅` | empty string / `None` / empty vec / zero / `Default` |
//! | Milieu `M` | spec-level (§3.8) | no `WorldModel` representation as of a8969b7 (A3); the `Structural → Core` witness carries environment membership only, which the kernel already preserves, so it is empty in the current schema |

use crate::rust_decimal::Decimal;
use crate::validate::{validate_mode, Severity};
use crate::{Id, InteractionType, InteractionUsability, Mode, SubstanceType, WorldModel};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The minimal (zero-information) flow role. `InteractionUsability` has no
/// `Default`, so the productive export is pinned as the witness baseline for the
/// `Operational → Core` catalogue's external-flow designation.
const MIN_USABILITY: InteractionUsability = InteractionUsability::Product;

/// Why an upgrade may not proceed, in [`OperationalError`](crate::operational::OperationalError)'s
/// shape: the seam location, the reason (citing the Lean hypothesis), and a hint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionError {
    pub location: String,
    pub reason: String,
    pub hint: Option<String>,
}

/// One forgotten value: the edge that shed it, its owner entity, the slot name,
/// and the value as JSON so any kernel field type round-trips through the witness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LossEntry {
    pub section: String,
    pub owner: Id,
    pub slot: String,
    pub value: serde_json::Value,
}

impl LossEntry {
    fn new(section: &str, owner: Id, slot: &str, value: &impl Serialize) -> Self {
        Self {
            section: section.to_string(),
            owner,
            slot: slot.to_string(),
            value: serde_json::to_value(value).expect("kernel field serializes"),
        }
    }
}

/// The recoverable record of a downgrade: the model's original stamp plus every
/// value shed on the way down, in application order (outermost edge first). It is
/// the other half of the split — `rebuild` re-injects it to reproduce the input.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LossWitness {
    /// The `mode` field the input carried, restored verbatim (so an absent stamp
    /// stays absent) — never the resolved [`Mode`].
    pub original_mode: Option<Mode>,
    pub to: Mode,
    pub entries: Vec<LossEntry>,
}

/// The outcome of [`validate_transition`].
#[derive(Clone)]
pub enum Transition {
    /// `from == to`: nothing to do, the input stands.
    Identity,
    /// A descent (or a cross-move that shed data): the re-stamped model and the
    /// witness that reproduces the input.
    Downgrade {
        model: WorldModel,
        loss: LossWitness,
    },
    /// An ascent (or a cross-move that shed nothing): the re-stamped model, with
    /// minimal slots already satisfied by the total schema.
    Upgrade { model: WorldModel },
}

impl std::fmt::Debug for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transition::Identity => write!(f, "Identity"),
            Transition::Downgrade { loss, .. } => f
                .debug_struct("Downgrade")
                .field("loss", loss)
                .finish_non_exhaustive(),
            Transition::Upgrade { .. } => write!(f, "Upgrade"),
        }
    }
}

/// Transition `model` from its stamped rung ([`WorldModel::mode`], never a
/// parameter — the stamp is the authority) to `to`.
///
/// A descent shears every out-of-mode field into the [`LossWitness`]; an ascent
/// verifies the target edge's hypothesis and refuses with its name on failure. A
/// cross-move composes both through the meet ([`Mode::Core`]).
pub fn validate_transition(
    model: &WorldModel,
    to: Mode,
) -> Result<Transition, Vec<TransitionError>> {
    let from = model.mode();
    if from == to {
        return Ok(Transition::Identity);
    }

    let meet = meet(from, to);
    let mut work = model.clone();
    let mut entries: Vec<LossEntry> = Vec::new();

    let mut rung = from;
    while rung != meet {
        let up = parent(rung).expect("only Core lacks a parent, and Core is a meet");
        apply_descent(rung, &mut work, &mut entries);
        rung = up;
    }

    let mut errors: Vec<TransitionError> = Vec::new();
    for entering in ascent_chain(meet, to) {
        errors.extend(precondition_errors(&work, entering));
    }
    if !errors.is_empty() {
        return Err(errors);
    }

    work.mode = Some(to);

    let descended = from != meet;
    let ascended = meet != to;
    let outcome = if descended && (!ascended || !entries.is_empty()) {
        Transition::Downgrade {
            model: work,
            loss: LossWitness {
                original_mode: model.mode,
                to,
                entries,
            },
        }
    } else {
        Transition::Upgrade { model: work }
    };
    Ok(outcome)
}

/// Re-inject a witness into a downgraded model, restoring the original stamp and
/// every shed value. The inverse of the descent that produced `loss`.
pub fn rebuild(mut model: WorldModel, loss: &LossWitness) -> WorldModel {
    for entry in &loss.entries {
        restore(&mut model, entry);
    }
    model.mode = loss.original_mode;
    model
}

/// The Core-ward parent of a rung, or `None` for the meet itself.
fn parent(mode: Mode) -> Option<Mode> {
    match mode {
        Mode::Full => Some(Mode::Operational),
        Mode::Operational | Mode::Structural => Some(Mode::Core),
        Mode::Core => None,
    }
}

/// The deepest rung on both Core-ward chains — the meet at which a cross-move pivots.
fn meet(a: Mode, b: Mode) -> Mode {
    let mut chain_a: HashSet<Mode> = HashSet::new();
    let mut walk = Some(a);
    while let Some(m) = walk {
        chain_a.insert(m);
        walk = parent(m);
    }
    let mut walk = Some(b);
    while let Some(m) = walk {
        if chain_a.contains(&m) {
            return m;
        }
        walk = parent(m);
    }
    Mode::Core
}

/// The rungs to *enter*, meet-ward first, climbing from `meet` up to `to`.
fn ascent_chain(meet: Mode, to: Mode) -> Vec<Mode> {
    let mut chain = Vec::new();
    let mut walk = Some(to);
    while let Some(m) = walk {
        if m == meet {
            break;
        }
        chain.push(m);
        walk = parent(m);
    }
    chain.reverse();
    chain
}

/// Shear the fields the rung being *left* owns, moving each non-minimal value
/// into `entries`. `rung` is the child mode; the descent lands on its parent.
fn apply_descent(rung: Mode, model: &mut WorldModel, entries: &mut Vec<LossEntry>) {
    match rung {
        Mode::Full => strip_dynamical(model, entries),
        Mode::Operational => strip_operational(model, entries),
        Mode::Structural => strip_structural(model, entries),
        Mode::Core => {}
    }
}

/// `Full → Operational`: the dynamical face — transformation (τ), history (η),
/// time constant (δ), exactly what `check_dynamical_face` reads.
fn strip_dynamical(model: &mut WorldModel, entries: &mut Vec<LossEntry>) {
    const SECTION: &str = "Full→Operational";
    for sys in &mut model.systems {
        let owner = sys.info.id.clone();
        if !sys.transformation.is_empty() {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "transformation",
                &sys.transformation,
            ));
            sys.transformation.clear();
        }
        if !sys.history.is_empty() {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "history",
                &sys.history,
            ));
            sys.history.clear();
        }
        if !sys.time_constant.is_empty() {
            entries.push(LossEntry::new(
                SECTION,
                owner,
                "time_constant",
                &sys.time_constant,
            ));
            sys.time_constant.clear();
        }
    }
}

/// `Operational → Core`: substance labels/units/amounts (κ), the flow direction
/// and external-flow designation, interfaces, and boundary porosity/fuzziness (π).
fn strip_operational(model: &mut WorldModel, entries: &mut Vec<LossEntry>) {
    const SECTION: &str = "Operational→Core";
    for ix in &mut model.interactions {
        let owner = ix.info.id.clone();
        if !ix.substance.sub_type.is_empty() {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "substance.sub_type",
                &ix.substance.sub_type,
            ));
            ix.substance.sub_type.clear();
        }
        if ix.substance.ty != SubstanceType::default() {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "substance.type",
                &ix.substance.ty,
            ));
            ix.substance.ty = SubstanceType::default();
        }
        if !ix.unit.is_empty() {
            entries.push(LossEntry::new(SECTION, owner.clone(), "unit", &ix.unit));
            ix.unit.clear();
        }
        if ix.amount != Decimal::ZERO {
            entries.push(LossEntry::new(SECTION, owner.clone(), "amount", &ix.amount));
            ix.amount = Decimal::ZERO;
        }
        if ix.ty != InteractionType::default() {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "interaction.type",
                &ix.ty,
            ));
            ix.ty = InteractionType::default();
        }
        if ix.usability != MIN_USABILITY {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "usability",
                &ix.usability,
            ));
            ix.usability = MIN_USABILITY;
        }
        if ix.source_interface.is_some() {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "source_interface",
                &ix.source_interface,
            ));
            ix.source_interface = None;
        }
        if ix.sink_interface.is_some() {
            entries.push(LossEntry::new(
                SECTION,
                owner,
                "sink_interface",
                &ix.sink_interface,
            ));
            ix.sink_interface = None;
        }
    }
    for sys in &mut model.systems {
        let owner = sys.info.id.clone();
        if sys.boundary.porosity != 0.0 {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "boundary.porosity",
                &sys.boundary.porosity,
            ));
            sys.boundary.porosity = 0.0;
        }
        if sys.boundary.perceptive_fuzziness != 0.0 {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "boundary.perceptive_fuzziness",
                &sys.boundary.perceptive_fuzziness,
            ));
            sys.boundary.perceptive_fuzziness = 0.0;
        }
        if !sys.boundary.interfaces.is_empty() {
            entries.push(LossEntry::new(
                SECTION,
                owner.clone(),
                "boundary.interfaces",
                &sys.boundary.interfaces,
            ));
            sys.boundary.interfaces.clear();
        }
        if sys.boundary.parent_interface.is_some() {
            entries.push(LossEntry::new(
                SECTION,
                owner,
                "boundary.parent_interface",
                &sys.boundary.parent_interface,
            ));
            sys.boundary.parent_interface = None;
        }
    }
}

/// `Structural → Core`: Bunge's rung adds the `HasBond` obligation, not stored
/// data. The inside/outside membership rides in the entity Ids and array
/// placement that `kernel` already preserves, and milieu `M` is spec-level (A3),
/// so there is nothing to shear in the current schema — the witness is empty. A
/// future Bunge-only field would be sheared here.
fn strip_structural(_model: &mut WorldModel, _entries: &mut [LossEntry]) {}

/// The upgrade hypothesis for entering `target`, run through [`validate_mode`]'s
/// existing error checks (`Kernel.HasBond` for Structural, `Kernel.Irreflexive`
/// for Operational and Full), each refusal naming the hypothesis it cites.
fn precondition_errors(model: &WorldModel, target: Mode) -> Vec<TransitionError> {
    let hypothesis = match target {
        Mode::Core => return Vec::new(),
        Mode::Structural => "Kernel.HasBond",
        Mode::Operational | Mode::Full => "Kernel.Irreflexive",
    };
    validate_mode(model, target)
        .issues
        .into_iter()
        .filter(|issue| issue.severity == Severity::Error)
        .map(|issue| TransitionError {
            location: issue.location,
            reason: format!(
                "entering {target:?} requires {hypothesis}: {}",
                issue.message
            ),
            hint: issue.suggestion,
        })
        .collect()
}

fn find_system<'m>(model: &'m mut WorldModel, owner: &Id) -> Option<&'m mut crate::System> {
    model.systems.iter_mut().find(|s| &s.info.id == owner)
}

fn find_interaction<'m>(
    model: &'m mut WorldModel,
    owner: &Id,
) -> Option<&'m mut crate::Interaction> {
    model.interactions.iter_mut().find(|i| &i.info.id == owner)
}

fn decode<T: for<'de> Deserialize<'de>>(value: &serde_json::Value) -> T {
    serde_json::from_value(value.clone()).expect("witness value decodes to its slot type")
}

fn restore(model: &mut WorldModel, entry: &LossEntry) {
    match entry.slot.as_str() {
        "transformation" => {
            if let Some(s) = find_system(model, &entry.owner) {
                s.transformation = decode(&entry.value);
            }
        }
        "history" => {
            if let Some(s) = find_system(model, &entry.owner) {
                s.history = decode(&entry.value);
            }
        }
        "time_constant" => {
            if let Some(s) = find_system(model, &entry.owner) {
                s.time_constant = decode(&entry.value);
            }
        }
        "boundary.porosity" => {
            if let Some(s) = find_system(model, &entry.owner) {
                s.boundary.porosity = decode(&entry.value);
            }
        }
        "boundary.perceptive_fuzziness" => {
            if let Some(s) = find_system(model, &entry.owner) {
                s.boundary.perceptive_fuzziness = decode(&entry.value);
            }
        }
        "boundary.interfaces" => {
            if let Some(s) = find_system(model, &entry.owner) {
                s.boundary.interfaces = decode(&entry.value);
            }
        }
        "boundary.parent_interface" => {
            if let Some(s) = find_system(model, &entry.owner) {
                s.boundary.parent_interface = decode(&entry.value);
            }
        }
        "substance.sub_type" => {
            if let Some(ix) = find_interaction(model, &entry.owner) {
                ix.substance.sub_type = decode(&entry.value);
            }
        }
        "substance.type" => {
            if let Some(ix) = find_interaction(model, &entry.owner) {
                ix.substance.ty = decode(&entry.value);
            }
        }
        "unit" => {
            if let Some(ix) = find_interaction(model, &entry.owner) {
                ix.unit = decode(&entry.value);
            }
        }
        "amount" => {
            if let Some(ix) = find_interaction(model, &entry.owner) {
                ix.amount = decode(&entry.value);
            }
        }
        "interaction.type" => {
            if let Some(ix) = find_interaction(model, &entry.owner) {
                ix.ty = decode(&entry.value);
            }
        }
        "usability" => {
            if let Some(ix) = find_interaction(model, &entry.owner) {
                ix.usability = decode(&entry.value);
            }
        }
        "source_interface" => {
            if let Some(ix) = find_interaction(model, &entry.owner) {
                ix.source_interface = decode(&entry.value);
            }
        }
        "sink_interface" => {
            if let Some(ix) = find_interaction(model, &entry.owner) {
                ix.sink_interface = decode(&entry.value);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Boundary, Complexity, Environment, IdType, Info, Interaction, InteractionUsability,
        Interface, InterfaceType, Substance, System, CURRENT_FILE_VERSION,
    };

    fn id(ty: IdType, indices: &[i64]) -> Id {
        Id {
            ty,
            indices: indices.to_vec(),
        }
    }

    fn info(i: Id, level: i32, name: &str) -> Info {
        Info {
            id: i,
            level,
            name: name.to_string(),
            description: String::new(),
        }
    }

    fn blank_system(sid: Id, name: &str, parent: Id) -> System {
        let level = sid.indices.len() as i32 - 1;
        let bid = Id {
            ty: IdType::Boundary,
            indices: sid.indices.clone(),
        };
        System {
            info: info(sid, level, name),
            sources: vec![],
            sinks: vec![],
            parent,
            complexity: Complexity::Atomic,
            boundary: Boundary {
                info: info(bid, level, ""),
                porosity: 0.0,
                perceptive_fuzziness: 0.0,
                interfaces: vec![],
                parent_interface: None,
            },
            radius: 50.0,
            transform: None,
            equivalence: String::new(),
            history: String::new(),
            transformation: String::new(),
            member_autonomy: 1.0,
            time_constant: String::new(),
            archetype: None,
            agent: None,
        }
    }

    fn iface(iid: Id, ty: InterfaceType) -> Interface {
        Interface {
            info: info(iid, 2, "i"),
            protocol: String::new(),
            ty,
            exports_to: vec![],
            receives_from: vec![],
            angle: Some(0.0),
        }
    }

    /// A minimal (Core-level) interaction: every operational slot at its witness
    /// baseline, so it is a bond and nothing more.
    fn minimal_flow(idx: i64, name: &str, source: Id, sink: Id) -> Interaction {
        Interaction {
            info: info(id(IdType::Flow, &[idx]), 0, name),
            substance: Substance {
                sub_type: String::new(),
                ty: SubstanceType::default(),
            },
            ty: InteractionType::default(),
            usability: MIN_USABILITY,
            source,
            source_interface: None,
            sink,
            sink_interface: None,
            amount: Decimal::ZERO,
            unit: String::new(),
            parameters: vec![],
            smart_parameters: vec![],
            endpoint_offset: None,
        }
    }

    /// Root S0 with two bonded components A and B; `mode` and elaboration are the
    /// caller's to set. Valid, bonded (HasBond), and irreflexive.
    fn scaffold(mode: Mode) -> WorldModel {
        let env = id(IdType::Environment, &[-1]);
        let s0 = id(IdType::System, &[0]);
        let a = id(IdType::Subsystem, &[0, 0]);
        let b = id(IdType::Subsystem, &[0, 1]);

        let root = blank_system(s0.clone(), "Root", env.clone());
        let comp_a = blank_system(a.clone(), "A", s0.clone());
        let comp_b = blank_system(b.clone(), "B", s0);

        WorldModel {
            version: CURRENT_FILE_VERSION,
            mode: Some(mode),
            environment: Environment {
                info: info(env, -1, "Environment"),
                sources: vec![],
                sinks: vec![],
            },
            systems: vec![root, comp_a, comp_b],
            interactions: vec![minimal_flow(0, "bond", a, b)],
            hidden_entities: vec![],
        }
    }

    /// A fully elaborated Full model: dynamical face on the root, and the bond
    /// carrying substance, unit, amount, a non-minimal role, interfaces, and
    /// boundary properties — so every catalogue slot is non-minimal.
    fn full_model() -> WorldModel {
        let mut m = scaffold(Mode::Full);
        let ifa = id(IdType::Interface, &[0, 0, 0]);
        let ifb = id(IdType::Interface, &[0, 1, 0]);

        m.systems[0].transformation = "assemble".to_string();
        m.systems[0].history = "log".to_string();
        m.systems[0].time_constant = "Second".to_string();

        m.systems[1].boundary.porosity = 0.3;
        m.systems[1].boundary.perceptive_fuzziness = 0.1;
        m.systems[1].boundary.interfaces = vec![iface(ifa.clone(), InterfaceType::Export)];
        m.systems[2].boundary.interfaces = vec![iface(ifb.clone(), InterfaceType::Import)];

        let bond = &mut m.interactions[0];
        bond.substance = Substance {
            sub_type: "water".to_string(),
            ty: SubstanceType::Material,
        };
        bond.unit = "L".to_string();
        bond.amount = Decimal::from(5);
        bond.ty = InteractionType::Force;
        bond.usability = InteractionUsability::Resource;
        bond.source_interface = Some(ifa);
        bond.sink_interface = Some(ifb);
        m
    }

    fn as_json(m: &WorldModel) -> serde_json::Value {
        serde_json::to_value(m).expect("model serializes")
    }

    fn downgrade(m: &WorldModel, to: Mode) -> (WorldModel, LossWitness) {
        match validate_transition(m, to).expect("descent never errs") {
            Transition::Downgrade { model, loss } => (model, loss),
            other => panic!("expected a downgrade to {to:?}, got {other:?}"),
        }
    }

    fn upgraded(m: &WorldModel, to: Mode) -> WorldModel {
        match validate_transition(m, to).expect("upgrade precondition holds") {
            Transition::Upgrade { model } => model,
            other => panic!("expected an upgrade to {to:?}, got {other:?}"),
        }
    }

    // ---- L1 round-trip (completeness) -----------------------------------
    //
    // Field-for-field over the current `WorldModel` fields (memo A2): recorded
    // runs H are out of scope by A2's Gate-4 constraint — H must live outside
    // `WorldModel`, so no `H` field participates in this round-trip.

    #[test]
    fn l1_full_to_operational_round_trips() {
        let m = full_model();
        let (down, loss) = downgrade(&m, Mode::Operational);
        assert_eq!(as_json(&rebuild(down, &loss)), as_json(&m));
    }

    #[test]
    fn l1_full_to_core_round_trips() {
        let m = full_model();
        let (down, loss) = downgrade(&m, Mode::Core);
        assert_eq!(as_json(&rebuild(down, &loss)), as_json(&m));
    }

    #[test]
    fn l1_structural_to_core_round_trips() {
        let mut m = full_model();
        m.mode = Some(Mode::Structural);
        let (down, loss) = downgrade(&m, Mode::Core);
        assert!(loss.entries.is_empty(), "Structural owns no stored field");
        assert_eq!(as_json(&rebuild(down, &loss)), as_json(&m));
    }

    // ---- L2 witness minimality ------------------------------------------

    #[test]
    fn l2_no_witness_entry_equals_its_minimal() {
        let m = full_model();
        let (_, loss) = downgrade(&m, Mode::Core);
        assert!(!loss.entries.is_empty(), "the rich model sheds real data");
        for e in &loss.entries {
            match e.slot.as_str() {
                "unit" | "substance.sub_type" | "transformation" | "history" | "time_constant" => {
                    assert_ne!(
                        e.value,
                        serde_json::Value::String(String::new()),
                        "{}",
                        e.slot
                    )
                }
                "source_interface" | "sink_interface" | "boundary.parent_interface" => {
                    assert_ne!(e.value, serde_json::Value::Null, "{}", e.slot)
                }
                _ => {}
            }
        }
    }

    #[test]
    fn l2_upgrade_then_downgrade_is_empty() {
        let core = scaffold(Mode::Core);
        for target in [Mode::Structural, Mode::Operational] {
            let up = upgraded(&core, target);
            let (_, loss) = downgrade(&up, Mode::Core);
            assert!(
                loss.entries.is_empty(),
                "{target:?}→Core after Core→{target:?} must shed nothing: {:?}",
                loss.entries
            );
        }
        let op = scaffold(Mode::Operational);
        let up = upgraded(&op, Mode::Full);
        let (_, loss) = downgrade(&up, Mode::Operational);
        assert!(
            loss.entries.is_empty(),
            "Full→Operational after Operational→Full: {:?}",
            loss.entries
        );
    }

    // ---- L3 kernel invariance -------------------------------------------

    #[test]
    fn l3_kernel_is_byte_identical_across_every_transition() {
        for start in [Mode::Core, Mode::Structural, Mode::Operational, Mode::Full] {
            let mut m = full_model();
            m.mode = Some(start);
            let before = m.kernel();
            for to in [Mode::Core, Mode::Structural, Mode::Operational, Mode::Full] {
                let after = match validate_transition(&m, to).expect("transition") {
                    Transition::Identity => m.kernel(),
                    Transition::Downgrade { model, .. } => model.kernel(),
                    Transition::Upgrade { model } => model.kernel(),
                };
                assert_eq!(before, after, "{start:?}→{to:?} moved the kernel");
            }
        }
    }

    // ---- L4 commuting ---------------------------------------------------

    #[test]
    fn l4_full_to_core_equals_composition_witnesses_concatenate() {
        let m = full_model();
        let (direct_model, direct_loss) = downgrade(&m, Mode::Core);

        let (mid_model, first_loss) = downgrade(&m, Mode::Operational);
        let (composed_model, second_loss) = downgrade(&mid_model, Mode::Core);

        assert_eq!(
            as_json(&direct_model),
            as_json(&composed_model),
            "direct and composed descents land on the same Core model"
        );

        let mut concatenated = first_loss.entries.clone();
        concatenated.extend(second_loss.entries.clone());
        assert_eq!(
            direct_loss.entries, concatenated,
            "witness order is outermost edge (Full→Operational) first, then Operational→Core"
        );
    }

    // ---- L5 refusal -----------------------------------------------------

    #[test]
    fn l5_downgrades_never_err() {
        let mut m = full_model();
        m.mode = Some(Mode::Full);
        // Even a self-loop (illegal at Operational) descends without error.
        m.interactions.push(minimal_flow(
            9,
            "loop",
            id(IdType::Subsystem, &[0, 0]),
            id(IdType::Subsystem, &[0, 0]),
        ));
        for to in [Mode::Operational, Mode::Core] {
            assert!(
                validate_transition(&m, to).is_ok(),
                "descent to {to:?} must not err"
            );
        }
    }

    #[test]
    fn l5_upgrade_errs_iff_precondition_fails_and_cites_hypothesis() {
        // No bond: entering Structural refuses citing HasBond.
        let mut unbonded = scaffold(Mode::Core);
        unbonded.interactions.clear();
        let errs = validate_transition(&unbonded, Mode::Structural).unwrap_err();
        assert!(
            errs.iter().any(|e| e.reason.contains("Kernel.HasBond")),
            "missing-bond refusal cites HasBond: {errs:#?}"
        );

        // Self-loop: entering Operational refuses citing Irreflexive.
        let mut looped = scaffold(Mode::Core);
        looped.interactions.push(minimal_flow(
            1,
            "loop",
            id(IdType::Subsystem, &[0, 0]),
            id(IdType::Subsystem, &[0, 0]),
        ));
        let errs = validate_transition(&looped, Mode::Operational).unwrap_err();
        assert!(
            errs.iter().any(|e| e.reason.contains("Kernel.Irreflexive")),
            "self-loop refusal cites Irreflexive: {errs:#?}"
        );

        // A bonded, irreflexive model upgrades cleanly.
        assert!(validate_transition(&scaffold(Mode::Core), Mode::Structural).is_ok());
        assert!(validate_transition(&scaffold(Mode::Core), Mode::Operational).is_ok());
    }

    // ---- L6 stratification ----------------------------------------------
    //
    // Entering a rung is not the executability gate on top of it: a model can
    // reach Operational/Full through `validate_transition` yet be refused by
    // `validate_operational` (bert#108). Two fixtures pin the two ways this
    // happens — an isolated component, and an interface-routed flow.

    fn isolated_component_model() -> WorldModel {
        // Root S0 with a source→A flow, plus a stray component B no flow touches.
        let env = id(IdType::Environment, &[-1]);
        let s0 = id(IdType::System, &[0]);
        let a = id(IdType::Subsystem, &[0, 0]);
        let b = id(IdType::Subsystem, &[0, 1]);
        let src = id(IdType::Source, &[-1, 0]);

        let mut root = blank_system(s0.clone(), "Root", env.clone());
        root.info.level = 0;
        let mut comp_a = blank_system(a.clone(), "A", s0.clone());
        comp_a.agent = Some(crate::AgentModel {
            primitive: Some(crate::ProcessPrimitive::Buffering),
            ..Default::default()
        });
        let mut comp_b = blank_system(b.clone(), "B", s0);
        comp_b.agent = Some(crate::AgentModel {
            primitive: Some(crate::ProcessPrimitive::Propelling),
            ..Default::default()
        });

        let flow = minimal_flow(0, "src→A", src.clone(), a);

        WorldModel {
            version: CURRENT_FILE_VERSION,
            mode: Some(Mode::Operational),
            environment: Environment {
                info: info(env, -1, "Environment"),
                sources: vec![crate::ExternalEntity {
                    info: info(src, -1, "Well"),
                    ty: crate::ExternalEntityType::Source,
                    transform: None,
                    equivalence: String::new(),
                    model: String::new(),
                    is_same_as_id: None,
                }],
                sinks: vec![],
            },
            systems: vec![root, comp_a, comp_b],
            interactions: vec![flow],
            hidden_entities: vec![],
        }
    }

    #[test]
    fn l6_reaching_full_does_not_imply_operational_executability() {
        let m = isolated_component_model();
        // Enters Full (irreflexive): the transition layer is satisfied.
        assert!(
            validate_transition(&m, Mode::Full).is_ok(),
            "an irreflexive model reaches Full"
        );
        // But the executability gate refuses the isolated component.
        let errs = crate::operational::validate_operational(&m).unwrap_err();
        assert!(
            errs.iter().any(|e| e.reason.contains("no flow touches")),
            "validate_operational refuses the stray component: {errs:#?}"
        );
    }

    #[test]
    fn l6_transition_to_operational_accepts_interface_routed_flow_lowers() {
        // A bonded model whose bond routes through an interface: valid Operational
        // structure the transition layer must accept (its only precondition is
        // Irreflexive). bert#108 4.2 has now shipped, so validate_operational no
        // longer refuses interface routing — it LOWERS it (identity default),
        // recording the Impeding provenance on the flow.
        let mut m = scaffold(Mode::Core);
        let ifb = id(IdType::Interface, &[0, 1, 0]);
        m.systems[2].boundary.interfaces = vec![iface(ifb.clone(), InterfaceType::Import)];
        m.interactions[0].sink_interface = Some(ifb);

        assert!(
            matches!(
                validate_transition(&m, Mode::Operational).unwrap(),
                Transition::Upgrade { .. }
            ),
            "transition to Operational must not reject interface routing"
        );

        let mut op = m.clone();
        op.mode = Some(Mode::Operational);
        // This bare structural scaffold still cannot execute (its systems carry
        // no agent primitives), but bert#108 4.2 has shipped: interface routing
        // is no longer among the reasons. The refusal that used to cite #108 is
        // gone — the routing lowers instead.
        let errs = crate::operational::validate_operational(&op).unwrap_err();
        assert!(
            !errs.iter().any(|e| e.reason.contains("bert#108")),
            "interface routing no longer refuses citing #108: {errs:#?}"
        );
    }

    // ---- L7 immutability ------------------------------------------------

    #[test]
    fn l7_input_is_never_mutated() {
        let m = full_model();
        let before = as_json(&m);
        let _ = validate_transition(&m, Mode::Core).expect("descent");
        let _ = validate_transition(&scaffold(Mode::Core), Mode::Operational).expect("ascent");
        assert_eq!(as_json(&m), before, "the borrowed model must be untouched");
    }

    // ---- Identity and cross-moves ---------------------------------------

    #[test]
    fn identity_when_from_equals_to() {
        let m = full_model();
        assert!(matches!(
            validate_transition(&m, Mode::Full).unwrap(),
            Transition::Identity
        ));
    }

    #[test]
    fn cross_move_structural_to_operational_upgrades_through_the_meet() {
        let m = scaffold(Mode::Structural);
        // Structural→Core sheds nothing, then Core→Operational checks Irreflexive:
        // a net ascent.
        assert!(matches!(
            validate_transition(&m, Mode::Operational).unwrap(),
            Transition::Upgrade { .. }
        ));
    }

    #[test]
    fn cross_move_operational_to_structural_sheds_then_reascends() {
        let m = full_model_operational();
        // Operational→Core sheds real data, then Core→Structural checks HasBond:
        // a net descent carrying loss.
        match validate_transition(&m, Mode::Structural).unwrap() {
            Transition::Downgrade { loss, .. } => {
                assert!(!loss.entries.is_empty(), "operational data was shed");
            }
            other => panic!("expected a downgrade, got {other:?}"),
        }
    }

    fn full_model_operational() -> WorldModel {
        let mut m = full_model();
        m.systems[0].transformation.clear();
        m.systems[0].history.clear();
        m.systems[0].time_constant.clear();
        m.mode = Some(Mode::Operational);
        m
    }

    // ---- Lean citation drift gate ---------------------------------------

    /// The Lean declaration names this module cites in its doc-comments. A rename
    /// in `ViewGeneration.lean` orphans a citation; this catches it the same day,
    /// string-level, with no Lake dependency.
    const CITED_LEAN_DECLS: [&str; 4] = [
        "HasBond",
        "Irreflexive",
        "toMobus_toKlir",
        "toMobus_toBunge",
    ];

    #[test]
    fn lean_citations_resolve_or_skip_gracefully() {
        let source = include_str!("transition.rs");
        for name in CITED_LEAN_DECLS {
            assert!(
                source.contains(name),
                "{name} is declared cited but does not appear in this module"
            );
        }

        let lean = format!(
            "{}/../../systems-science-foundations/Systems/Klir/ViewGeneration.lean",
            env!("CARGO_MANIFEST_DIR")
        );
        let Ok(text) = std::fs::read_to_string(&lean) else {
            eprintln!("SKIP lean_citations_resolve: SSF repo not found at {lean}");
            return;
        };
        for name in CITED_LEAN_DECLS {
            assert!(
                text.contains(name),
                "cited Lean declaration `{name}` is absent from ViewGeneration.lean — a rename orphaned it"
            );
        }
    }
}
