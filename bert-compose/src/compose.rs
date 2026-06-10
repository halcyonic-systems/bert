//! Model composition — the algebra licensed by the Lean formalization.
//!
//! `Systems/Mobus/Composition.lean` (systems-science-foundations) proves that
//! 8-tuple composition is **unconditional** and that the bipartite constraint
//! on external flows transfers (`bipartite_edge_classification`): composition
//! only ever *removes* external edges — a bound pair of boundary flows is
//! reclassified as one internal bond (both endpoints now inside the composed
//! component set), and every unbound external flow provably survives with
//! exactly one endpoint inside, one outside. Nothing else can happen.
//!
//! This module is that theorem, operationally: two `WorldModel`s become
//! subsystems of a fresh composite root; user-chosen sink↔source bindings
//! become internal flows whose interfaces persist (boundary contraction);
//! everything unbound lands in the composite environment.

use bert_core::{
    Boundary, Complexity, Environment, ExternalEntity, Id, IdType, Info,
    Interaction, System, WorldModel,
};

/// One sink↔source binding between the two models.
/// `sink_a` indexes into A's environment sinks when `a_to_b`, otherwise B's;
/// `source_b` symmetrical.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binding {
    /// Index into the *outflow* model's environment sinks.
    pub sink_idx: usize,
    /// Index into the *inflow* model's environment sources.
    pub source_idx: usize,
    /// true = A's sink feeds B's source; false = B's sink feeds A's source.
    pub a_to_b: bool,
}

/// What composition produced beyond the sum of parts — the "what emerged"
/// panel's data, and the audit trail for each theorem-licensed move.
#[derive(Debug, Default)]
pub struct Emergence {
    /// New internal flows created from bound external pairs (names).
    pub internal_bonds: Vec<String>,
    /// External entities consumed by internalization (names).
    pub internalized: Vec<String>,
    /// External flows that survive across the composite boundary (names).
    pub surviving: Vec<String>,
}

fn prefix_id(id: &Id, slot: i64) -> Id {
    // Ids are scope paths: indices[0] == -1 is environment scope (handled by
    // the remap in `compose`); indices[0] == 0 is inside-the-root scope and
    // gets the composite slot spliced in: [0, rest..] -> [0, slot, rest..].
    if id.indices.first() != Some(&0) {
        return id.clone();
    }
    let mut indices = Vec::with_capacity(id.indices.len() + 1);
    indices.push(0);
    indices.push(slot);
    indices.extend(id.indices.iter().skip(1));
    // The root ([0], len 1) becomes a subsystem ([0, slot]).
    let ty = if id.ty == IdType::System && id.indices.len() == 1 {
        IdType::Subsystem
    } else {
        id.ty
    };
    Id { ty, indices }
}

fn prefix_all(model: &mut WorldModel, slot: i64) {
    let fix = |id: &mut Id| *id = prefix_id(id, slot);
    for sys in &mut model.systems {
        fix(&mut sys.info.id);
        sys.info.level += 1;
        fix(&mut sys.parent);
        fix(&mut sys.boundary.info.id);
        if let Some(pi) = &mut sys.boundary.parent_interface {
            fix(pi);
        }
        for iface in &mut sys.boundary.interfaces {
            fix(&mut iface.info.id);
            iface.info.level += 1;
            for id in iface.exports_to.iter_mut().chain(iface.receives_from.iter_mut()) {
                fix(id);
            }
        }
        for ext in sys.sources.iter_mut().chain(sys.sinks.iter_mut()) {
            fix(&mut ext.info.id);
            ext.info.level += 1;
        }
    }
    for flow in &mut model.interactions {
        fix(&mut flow.info.id);
        fix(&mut flow.source);
        fix(&mut flow.sink);
        if let Some(si) = &mut flow.source_interface {
            fix(si);
        }
        if let Some(si) = &mut flow.sink_interface {
            fix(si);
        }
    }
    for id in &mut model.hidden_entities {
        fix(id);
    }
    // Re-parent the (former) root to the composite root.
    if let Some(root) = model.systems.first_mut() {
        root.parent = Id { ty: IdType::System, indices: vec![0] };
    }
}

fn root_id() -> Id {
    Id { ty: IdType::System, indices: vec![0] }
}

/// Compose two models into one, per the given bindings.
/// Returns the composite plus the emergence audit.
pub fn compose(
    a: &WorldModel,
    b: &WorldModel,
    bindings: &[Binding],
    name: &str,
) -> (WorldModel, Emergence) {
    let mut a = a.clone();
    let mut b = b.clone();
    prefix_all(&mut a, 0);
    prefix_all(&mut b, 1);

    let mut emergence = Emergence::default();

    // Partition each side's environment externals into bound / unbound.
    let mut a_sinks_bound = vec![false; a.environment.sinks.len()];
    let mut a_sources_bound = vec![false; a.environment.sources.len()];
    let mut b_sinks_bound = vec![false; b.environment.sinks.len()];
    let mut b_sources_bound = vec![false; b.environment.sources.len()];

    let mut interactions: Vec<Interaction> = Vec::new();

    // Bound pairs → one internal bond each. The outflow side's interaction
    // (… → sink) supplies substance + interfaces on the source end; the
    // inflow side's (source → …) supplies the destination end.
    for binding in bindings {
        let (out_model, in_model, out_bound, in_bound) = if binding.a_to_b {
            (&a, &b, &mut a_sinks_bound, &mut b_sources_bound)
        } else {
            (&b, &a, &mut b_sinks_bound, &mut a_sources_bound)
        };
        let Some(sink_ext) = out_model.environment.sinks.get(binding.sink_idx) else { continue };
        let Some(source_ext) = in_model.environment.sources.get(binding.source_idx) else {
            continue;
        };
        let out_flow = out_model.interactions.iter().find(|f| f.sink == sink_ext.info.id);
        let in_flow = in_model.interactions.iter().find(|f| f.source == source_ext.info.id);
        let (Some(out_flow), Some(in_flow)) = (out_flow, in_flow) else { continue };

        let mut bond = out_flow.clone();
        // Fresh composite-root-scope id — bonds are new edges in scope [0].
        bond.info.id = Id {
            ty: IdType::Flow,
            indices: vec![0, emergence.internal_bonds.len() as i64],
        };
        bond.info.level = 0;
        bond.sink = in_flow.sink.clone();
        bond.sink_interface = in_flow.sink_interface.clone();
        bond.info.name = format!("{} ⇒ {}", out_flow.info.name, in_flow.info.name);
        bond.info.description = format!(
            "Internal bond from composition: '{}' bound to '{}'. Licensed by \
             bipartite_edge_classification (both endpoints inside the composite).",
            sink_ext.info.name, source_ext.info.name
        );
        emergence.internal_bonds.push(bond.info.name.clone());
        emergence.internalized.push(sink_ext.info.name.clone());
        emergence.internalized.push(source_ext.info.name.clone());
        interactions.push(bond);

        out_bound[binding.sink_idx] = true;
        in_bound[binding.source_idx] = true;
    }

    // Environment-scope entities get renumbered into the composite's
    // environment scope ([-1, n] per id type), with a remap table so the
    // surviving flows' endpoint references follow them.
    // Per-model remaps: environment-scope ids are NOT slot-prefixed, so the
    // same id string exists in both models — a shared table would cross-wire.
    let mut remap_a: Vec<(Id, Id)> = Vec::new();
    let mut remap_b: Vec<(Id, Id)> = Vec::new();
    let mut environment = Environment {
        info: Info {
            id: Id { ty: IdType::Environment, indices: vec![-1] },
            level: -1,
            name: a.environment.info.name.clone(),
            description: format!(
                "Composite environment of '{name}' (union of both environments minus \
                 internalized entities)"
            ),
        },
        sources: Vec::new(),
        sinks: Vec::new(),
    };
    let mut src_n: i64 = 0;
    let mut sink_n: i64 = 0;
    for (model, sources_bound, sinks_bound, remap) in [
        (&a, &a_sources_bound, &a_sinks_bound, &mut remap_a),
        (&b, &b_sources_bound, &b_sinks_bound, &mut remap_b),
    ] {
        for (ext, bound) in model.environment.sources.iter().zip(sources_bound) {
            if *bound {
                continue;
            }
            let mut e = ext.clone();
            let new_id = Id { ty: IdType::Source, indices: vec![-1, src_n] };
            src_n += 1;
            remap.push((e.info.id.clone(), new_id.clone()));
            e.info.id = new_id;
            environment.sources.push(e);
        }
        for (ext, bound) in model.environment.sinks.iter().zip(sinks_bound) {
            if *bound {
                continue;
            }
            let mut e = ext.clone();
            let new_id = Id { ty: IdType::Sink, indices: vec![-1, sink_n] };
            sink_n += 1;
            remap.push((e.info.id.clone(), new_id.clone()));
            e.info.id = new_id;
            environment.sinks.push(e);
        }
    }
    for e in environment.sources.iter().chain(environment.sinks.iter()) {
        emergence.surviving.push(e.info.name.clone());
    }

    // Surviving interactions: everything except the two halves of each bound
    // pair (fused above). Environment-scope flows get fresh [-1, n] ids and
    // their external endpoints follow the remap.
    let bound_ext_ids_a: Vec<Id> = a
        .environment
        .sinks
        .iter()
        .zip(&a_sinks_bound)
        .chain(a.environment.sources.iter().zip(&a_sources_bound))
        .filter(|(_, bound)| **bound)
        .map(|(e, _)| e.info.id.clone())
        .collect();
    let bound_ext_ids_b: Vec<Id> = b
        .environment
        .sinks
        .iter()
        .zip(&b_sinks_bound)
        .chain(b.environment.sources.iter().zip(&b_sources_bound))
        .filter(|(_, bound)| **bound)
        .map(|(e, _)| e.info.id.clone())
        .collect();
    let mut env_flow_n: i64 = 0;
    for (model, remap, bound_ids) in
        [(&a, &remap_a, &bound_ext_ids_a), (&b, &remap_b, &bound_ext_ids_b)]
    {
        for flow in &model.interactions {
            if bound_ids.contains(&flow.sink) || bound_ids.contains(&flow.source) {
                continue; // consumed by a bond
            }
            let mut f = flow.clone();
            if let Some((_, new_id)) = remap.iter().find(|(old, _)| *old == f.source) {
                f.source = new_id.clone();
            }
            if let Some((_, new_id)) = remap.iter().find(|(old, _)| *old == f.sink) {
                f.sink = new_id.clone();
            }
            if f.info.id.indices.first() == Some(&-1) {
                f.info.id = Id { ty: IdType::Flow, indices: vec![-1, env_flow_n] };
                env_flow_n += 1;
            }
            interactions.push(f);
        }
    }

    // The composite root: contains both former roots as subsystems.
    let composite_root = System {
        info: Info {
            id: root_id(),
            level: 0,
            name: name.to_string(),
            description: format!(
                "Composition of '{}' and '{}'. Composition is unconditional \
                 (Systems/Mobus/Composition.lean); {} external pair(s) internalized, \
                 {} external flow(s) survive across the new boundary.",
                a.systems.first().map(|s| s.info.name.as_str()).unwrap_or("A"),
                b.systems.first().map(|s| s.info.name.as_str()).unwrap_or("B"),
                bindings.len(),
                environment.sources.len() + environment.sinks.len(),
            ),
        },
        sources: Vec::new(),
        sinks: Vec::new(),
        parent: Id { ty: IdType::Environment, indices: vec![-1] },
        complexity: Complexity::Complex { adaptable: true, evolveable: true },
        boundary: Boundary {
            info: Info {
                id: Id { ty: IdType::Boundary, indices: vec![0] },
                level: 0,
                name: String::new(),
                description: String::new(),
            },
            porosity: 0.0,
            perceptive_fuzziness: 0.0,
            interfaces: Vec::new(),
            parent_interface: None,
        },
        radius: 600.0,
        transform: None,
        equivalence: String::new(),
        history: String::new(),
        transformation: String::new(),
        member_autonomy: 1.0,
        time_constant: String::new(),
        archetype: None,
        agent: None,
    };

    let mut systems = vec![composite_root];
    systems.extend(a.systems);
    systems.extend(b.systems);

    let mut hidden = a.hidden_entities;
    hidden.extend(b.hidden_entities);

    let composite = WorldModel {
        version: a.version,
        environment,
        systems,
        interactions,
        hidden_entities: hidden,
    };
    (composite, emergence)
}

/// Auto-suggest bindings: a sink and a source match when their substance
/// types agree on the flows that touch them (diversity derives from
/// interaction — typing comes from what flows, not from labels).
pub fn suggest_bindings(a: &WorldModel, b: &WorldModel) -> Vec<Binding> {
    let mut out = Vec::new();
    let substance_of_sink = |m: &WorldModel, e: &ExternalEntity| {
        m.interactions.iter().find(|f| f.sink == e.info.id).map(|f| f.substance.ty)
    };
    let substance_of_source = |m: &WorldModel, e: &ExternalEntity| {
        m.interactions.iter().find(|f| f.source == e.info.id).map(|f| f.substance.ty)
    };
    let mut b_used = vec![false; b.environment.sources.len()];
    for (i, sink) in a.environment.sinks.iter().enumerate() {
        let st = substance_of_sink(a, sink);
        if st.is_none() {
            continue;
        }
        for (j, source) in b.environment.sources.iter().enumerate() {
            if !b_used[j] && substance_of_source(b, source) == st {
                out.push(Binding { sink_idx: i, source_idx: j, a_to_b: true });
                b_used[j] = true;
                break;
            }
        }
    }
    let mut a_used = vec![false; a.environment.sources.len()];
    for (i, sink) in b.environment.sinks.iter().enumerate() {
        let st = substance_of_sink(b, sink);
        if st.is_none() {
            continue;
        }
        for (j, source) in a.environment.sources.iter().enumerate() {
            if !a_used[j] && substance_of_source(a, source) == st {
                out.push(Binding { sink_idx: i, source_idx: j, a_to_b: false });
                a_used[j] = true;
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load(name: &str) -> WorldModel {
        let path = format!(
            "{}/../assets/models/examples/{name}",
            env!("CARGO_MANIFEST_DIR")
        );
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn compose_bitcoin_ethereum_validates() {
        let a = load("bitcoin.json");
        let b = load("ethereum.json");
        let bindings = suggest_bindings(&a, &b);
        let (composite, emergence) = compose(&a, &b, &bindings, "Bitcoin × Ethereum");

        // Composition is unconditional: structure must hold for any bindings.
        assert_eq!(composite.systems[0].info.id.indices, vec![0]);
        assert_eq!(
            composite.systems.len(),
            1 + a.systems.len() + b.systems.len(),
            "both models nest under the composite root"
        );
        // Edge conservation: every original interaction is either fused
        // (two halves -> one bond) or survives. Never added (the theorem).
        assert_eq!(
            composite.interactions.len(),
            a.interactions.len() + b.interactions.len() - emergence.internal_bonds.len(),
        );
        // Internalization removes exactly the bound externals.
        let total_ext = |m: &WorldModel| m.environment.sources.len() + m.environment.sinks.len();
        assert_eq!(
            total_ext(&composite),
            total_ext(&a) + total_ext(&b) - emergence.internalized.len()
        );

        let result = bert_core::validate::validate(&composite);
        let errors: Vec<_> = result
            .issues
            .iter()
            .filter(|i| i.severity == bert_core::validate::Severity::Error)
            .collect();
        assert!(errors.is_empty(), "composite must validate: {errors:#?}");

        // Full file round-trip: the composite must survive disk as ordinary
        // BERT JSON (what "open it in the editor" depends on).
        let json = serde_json::to_string_pretty(&composite).unwrap();
        let path = std::env::temp_dir().join("bert-compose-roundtrip.json");
        std::fs::write(&path, &json).unwrap();
        let reloaded: WorldModel =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(reloaded.systems.len(), composite.systems.len());
        assert_eq!(reloaded.interactions.len(), composite.interactions.len());
        let re_result = bert_core::validate::validate(&reloaded);
        assert!(
            !re_result
                .issues
                .iter()
                .any(|i| i.severity == bert_core::validate::Severity::Error),
            "reloaded composite must validate"
        );
    }

    #[test]
    fn compose_no_bindings_is_disjoint_union_under_new_root() {
        let a = load("bitcoin.json");
        let b = load("cosmos-hub.json");
        let (composite, emergence) = compose(&a, &b, &[], "Disjoint");
        assert!(emergence.internal_bonds.is_empty());
        assert_eq!(
            composite.interactions.len(),
            a.interactions.len() + b.interactions.len()
        );
        let result = bert_core::validate::validate(&composite);
        let errors: Vec<_> = result
            .issues
            .iter()
            .filter(|i| i.severity == bert_core::validate::Severity::Error)
            .collect();
        assert!(errors.is_empty(), "disjoint composite must validate: {errors:#?}");
    }
}
