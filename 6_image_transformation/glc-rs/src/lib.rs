mod camera;
mod color_cube;
mod image;
mod render;
mod scene;
mod utils;

use bevy::ecs::event::Events;
use bevy::prelude::*;
use itertools::Itertools;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::ImageData;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    utils::log("Initializing glc-rs");
}

#[wasm_bindgen]
pub struct Glc {
    app: App,
    camera_events: Vec<camera::CameraMoveEvent>,
    image_events: Vec<image::SetInputImageEvent>,
    xform_events: Vec<image::SetColorTransformationEvent>,
    output_events: Vec<image::SetOutputCanvasEvent>,
}

#[wasm_bindgen]
impl Glc {
    pub fn new(canvas_id: &str) -> Self {
        utils::log(&format!("Creating new GLC app with canvas id: {canvas_id}"));

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

        let mut app = App::new();
        app.insert_resource(WindowDescriptor {
            #[cfg(target_arch = "wasm32")]
            canvas: Some(format!("#{canvas_id}")),
            decorations: false,
            width: canvas.width() as f32,
            height: canvas.height() as f32,
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            brightness: 0.5,
            color: Color::WHITE,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(render::GlcRenderingPlugin)
        .add_event::<camera::CameraMoveEvent>()
        .add_event::<image::SetInputImageEvent>()
        .add_event::<image::SetColorTransformationEvent>()
        .add_event::<image::SetOutputCanvasEvent>()
        .add_event::<color_cube::UpdateColorCubeEvent>()
        .add_event::<image::TransformImageEvent>()
        .add_event::<image::RenderRequest>()
        .add_startup_system(scene::create_scene)
        .add_startup_system(camera::create_camera)
        .add_system(camera::move_camera)
        .add_system(image::set_input_image)
        .add_system(image::set_color_transformation)
        .add_system(image::set_output_canvas)
        .add_system(color_cube::update_color_cube)
        .add_system(image::transform_image)
        .add_system(image::render_image)
        .update();

        Self {
            app,
            camera_events: vec![],
            image_events: vec![],
            xform_events: vec![],
            output_events: vec![],
        }
    }

    pub fn update(&mut self) {
        self.app.update();

        let mut events = self
            .app
            .world
            .get_resource_mut::<Events<camera::CameraMoveEvent>>()
            .unwrap();
        for evt in self.camera_events.iter() {
            events.send(evt.clone());
        }
        self.camera_events.clear();

        let mut events = self
            .app
            .world
            .get_resource_mut::<Events<image::SetInputImageEvent>>()
            .unwrap();
        for evt in self.image_events.iter() {
            events.send(evt.clone());
        }
        self.image_events.clear();

        let mut events = self
            .app
            .world
            .get_resource_mut::<Events<image::SetColorTransformationEvent>>()
            .unwrap();
        for evt in self.xform_events.iter() {
            events.send(evt.clone());
        }
        self.xform_events.clear();

        let mut events = self
            .app
            .world
            .get_resource_mut::<Events<image::SetOutputCanvasEvent>>()
            .unwrap();
        for evt in self.output_events.iter() {
            events.send(evt.clone());
        }
        self.output_events.clear();
    }

    pub fn move_camera(&mut self, rx: f32, ry: f32, z: f32) {
        self.camera_events.push(camera::CameraMoveEvent {
            rotate: Vec2::new(rx, ry),
            zoom: z,
        });
    }

    pub fn set_input_image(&mut self, image_data: ImageData) {
        self.image_events.push(image::SetInputImageEvent {
            width: image_data.width(),
            height: image_data.height(),
            data: image_data
                .data()
                .0
                .iter()
                .map(|&x| x as f32 / 255.0)
                .chunks(4)
                .into_iter()
                .map(|mut chunk| {
                    Color::rgba(
                        chunk.next().unwrap(),
                        chunk.next().unwrap(),
                        chunk.next().unwrap(),
                        chunk.next().unwrap(),
                    )
                })
                .collect(),
        });
    }

    pub fn set_output_canvas(&mut self, canvas_id: &str) {
        self.output_events.push(image::SetOutputCanvasEvent {
            canvas_id: canvas_id.to_string(),
        })
    }

    pub fn rotate(&mut self, r: f32) {
        self.xform_events.push(image::SetColorTransformationEvent {
            rotation: Quat::from_axis_angle(Vec3::Y, r.to_radians()),
        });
    }
}
