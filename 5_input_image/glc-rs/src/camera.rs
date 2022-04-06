use bevy::prelude::*;

#[derive(Component)]
pub struct CameraRoot;

#[derive(Component)]
pub struct CameraPivot;

#[derive(Clone, Debug)]
pub struct CameraMoveEvent {
    pub rotate: Vec2,
    pub zoom: f32,
}

pub fn create_camera(mut commands: Commands) {
    commands
        .spawn_bundle((
            Transform {
                translation: Vec3::ZERO,
                rotation: Quat::from_axis_angle(Vec3::Y, 20f32.to_radians()),
                scale: Vec3::ONE,
            },
            GlobalTransform::identity(),
            CameraRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle((
                    Transform {
                        translation: Vec3::ZERO,
                        rotation: Quat::from_axis_angle(Vec3::X, 10f32.to_radians()),
                        scale: Vec3::ONE,
                    },
                    GlobalTransform::identity(),
                    CameraPivot,
                ))
                .with_children(|parent| {
                    parent.spawn_bundle(PerspectiveCameraBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 20.0),
                        ..Default::default()
                    });
                });
        });
}

pub fn move_camera(
    mut events: EventReader<CameraMoveEvent>,
    mut query: QuerySet<(
        QueryState<&mut Transform, With<CameraRoot>>,
        QueryState<&mut Transform, With<CameraPivot>>,
        QueryState<&mut Transform, With<Camera>>,
    )>,
) {
    let evts = events.iter().collect::<Vec<_>>();
    if let Some(evt) = evts.into_iter().last() {
        for mut t in query.q0().iter_mut() {
            t.rotate(Quat::from_axis_angle(Vec3::Y, -evt.rotate.x));
        }

        for mut t in query.q1().iter_mut() {
            t.rotate(Quat::from_axis_angle(Vec3::X, -evt.rotate.y));
        }

        for mut t in query.q2().iter_mut() {
            t.translation += Vec3::new(0.0, 0.0, evt.zoom);
        }
    }
}
