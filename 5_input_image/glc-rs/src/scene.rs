use bevy::prelude::*;

use crate::color_cube;
use crate::image;

const RESOLUTION: u32 = 32;
const SIZE: f32 = 10.0;
const THRESHOLD: f32 = 0.001;

pub fn create_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn_bundle((image::Image::default(),));

    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    color_cube::create_color_cube(commands, RESOLUTION, mesh, SIZE, THRESHOLD);
}
