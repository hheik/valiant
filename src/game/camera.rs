use crate::util::{move_towards_vec3, vec3_lerp};
use bevy::prelude::*;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            CameraSystem
                .after(CoreSet::PostUpdate)
                .before(CoreSet::PostUpdateFlush),
        )
        .register_type::<CameraFollow>()
        .register_type::<GameCamera>()
        .add_startup_system(camera_setup)
        .add_system(camera_system.in_base_set(CameraSystem));
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, SystemSet)]
#[system_set(base)]
pub struct CameraSystem;

#[derive(Clone, Copy, PartialEq, Reflect)]
pub enum FollowMovement {
    Instant,
    Linear(f32),
    Smooth(f32),
}

impl Default for FollowMovement {
    fn default() -> Self {
        Self::Instant
    }
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct GameCamera;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct CameraFollow {
    pub priority: i32,
    pub movement: FollowMovement,
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Game Camera"),
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.25,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::rgb(0.0, 0.0, 0.0),
                ),
            },
            ..default()
        },
        GameCamera,
    ));
}

fn camera_system(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    follow_query: Query<(&Transform, &CameraFollow), Without<Camera2d>>,
) {
    let (target, follow) = match follow_query
        .iter()
        .max_by_key(|(_transform, follow)| follow.priority)
    {
        Some(followed) => followed,
        None => return,
    };

    let target = Vec3 {
        z: 999.9,
        ..target.translation
    };

    for mut camera_transform in camera_query.iter_mut() {
        match follow.movement {
            FollowMovement::Instant => {
                camera_transform.translation = target;
            }
            FollowMovement::Linear(speed) => {
                camera_transform.translation = move_towards_vec3(
                    camera_transform.translation,
                    target,
                    speed * time.delta_seconds(),
                );
            }
            FollowMovement::Smooth(speed) => {
                camera_transform.translation = vec3_lerp(
                    camera_transform.translation,
                    target,
                    (speed * time.delta_seconds()).min(1.0),
                );
            }
        }
    }
}
