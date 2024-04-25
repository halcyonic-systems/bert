use bevy::ecs::system::BoxedSystem;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Component)]
pub struct SpawnOnSelected {
    on_selected: SpawnSystem,
    spawned_entities: Vec<Entity>,
}

impl SpawnOnSelected {
    pub fn new<Marker>(on_selected: impl IntoSystem<(), (), Marker>) -> Self {
        Self {
            on_selected: SpawnSystem::New(Box::new(IntoSystem::into_system(on_selected))),
            spawned_entities: Vec::new(),
        }
    }

    pub fn take(&mut self) -> SpawnSystem {
        std::mem::take(&mut self.on_selected)
    }
}

#[derive(Default)]
pub enum SpawnSystem {
    #[default]
    Empty,
    New(BoxedSystem),
    Initialized(BoxedSystem),
}

impl SpawnSystem {
    pub fn run(&mut self, world: &mut World) {
        let mut system = match std::mem::take(self) {
            SpawnSystem::Empty => return,
            SpawnSystem::New(mut system) => {
                system.initialize(world);
                system
            }
            SpawnSystem::Initialized(system) => system,
        };

        system.run((), world);
        system.apply_deferred(world);
        *self = SpawnSystem::Initialized(system);
    }
}

pub fn selection_changed(
    query: Query<&PickSelection, (With<SpawnOnSelected>, Changed<PickSelection>)>,
) -> bool {
    !query.is_empty()
}

pub fn spawn_on_selected(
    world: &mut World,
    query: &mut QueryState<(Entity, &PickSelection, &mut SpawnOnSelected)>,
) {
    let mut query_results = vec![];

    for (entity, selection, mut spawn_on_selected) in query.iter_mut(world) {
        query_results.push((
            entity,
            selection.clone(),
            spawn_on_selected.take(),
            spawn_on_selected.spawned_entities.clone(),
        ));
    }

    for (entity, selection, on_selected, spawned) in &mut query_results {
        world.insert_resource(SelectedSpawnInput {
            selected: *entity,
            spawned: vec![],
        });

        if selection.is_selected {
            on_selected.run(world);

            spawned.extend(world.resource::<SelectedSpawnInput>().spawned.clone());
        } else {
            for entity in spawned.drain(..) {
                world.despawn(entity);
            }
        }

        world.remove_resource::<SelectedSpawnInput>();
    }

    for ((_, _, mut spawn_on_selected), (_, _, on_selected, spawned)) in
        query.iter_mut(world).zip(query_results.drain(..))
    {
        spawn_on_selected.on_selected = on_selected;
        spawn_on_selected.spawned_entities = spawned;
    }
}

#[derive(Clone, PartialEq, Debug, Resource)]
pub struct SelectedSpawnInput {
    selected: Entity,
    spawned: Vec<Entity>,
}

impl SelectedSpawnInput {
    pub fn selected(&self) -> Entity {
        self.selected
    }

    pub fn add_spawned(&mut self, entity: Entity) {
        self.spawned.push(entity);
    }
}

pub type SelectedSpawnListener<'w> = ResMut<'w, SelectedSpawnInput>;
