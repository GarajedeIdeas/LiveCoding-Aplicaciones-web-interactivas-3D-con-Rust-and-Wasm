use bevy::prelude::*;

use crate::color_cube;

const RESOLUTION: u32 = 32;
const SIZE: f32 = 10.0;

pub fn create_scene(
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    color_cube::create_color_cube(commands, RESOLUTION, mesh, SIZE);
}

