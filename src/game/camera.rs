use crate::util::{move_towards_vec3, vec3_lerp};
use bevy::{prelude::*, render::camera::WindowOrigin};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraFollow>()
            .register_type::<GameCamera>()
            .add_startup_system(camera_setup)
            .add_system_to_stage(CoreStage::PostUpdate, camera_system);
    }
}

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
                window_origin: WindowOrigin::Center,
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

    for mut camera_transform in camera_query.iter_mut() {
        let offset = Vec3::new(0.0, 0.0, 999.9);
        match follow.movement {
            FollowMovement::Instant => {
                camera_transform.translation = target.translation + offset;
            }
            FollowMovement::Linear(speed) => {
                camera_transform.translation = move_towards_vec3(
                    camera_transform.translation,
                    target.translation + offset,
                    speed * time.delta_seconds(),
                );
            }
            FollowMovement::Smooth(speed) => {
                camera_transform.translation = vec3_lerp(
                    camera_transform.translation,
                    target.translation + offset,
                    (speed * time.delta_seconds()).min(1.0),
                );
            }
        }
    }
}
