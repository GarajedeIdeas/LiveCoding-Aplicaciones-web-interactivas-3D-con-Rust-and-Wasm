use bevy::prelude::*;

pub fn create_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 1.0,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::AQUAMARINE,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }
    );
}
