mod camera;
mod scene;
mod utils;

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
                brightness: 0.1,
                color: Color::WHITE,
            })
            .add_plugins(DefaultPlugins)
            .add_startup_system(scene::create_scene)
            .add_startup_system(camera::create_camera)
            .update();
        
        Self {
            app
        }
    }

}
