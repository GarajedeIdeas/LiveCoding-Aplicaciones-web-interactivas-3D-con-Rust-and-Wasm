use bevy::prelude::*;

use crate::color_cube;

#[derive(Component)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

#[derive(Clone, Debug)]
pub struct SetInputImageEvent {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Default for Image {
    fn default() -> Self {
        Image {
            width: 0,
            height: 0,
            data: vec![],
        }
    }
}

pub fn set_input_image(
    mut events: EventReader<SetInputImageEvent>,
    mut out_events: EventWriter<color_cube::UpdateColorCubeEvent>,
    mut query: Query<&mut Image>,
) {
    let evts = events.iter().collect::<Vec<_>>();
    if let Some(evt) = evts.into_iter().last() {
        if let Some(mut image) = query.iter_mut().last() {
            image.width = evt.width;
            image.height = evt.height;
            image.data = evt.data.clone();
            out_events.send(color_cube::UpdateColorCubeEvent);
        }
    }
}
