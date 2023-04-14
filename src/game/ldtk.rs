use bevy::ecs::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_ldtk::prelude::*;

pub struct LdtkHelperPlugin;

impl Plugin for LdtkHelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: false,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..default()
            })
            .add_event::<EntityInstanceAdded>()
            .register_ldtk_int_cell::<WallBundle>(1)
            .insert_resource(WordlyInstances::default())
            .add_system(entity_instance_events)
            .add_system(entity_namer.in_base_set(CoreSet::PostUpdate))
            .add_system(unique_handler.in_base_set(CoreSet::PostUpdate));
    }
}

pub struct EntityInstanceAdded {
    pub entity: Entity,
    pub instance: EntityInstance,
}

#[derive(Resource, Default)]
pub struct WordlyInstances {
    pub def_uid_map: HashMap<i32, Entity>,
}

#[derive(Bundle, LdtkIntCell, Default, Clone, Debug)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct Wall;

fn entity_instance_events(
    mut q2: EventReader<bevy_ecs_ldtk::LevelEvent>,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    worldly_instances: Res<WordlyInstances>,
    mut events: EventWriter<EntityInstanceAdded>,
    mut commands: Commands,
) {
    for event in q2.iter() {
        info!("{event:?}");
    }
    for (entity, instance) in query.iter() {
        // Spawn the entity if it's not in the unique instances list (or if the old one is deleted)
        // TODO: Detect deleted entities safely: https://github.com/bevyengine/bevy/issues/3845
        if worldly_instances
            .def_uid_map
            .get(&instance.def_uid)
            .map_or(true, |ent| commands.get_entity(*ent).is_none())
        {
            events.send(EntityInstanceAdded {
                entity,
                instance: instance.clone(),
            });
        }
    }
}

fn entity_namer(
    mut commands: Commands,
    mut events: EventReader<EntityInstanceAdded>,
    nameless_query: Query<(), Without<Name>>,
) {
    for event in events.iter() {
        if nameless_query.contains(event.entity) {
            commands
                .entity(event.entity)
                .insert(Name::new(event.instance.identifier.clone()));
        }
    }
}

fn unique_handler(
    query: Query<(Entity, &EntityInstance), Added<Worldly>>,
    mut worldly_instances: ResMut<WordlyInstances>,
) {
    for (entity, instance) in query.iter() {
        worldly_instances
            .def_uid_map
            .insert(instance.def_uid, entity);
    }
}
