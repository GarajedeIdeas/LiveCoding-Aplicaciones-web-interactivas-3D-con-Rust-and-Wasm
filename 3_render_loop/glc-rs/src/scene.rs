use bevy::prelude::*;

const N: u32 = 5;

pub fn create_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    let step = 1.0 / N as f32;
    commands
        .spawn_bundle((
            Transform::from_xyz(-0.5, -0.5, -0.5),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            for xi in 0..N {
                let x = xi as f32 * step;
                for yi in 0..N {
                    let y = yi as f32 * step;
                    for zi in 0..N {
                        let z = zi as f32 * step;

                        let material = materials.add(StandardMaterial {
                            base_color: Color::rgb(x, y, z),
                            ..Default::default()
                        });

                        parent.spawn_bundle(PbrBundle {
                            mesh: mesh.clone(),
                            material,
                            transform: Transform::from_xyz(x, y, z),
                            ..Default::default()
                        });
                    }
                }
            }
        });
}

pub fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<Handle<Mesh>>>) {
    let t = time.seconds_since_startup() as f32;
    for mut xform in query.iter_mut() {
        let offset = xform.translation.x + 2.0 * xform.translation.y + 3.0 * xform.translation.z;
        let s = 0.5 * 0.5 * (t + offset).sin();
        xform.scale = Vec3::new(s, s, s);
    }
}
