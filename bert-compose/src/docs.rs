//! Per-primitive teaching cards — plain English first, then math, substance,
//! theory, and the actual transfer function. Progressive disclosure so a
//! social scientist meets a sentence and an engineer can drill to the code.

use crate::circuit::NodeKind;
use bert_core::ProcessPrimitive;

pub struct Doc {
    /// One sentence anyone can read.
    pub plain: &'static str,
    /// A relatable everyday instance.
    pub everyday: &'static str,
    /// The rule, in symbols.
    pub math: &'static str,
    /// What substances it takes and gives.
    pub substance: &'static str,
    /// Where it comes from.
    pub theory: &'static str,
    /// The transfer function, lightly paraphrased from python/agents.py.
    pub code: &'static str,
}

pub fn doc(kind: NodeKind) -> Doc {
    use ProcessPrimitive::*;
    match kind {
        NodeKind::Source => Doc {
            plain: "Where something enters the system from outside.",
            everyday: "A paycheck arriving, rain falling, new members joining.",
            math: "out = rate (every tick)",
            substance: "emits one substance you choose (matter, energy, or information).",
            theory: "Environment input — outside the boundary (Mobus 8-tuple: the milieu).",
            code: "activity = rate",
        },
        NodeKind::Sink => Doc {
            plain: "Where something leaves the system for good.",
            everyday: "Spending money, waste leaving, people exiting.",
            math: "total += everything that arrives",
            substance: "absorbs anything.",
            theory: "Environment output — beyond the boundary.",
            code: "total += inflow",
        },
        NodeKind::Process(p) => match p {
            Buffering => Doc {
                plain: "A store. It holds what flows in and releases at its own rate — the system's memory. A capacity bounds it: above the ceiling it overflows.",
                everyday: "A savings account, a reservoir, a warehouse, a population.",
                math: "stock += inflow;  out = min(stock, rate · gate);  stock ≤ capacity",
                substance: "holds matter or energy; an optional information input can throttle the release (a valve). Capacity 0 = unbounded. Drain is a fixed rate or first-order (τ). Maintenance is a continuous upkeep loss, dissipated (self-discharge, spoilage).",
                theory: "Mobus Ch.3 — the conservative reservoir, the ONLY primitive that carries state. It 'smooths flow over time' (Ch.3 → time-constant drain); containers have a capacity (Ch.4); overflow is waste (Fig 3.17).",
                code: "out = (tc>0) ? storage/tc : rate;   // first-order vs fixed\nreleased = min(storage, out * gate);\nstorage -= released;\nif cap > 0 { storage = min(storage, cap) }",
            },
            Combining => Doc {
                plain: "Merges several inflows into one.",
                everyday: "Tributaries joining a river, incomes pooling into a budget.",
                math: "out = Σ inflows",
                substance: "matter/energy only — you can't 'merge' information by adding it (that's not how signals work).",
                theory: "Mobus atomic work process. Conserves: nothing is lost or made.",
                code: "activity = sum(physical inflows)",
            },
            Splitting => Doc {
                plain: "Divides one inflow evenly across its outputs.",
                everyday: "Splitting a bill, distributing rations, a river delta.",
                math: "each out = total / (number of outputs)",
                substance: "matter/energy only — you SPLIT matter (divide a fixed amount) but you COPY information (use Copying for signals).",
                theory: "Mobus atomic work process. Conserves: the outputs always sum back to the input.",
                code: "share = total / n_outputs",
            },
            Amplifying => Doc {
                plain: "Makes a weak signal stronger — but only by drawing on a power supply.",
                everyday: "A megaphone, a transistor, a leader amplifying a movement (needs energy/resources).",
                math: "out = min(signal · gain, power available)",
                substance: "takes an information SIGNAL and an energy POWER; emits a stronger signal. Gain never makes mass — the output is capped by the power you feed it.",
                theory: "Mobus Fig 3.19. The 'no free lunch' primitive — amplification is metered by real power.",
                code: "gain = 1 + 9·agency;\nout = min(signal*gain, energy_in)",
            },
            Modulating => Doc {
                plain: "A valve. A control signal decides how much of a flow gets through.",
                everyday: "A thermostat throttling heat, a quorum gating a decision, a tap.",
                math: "out = primary · clamp(control, 0..1)",
                substance: "a physical flow as the primary input, an information signal as the control. The gate can only restrict (≤1), never amplify.",
                theory: "Mobus atomic work process. The regulating element of every feedback loop.",
                code: "out = primary * control.clamp(0,1)",
            },
            Sensing => Doc {
                plain: "Reads how much of something is flowing (or stored) and reports it as a signal.",
                everyday: "A thermometer, a vote count, a gauge — measuring without consuming.",
                math: "signal = k · (what it observes)",
                substance: "reads a physical flow OR a stock's level and emits information. Crucially, it reads without draining (observation is low-power).",
                theory: "Mobus Ch.3. The input side of every control loop — you can't regulate what you can't sense (Conant–Ashby).",
                code: "signal = observed_level * k  // non-draining read",
            },
            Inverting => Doc {
                plain: "The controller. It compares the measurement to a reference (the setpoint) and outputs the error — 'the fuller it gets, the less I want'.",
                everyday: "A thermostat's dial: the loop drives the stock toward the setpoint you choose.",
                math: "out = max(0, setpoint − signal)",
                substance: "information only. Raise the setpoint to hold a higher regulated level.",
                theory: "Mobus Fig 4.12 — the comparator computes reference − measured. The heart of self-regulation (setpoint defaults to 1).",
                code: "out = (setpoint - signal).max(0)",
            },
            Copying => Doc {
                plain: "Replicates a signal to everyone downstream — free of charge.",
                everyday: "Broadcasting news, sharing a file, announcing a vote.",
                math: "each out = in",
                substance: "information only. Information is the one substance that copies for free — copying matter would be counterfeiting; copying energy, perpetual motion.",
                theory: "Mobus atomic work process. NOT conservative — and that's exactly the point. The asymmetry between matter and information.",
                code: "for each output: out = signal",
            },
            Propelling => Doc {
                plain: "Pushes a flow along, losing a little to inefficiency.",
                everyday: "A pump, a courier, an economy moving goods (with friction).",
                math: "out = in · efficiency",
                substance: "moves anything.",
                theory: "Mobus atomic work process. Real transport is never perfectly efficient.",
                code: "out = inflow * efficiency",
            },
            Impeding => Doc {
                plain: "Resists a flow, holding some of it back as pressure.",
                everyday: "A bottleneck, bureaucracy, a resistor, traffic.",
                math: "out = in · (1 − impedance)",
                substance: "resists anything.",
                theory: "Mobus atomic work process. The counterpart to Propelling — every system has resistances.",
                code: "out = inflow * (1 - impedance)",
            },
        },
    }
}
