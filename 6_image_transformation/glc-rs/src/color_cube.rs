use bevy::prelude::*;

use crate::image;
use crate::render::{InstanceData, InstancedMesh};

#[derive(Component)]
pub struct ColorCube {
    pub resolution: u32,
    pub threshold: f32,
}

#[derive(Clone, Debug)]
pub struct UpdateColorCubeEvent;

pub fn create_instance_data(resolution: u32) -> Vec<InstanceData> {
    let mut data = vec![];
    let step = 1.0 / (resolution - 1) as f32;

    for xi in 0..resolution {
        let x = xi as f32 * step;
        for yi in 0..resolution {
            let y = yi as f32 * step;
            for zi in 0..resolution {
                let z = zi as f32 * step;

                data.push(InstanceData {
                    position: Vec3::new(x, y, z),
                    color: Color::rgb(x, y, z).as_rgba_f32(),
                    scale: step,
                });
            }
        }
    }

    data
}

pub fn create_color_cube(
    mut commands: Commands,
    resolution: u32,
    mesh: Handle<Mesh>,
    size: f32,
    threshold: f32,
) {
    let instance_data = create_instance_data(resolution);
    commands.spawn_bundle((
        Transform {
            translation: Vec3::new(-size / 2.0, -size / 2.0, -size / 2.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(size, size, size),
        },
        GlobalTransform::identity(),
        mesh,
        InstancedMesh(instance_data),
        ColorCube {
            resolution,
            threshold,
        },
        Visibility::default(),
        ComputedVisibility::default(),
    ));
}

pub fn update_color_cube(
    mut events: EventReader<UpdateColorCubeEvent>,
    image_query: Query<&image::Image, With<image::Output>>,
    mut cube_query: Query<(&mut InstancedMesh, &ColorCube)>,
) {
    let evts = events.iter().collect::<Vec<_>>();
    if let Some(_evt) = evts.into_iter().last() {
        if let Some(image) = image_query.iter().last() {
            let num_pixels = (image.width * image.height) as f32;
            if let Some((mut mesh, cube)) = cube_query.iter_mut().last() {
                for d in mesh.0.iter_mut() {
                    d.scale = 0.0;
                }
                let r = cube.resolution as usize;
                let r2 = r * r;
                let step = 1.0 / (r - 1) as f32;
                for c in image.data.iter() {
                    let xi = ((c.r() * cube.resolution as f32).floor() as usize).clamp(0, r - 1);
                    let yi = ((c.g() * cube.resolution as f32).floor() as usize).clamp(0, r - 1);
                    let zi = ((c.b() * cube.resolution as f32).floor() as usize).clamp(0, r - 1);
                    let idx = xi * r2 + yi * r + zi;
                    mesh.0[idx].scale += step;
                }
                let threshold = cube.threshold * num_pixels;
                for d in mesh.0.iter_mut() {
                    d.scale = (d.scale / threshold).clamp(0.0, step);
                }
            }
        }
    }
}
