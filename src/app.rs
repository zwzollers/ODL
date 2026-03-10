use std::sync::{Arc, RwLock};

use super::render::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    
    text_width: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    object_view: ObjectView,

    #[serde(skip)] // This how you opt-out of serialization of a field
    object: Arc<RwLock<Object>>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub camera:Camera,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub camera_controller: CameraController,
}

impl Default for TemplateApp {
    fn default() -> Self {

        let camera = Camera {
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let camera_controller = CameraController::new(0.02, 0.002, 0.1);

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            text_width: 0.0,
            value: 2.7,
            object_view: ObjectView::default(),
            object: Arc::new(RwLock::new(Object::test())),
            camera,
            camera_controller,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        let app = TemplateApp::default();
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        init_object_View(cc, app.object.clone());
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    // Place the TextEdit inside the ScrollArea
                    
                    ui.add(
                        egui::TextEdit::multiline(&mut self.label)
                            .desired_width(ui.available_width()-self.text_width-20.0)// Allows the text to extend horizontally indefinitely
                            .code_editor() // Optional: Adds code editor styling/features
                    );
                });
            });
        });
        
        egui::SidePanel::right("obj_view").resizable(true).show(ctx, |ui| {
            self.text_width = ui.available_width();
            render_object_view(self, ui);
        });
    }
}
