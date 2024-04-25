use crate::components::FlowCurve;
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct OutflowInterfaceConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct InflowInterfaceConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct InflowSourceConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct OutflowSinkConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct InterfaceSubsystemConnection {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct SubsystemParentFlowConnection {
    pub target: Entity,
}

pub trait Connection {
    fn target(&self) -> Entity;
}

macro_rules! impl_connection {
    ($name:ident) => {
        impl Connection for $name {
            #[inline(always)]
            fn target(&self) -> Entity {
                self.target
            }
        }
    };
}

impl_connection!(OutflowInterfaceConnection);
impl_connection!(InflowInterfaceConnection);
impl_connection!(InflowSourceConnection);
impl_connection!(OutflowSinkConnection);
impl_connection!(InterfaceSubsystemConnection);
impl_connection!(SubsystemParentFlowConnection);

pub trait FlowConnection {
    fn apply_delta_to_flow(&self, flow: &mut FlowCurve, delta: Vec2);
}

impl FlowConnection for OutflowSinkConnection {
    fn apply_delta_to_flow(&self, flow: &mut FlowCurve, delta: Vec2) {
        flow.end += delta;
    }
}

impl FlowConnection for InflowSourceConnection {
    fn apply_delta_to_flow(&self, flow: &mut FlowCurve, delta: Vec2) {
        flow.start += delta;
    }
}

impl FlowConnection for OutflowInterfaceConnection {
    fn apply_delta_to_flow(&self, flow: &mut FlowCurve, delta: Vec2) {
        flow.start += delta;
    }
}

impl FlowConnection for InflowInterfaceConnection {
    fn apply_delta_to_flow(&self, flow: &mut FlowCurve, delta: Vec2) {
        flow.end += delta;
    }
}
