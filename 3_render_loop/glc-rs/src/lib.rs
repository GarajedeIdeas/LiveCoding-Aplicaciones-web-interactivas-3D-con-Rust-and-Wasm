mod camera;
mod scene;
mod utils;

use bevy::ecs::event::Events;
use bevy::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

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
        .add_event::<camera::CameraMoveEvent>()
        .add_startup_system(scene::create_scene)
        .add_startup_system(camera::create_camera)
        .add_system(scene::animate)
        .add_system(camera::move_camera)
        .update();

        Self {
            app,
            camera_events: vec![],
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
    }

    pub fn move_camera(&mut self, rx: f32, ry: f32, z: f32) {
        self.camera_events.push(camera::CameraMoveEvent {
            rotate: Vec2::new(rx, ry),
            zoom: z,
        });
    }
}
