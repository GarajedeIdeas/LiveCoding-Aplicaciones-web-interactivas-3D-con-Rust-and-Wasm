use bevy::prelude::*;

use crate::render::{InstanceData, InstancedMesh};

#[derive(Component)]
pub struct ColorCube {
    pub resolution: u32,
}

pub fn create_instance_data(
    resolution: u32,
) -> Vec<InstanceData> {
    let mut data = vec![];
    let scale = 1.0 / resolution as f32;

    for xi in 0..resolution {
        let x = xi as f32 * scale as f32;
        for yi in 0..resolution {
            let y = yi as f32 * scale as f32;
            for zi in 0..resolution {
                let z = zi as f32 * scale as f32;

                data.push(InstanceData {
                    position: Vec3::new(x, y, z),
                    color: Color::rgb(x, y, z).as_rgba_f32(),
                    scale
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
            resolution
        },
        Visibility::default(),
        ComputedVisibility::default(),
    ));
}
