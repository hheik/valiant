use crate::util::{move_towards_vec3, vec3_lerp};
use bevy::{input::mouse::MouseMotion, prelude::*, transform::TransformSystem};
use bevy_ecs_ldtk::prelude::*;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            CameraSystem
                .after(CoreSet::UpdateFlush)
                .before(TransformSystem::TransformPropagate),
        )
        .register_type::<CameraFollow>()
        .register_type::<GameCamera>()
        .add_startup_system(camera_setup)
        .add_systems(
            (camera_mode_system, room_restraint)
                .chain()
                .in_base_set(CameraSystem),
        );
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
pub struct GameCamera {
    pub mode: CameraMode,
}

#[derive(Default, Reflect)]
pub enum CameraMode {
    #[default]
    Free,
    Follow,
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct CameraFollow {
    pub priority: i32,
    pub movement: FollowMovement,
}

#[derive(Default, Component)]
pub struct CameraRoomRestraint;

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
        GameCamera::default(),
        CameraRoomRestraint,
    ));
}

fn camera_mode_system(
    time: Res<Time>,
    mut camera_query: Query<
        (&mut Transform, &mut GameCamera, &OrthographicProjection),
        With<Camera2d>,
    >,
    follow_query: Query<(&Transform, &CameraFollow), Without<Camera2d>>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_events: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
) {
    let raw_mouse_motion: Vec2 = mouse_events.iter().map(|e| e.delta).sum();

    for (mut camera_transform, mut camera, projection) in camera_query.iter_mut() {
        if key_input.just_pressed(KeyCode::Tab) {
            camera.mode = CameraMode::Follow
        }

        match camera.mode {
            CameraMode::Free => {
                let mouse_motion = raw_mouse_motion * projection.scale * Vec2::new(-1.0, 1.0);
                if mouse_input.pressed(MouseButton::Middle) {
                    input_movement(&mut camera_transform, mouse_motion)
                }
            }
            CameraMode::Follow => {
                if mouse_input.pressed(MouseButton::Middle)
                    && raw_mouse_motion.abs_diff_eq(Vec2::ZERO, 0.001)
                {
                    camera.mode = CameraMode::Free
                }

                let (target, follow) = match follow_query
                    .iter()
                    .max_by_key(|(_transform, follow)| follow.priority)
                {
                    Some((target, follow)) => (target, follow),
                    None => return,
                };

                let target = Vec3 {
                    z: 999.9,
                    ..target.translation
                };

                follow_movement(
                    &mut camera_transform,
                    &follow.movement,
                    target,
                    time.delta_seconds(),
                )
            }
        }
    }
}

fn room_restraint(
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), With<CameraRoomRestraint>>,
    level_query: Query<(&GlobalTransform, &Handle<LdtkLevel>), Without<OrthographicProjection>>,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (mut camera_transform, projection) in camera_query.iter_mut() {
        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let top = camera_transform.translation.y + projection.area.max.y;
                    let bottom = camera_transform.translation.y + projection.area.min.y;
                    let left = camera_transform.translation.x + projection.area.min.x;
                    let right = camera_transform.translation.x + projection.area.max.x;

                    let top_limit = level_transform.translation().y + level.px_hei as f32;
                    let bottom_limit = level_transform.translation().y;
                    let left_limit = level_transform.translation().x;
                    let right_limit = level_transform.translation().x + level.px_wid as f32;

                    let top_move = (top_limit - top).min(0.0);
                    let bottom_move = (bottom_limit - bottom).max(0.0);
                    let left_move = (left_limit - left).max(0.0);
                    let right_move = (right_limit - right).min(0.0);

                    camera_transform.translation.x += left_move + right_move;
                    camera_transform.translation.y += top_move + bottom_move;
                }
            }
        }
    }
}

fn follow_movement(
    transform: &mut Transform,
    movement: &FollowMovement,
    target: Vec3,
    delta_time: f32,
) {
    match movement {
        FollowMovement::Instant => {
            transform.translation = target;
        }
        FollowMovement::Linear(speed) => {
            transform.translation =
                move_towards_vec3(transform.translation, target, speed * delta_time);
        }
        FollowMovement::Smooth(speed) => {
            transform.translation =
                vec3_lerp(transform.translation, target, (speed * delta_time).min(1.0));
        }
    }
}

fn input_movement(transform: &mut Transform, movement: Vec2) {
    transform.translation += movement.extend(0.0);
}
