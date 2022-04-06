use bevy::prelude::*;

use crate::color_cube;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

#[derive(Component)]
pub struct Image {
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

#[derive(Component)]
pub struct Input;

#[derive(Component)]
pub struct Output {
    pub canvas_id: Option<String>,
}

impl Default for Output {
    fn default() -> Self {
        Output { canvas_id: None }
    }
}

#[derive(Component)]
pub struct ColorTransformation {
    pub rotation: Quat,
}

impl Default for ColorTransformation {
    fn default() -> Self {
        ColorTransformation {
            rotation: Quat::IDENTITY,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SetInputImageEvent {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

#[derive(Clone, Debug)]
pub struct TransformImageEvent;

#[derive(Clone, Debug)]
pub struct SetColorTransformationEvent {
    pub rotation: Quat,
}

#[derive(Clone, Debug)]
pub struct SetOutputCanvasEvent {
    pub canvas_id: String,
}

#[derive(Clone, Debug)]
pub struct RenderRequest;

pub fn set_input_image(
    mut events: EventReader<SetInputImageEvent>,
    mut out_image_events: EventWriter<TransformImageEvent>,
    mut query: Query<&mut Image, With<Input>>,
) {
    let evts = events.iter().collect::<Vec<_>>();
    if let Some(evt) = evts.into_iter().last() {
        if let Some(mut image) = query.iter_mut().last() {
            image.width = evt.width;
            image.height = evt.height;
            image.data = evt.data.clone();
            out_image_events.send(TransformImageEvent);
        }
    }
}

pub fn set_output_canvas(
    mut events: EventReader<SetOutputCanvasEvent>,
    mut out_events: EventWriter<RenderRequest>,
    mut query: Query<&mut Output>,
) {
    let evts = events.iter().collect::<Vec<_>>();
    if let Some(evt) = evts.into_iter().last() {
        if let Some(mut output) = query.iter_mut().last() {
            output.canvas_id = Some(evt.canvas_id.clone());
            out_events.send(RenderRequest);
        }
    }
}

pub fn set_color_transformation(
    mut events: EventReader<SetColorTransformationEvent>,
    mut out_events: EventWriter<TransformImageEvent>,
    mut query: Query<&mut ColorTransformation>,
) {
    let evts = events.iter().collect::<Vec<_>>();
    if let Some(evt) = evts.into_iter().last() {
        if let Some(mut xform) = query.iter_mut().last() {
            xform.rotation = evt.rotation;
            out_events.send(TransformImageEvent);
        }
    }
}

pub fn transform_image(
    mut events: EventReader<TransformImageEvent>,
    input_query: Query<&Image, (With<Input>, Without<Output>)>,
    mut output_query: Query<(&mut Image, &ColorTransformation), With<Output>>,
    mut out_cube_events: EventWriter<color_cube::UpdateColorCubeEvent>,
    mut out_render_events: EventWriter<RenderRequest>,
) {
    let evts = events.iter().collect::<Vec<_>>();
    if let Some(_evt) = evts.into_iter().last() {
        if let Some(input) = input_query.iter().last() {
            if let Some((mut output, xform)) = output_query.iter_mut().last() {
                output.width = input.width;
                output.height = input.height;
                output.data = input
                    .data
                    .iter()
                    .map(|c| {
                        let p = Vec3::new(c.r(), c.g(), c.b());
                        let p = xform.rotation.mul_vec3(p - Vec3::new(0.5, 0.5, 0.5))
                            + Vec3::new(0.5, 0.5, 0.5);
                        Color::rgba(p.x, p.y, p.z, c.a())
                    })
                    .collect();
                out_cube_events.send(color_cube::UpdateColorCubeEvent);
                out_render_events.send(RenderRequest);
            }
        }
    }
}

pub fn render_image(
    mut events: EventReader<RenderRequest>,
    query: Query<(&Image, &Output)>,
) {
    let evts = events.iter().collect::<Vec<_>>();
    if let Some(_evt) = evts.into_iter().last() {
        if let Some((image, output)) = query.iter().last() {
            if output.canvas_id.is_none() || image.width == 0 || image.height == 0 {
                return;
            }
            let canvas_id = output.canvas_id.clone().unwrap();

            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = document.get_element_by_id(&canvas_id).unwrap();
            let canvas: web_sys::HtmlCanvasElement = canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            let src_width = image.width;
            let src_height = image.height;
            let ratio = src_width as f64 / src_height as f64;
            let dst_width = canvas.width();
            let dst_height = (dst_width as f64 / ratio) as u32;

            canvas.set_width(src_width);
            canvas.set_height(src_height);

            let data = image
                .data
                .iter()
                .flat_map(|c| c.as_rgba_f32().into_iter())
                .map(|x| (x * 255.0).floor() as u8)
                .collect::<Vec<_>>();
            let clamped_data = Clamped(&data[..]);

            let image_data =
                ImageData::new_with_u8_clamped_array(clamped_data, src_width).unwrap();

            context.put_image_data(&image_data, 0.0, 0.0).unwrap();
            context
                .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &canvas,
                    0.0,
                    0.0,
                    src_width as f64,
                    src_height as f64,
                    0.0,
                    0.0,
                    dst_width as f64,
                    dst_height as f64,
                )
                .unwrap();

            let image_data = context
                .get_image_data(0.0, 0.0, dst_width as f64, dst_height as f64)
                .unwrap();

            canvas.set_width(dst_width);
            canvas.set_height(dst_height);
            context.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    }
}
