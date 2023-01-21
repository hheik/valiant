use bevy::prelude::*;

use crate::util::Vector2I;

use super::position::Position;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actor>()
            .insert_resource(ActorQueue::default())
            .add_system(register_actor)
            .add_system(actor_system);
    }
}

#[derive(Resource, Default, Debug)]
pub struct ActorQueue {
    pub registered: Vec<Entity>,
    pub current_index: usize,
}

impl ActorQueue {
    pub fn current_entity(&self) -> Option<Entity> {
        self.registered.get(self.current_index).copied()
    }

    pub fn next(&mut self) -> Option<Entity> {
        self.current_index = (self.current_index + 1) % self.registered.len();
        self.current_entity()
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Actor;

fn actor_system(
    mut query: Query<&mut Position, With<Actor>>,
    mut queue: ResMut<ActorQueue>,
    input: Res<Input<KeyCode>>,
) {
    let current = queue.current_entity().map(|curr| curr.clone());
    let current_index = queue.current_index;
    if let Some(current) = current {
        if let Ok(mut position) = query.get_mut(current) {
            let mut movement = None;
            if input.just_pressed(KeyCode::Up) {
                movement = Some(Vector2I::UP);
            } else if input.just_pressed(KeyCode::Down) {
                movement = Some(Vector2I::DOWN);
            } else if input.just_pressed(KeyCode::Left) {
                movement = Some(Vector2I::LEFT);
            } else if input.just_pressed(KeyCode::Right) {
                movement = Some(Vector2I::RIGHT);
            }

            if let Some(movement) = movement {
                position.0 = position.0 + movement;
                queue.next();
            }
        } else {
            queue.registered.remove(current_index);
        }
    } else {
        queue.next();
    }
}

fn register_actor(new_actors: Query<Entity, Added<Actor>>, mut queue: ResMut<ActorQueue>) {
    for entity in new_actors.iter() {
        queue.registered.push(entity);
    }
}
