use bevy::prelude::*;

pub fn create_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(
        PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()
        }
    );
}
