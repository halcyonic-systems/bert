//! The **recorded trace** — the trajectory a circuit traces under `(T, Δt)` —
//! kept OUTSIDE the `WorldModel` (memo A2, `bert-core/src/transition.rs`) and
//! keyed to a content hash of the [`OperationalSpec`] it ran on. A structural
//! edit moves the spec's hash, so a recording no longer matches the model and
//! surfaces as stale: the old trajectory never poses as current.
//!
//! This is an *observer's* record of a simulation, distinct in kind from the
//! 8-tuple's H slot (the authored `System.history`, η): that lives in the
//! model, is serialized, and can condition the system's own transfer; this is
//! pure downstream output that never feeds back and is never serialized. The
//! two must not be conflated — this trace is not the system's "memory" (grounding
//! F1). It is a session artifact, discarded when the app closes.
//!
//! The engine is a discrete synchronous map — one `Circuit::step` advances one
//! Δt. The transfer functions are defined per-Δt, so unit Δt reproduces the
//! engine's native behavior exactly; Δt is the physical duration a tick
//! represents, which is what turns a horizon `T` into a tick count (`T / Δt`).
//! A run is therefore `(T, Δt) →` a recorded trace, which carries the Δt it ran
//! under.

use crate::circuit::Circuit;
use bert_core::operational::OperationalSpec;

/// One recorded run: the trace a circuit traced, the Δt it advanced under, and
/// the spec-hash key that says which model the trace belongs to.
pub struct RecordedRun {
    /// [`OperationalSpec::content_hash`] at the moment of the run — the identity
    /// of the model this trace belongs to. Not the model itself: A2 keeps the
    /// trace out of the `WorldModel`, so the tie is a hash held here, never a
    /// field over there.
    key: u64,
    /// The step size each tick advanced under (the Δt the trace ran at).
    pub dt: f64,
    /// Per-tick rows `[tick, n0.activity, n0.storage, n0.total, n1…]` — the
    /// circuit's own recording, snapshotted so the trace is self-contained.
    pub history: Vec<Vec<f32>>,
    /// Per-tick conservation ledger `[emitted, sunk, stored, dissipated]`.
    pub ledger_history: Vec<[f32; 4]>,
    /// The conservation residual at the end of the run (≈0 = mass conserved).
    pub final_balance: f32,
}

impl RecordedRun {
    /// Run `circuit` for `ticks` steps of size `dt`, recording the trace keyed
    /// to `spec` — the projection the circuit was built from. Resets first so the
    /// run starts from the asserted initial stocks.
    pub fn record(circuit: &mut Circuit, spec: &OperationalSpec, dt: f64, ticks: usize) -> Self {
        circuit.reset();
        for _ in 0..ticks {
            circuit.step();
        }
        Self {
            key: spec.content_hash(),
            dt,
            history: circuit.history.clone(),
            ledger_history: circuit.ledger_history.clone(),
            final_balance: circuit.balance(),
        }
    }

    /// Run `circuit` over a horizon `total_time` at step size `dt`: the `(T, Δt)`
    /// form. The tick count is `round(total_time / dt)`, so halving Δt doubles
    /// the resolution of the same horizon.
    pub fn record_over(
        circuit: &mut Circuit,
        spec: &OperationalSpec,
        dt: f64,
        total_time: f64,
    ) -> Self {
        let ticks = (total_time / dt).round().max(0.0) as usize;
        Self::record(circuit, spec, dt, ticks)
    }

    /// Does this trace still belong to `spec`? True iff the spec hashes to the
    /// key the run recorded — a structural edit breaks it.
    pub fn is_valid_for(&self, spec: &OperationalSpec) -> bool {
        self.key == spec.content_hash()
    }

    /// The trajectory, but only for the spec it actually ran on. `None` once the
    /// model has structurally changed — a stale trace is refused, not returned
    /// as if current.
    pub fn history_for(&self, spec: &OperationalSpec) -> Option<&[Vec<f32>]> {
        self.is_valid_for(spec).then_some(self.history.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::{Circuit, Node, NodeKind, Wire};
    use crate::export::{from_spec, to_world_model};
    use bert_core::operational::{validate_operational, InterfacePrimitive};
    use bert_core::{Id, IdType, Info, Interface, InterfaceType, ProcessPrimitive, WorldModel};
    use egui::pos2;

    /// A conserving source → buffer → sink circuit exported to a model.
    fn conserving_model() -> WorldModel {
        let mut c = Circuit::default();
        c.nodes
            .push(Node::new(NodeKind::Source, 1, pos2(-200.0, 0.0)));
        c.nodes.push(Node::new(
            NodeKind::Process(ProcessPrimitive::Buffering),
            2,
            pos2(0.0, 0.0),
        ));
        c.nodes.push(Node::new(NodeKind::Sink, 3, pos2(200.0, 0.0)));
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::new(1, 2));
        c.nodes[0].param = 2.5;
        c.nodes[1].initial_storage = 6.0;
        to_world_model(&c, "Interface Routed")
    }

    /// Route the source → buffer flow through a real Import interface on the
    /// buffer's boundary: the exact shape validate_operational refused before
    /// bert#108 4.2. The interface is attached so it resolves (a dangling id
    /// would fail `validate` for a different reason).
    fn route_through_interface(model: &mut WorldModel) {
        let iface_id = Id {
            ty: IdType::Interface,
            indices: vec![0, 0, 0],
        };
        let iface = Interface {
            info: Info {
                id: iface_id.clone(),
                level: 1,
                name: "in".to_string(),
                description: String::new(),
            },
            protocol: String::new(),
            ty: InterfaceType::Import,
            exports_to: vec![],
            receives_from: vec![],
            angle: Some(0.0),
        };
        let buffer = model
            .systems
            .iter_mut()
            .find(|s| s.info.level == 1)
            .expect("the buffer is a level-1 system");
        buffer.boundary.interfaces.push(iface);
        model.interactions[0].sink_interface = Some(iface_id);
    }

    /// THE bert#108 4.2 acceptance test (this IS the contract): a model that
    /// routes a flow through a boundary interface — which validate_operational
    /// REFUSED before the identity-default lowering — now projects, builds a
    /// circuit, and runs 30 ticks with the ledger balanced, with ZERO edits to
    /// the model. Interface routing is runnable input end to end.
    #[test]
    fn interface_routed_model_runs_unedited_bert108() {
        let mut model = conserving_model();
        route_through_interface(&mut model);

        // Pre-change refusal is GONE: interface routing now lowers, not refuses.
        let spec = validate_operational(&model)
            .expect("bert#108 4.2: an interface-routed model is runnable input, unedited");
        assert!(
            spec.flows
                .iter()
                .any(|f| f.interface_routing == Some(InterfacePrimitive::Impeding)),
            "the routed flow carries its bert#108 Impeding lowering provenance"
        );

        // ZERO edits: the same model builds a circuit and runs, conserving.
        let mut circuit = from_spec(&spec);
        let run = RecordedRun::record(&mut circuit, &spec, 1.0, 30);
        assert_eq!(run.history.len(), 30, "30 ticks recorded");
        assert_eq!(
            run.ledger_history.len(),
            30,
            "the conservation ledger is recorded per tick alongside H"
        );
        assert!(
            run.final_balance.abs() < 1e-3,
            "conserves over 30 ticks: residual {}",
            run.final_balance
        );
    }

    /// H is keyed to the spec it ran on and lives here, outside the WorldModel
    /// (A2). A structural edit moves the key, so the recording surfaces as stale
    /// rather than silently returning the old trajectory.
    #[test]
    fn structural_edit_invalidates_recorded_h() {
        let model = conserving_model();
        let spec = validate_operational(&model).expect("projects");
        let mut circuit = from_spec(&spec);
        let run = RecordedRun::record(&mut circuit, &spec, 1.0, 30);

        assert!(run.is_valid_for(&spec), "H is valid for the spec it ran on");
        assert!(
            run.history_for(&spec).is_some(),
            "the trajectory is returned for its own spec"
        );

        // A structural edit: change the buffer's work-process primitive.
        let mut edited = model.clone();
        edited.systems[1].agent.as_mut().unwrap().primitives = vec![ProcessPrimitive::Propelling];
        let spec2 = validate_operational(&edited).expect("still projects");

        assert!(
            !run.is_valid_for(&spec2),
            "a structural edit invalidates the recorded H"
        );
        assert!(
            run.history_for(&spec2).is_none(),
            "stale H is refused, not silently returned as current"
        );
    }

    /// Δt supply: a run accepts an explicit Δt and stamps it into H; the `(T, Δt)`
    /// form turns a horizon into a tick count; and the Δt=1 driver is exactly
    /// stepping the circuit N times (observational equivalence).
    #[test]
    fn run_accepts_explicit_dt() {
        let model = conserving_model();
        let spec = validate_operational(&model).expect("projects");

        let mut c1 = from_spec(&spec);
        let run = RecordedRun::record(&mut c1, &spec, 0.5, 30);
        assert_eq!(run.dt, 0.5, "the run records its Δt");
        assert_eq!(run.history.len(), 30);

        let mut c2 = from_spec(&spec);
        let over = RecordedRun::record_over(&mut c2, &spec, 0.5, 15.0);
        assert_eq!(
            over.history.len(),
            30,
            "T = 15 at Δt = 0.5 is 30 ticks (T / Δt)"
        );

        // Δt = 1 over N ticks reproduces N raw steps, byte for byte.
        let mut c3 = from_spec(&spec);
        let driven = RecordedRun::record(&mut c3, &spec, 1.0, 20);
        let mut c4 = from_spec(&spec);
        c4.reset();
        for _ in 0..20 {
            c4.step();
        }
        assert_eq!(
            driven.history, c4.history,
            "the Δt = 1 driver is exactly stepping the circuit"
        );
    }
}
