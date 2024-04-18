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
