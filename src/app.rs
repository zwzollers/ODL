use std::{f32, sync::{Arc, RwLock}};

use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Label, Margin, Rect, Stroke, StrokeKind, Style, TextStyle, Vec2};

use super::render::*;

use super::stl::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // Example stuff:
    label: String,
    
    text_width: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    object_view: ObjectView,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub object: Arc<RwLock<Object>>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub camera:Camera,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub camera_controller: CameraController,
}

impl Default for App {
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

        let camera_controller = CameraController::new(0.02, 0.001, 0.1);

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            text_width: 0.0,
            value: 2.7,
            object_view: ObjectView::default(),
            object: Arc::new(RwLock::new(Object::from(STL::try_from_bytes(include_bytes!("../utah_teapot.stl")).unwrap()))),
            camera,
            camera_controller,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        let app = App::default();
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        init_object_view(cc, Arc::clone(&app.object));
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let width = ctx.content_rect().width();
        
        egui::SidePanel::right("obj_view")
            .default_width(width/2.0)
            .resizable(true)
            .max_width(width - 200.0)
            .show(ctx, |ui| {
            //self.text_width = ui.available_width();
            render_object_view(self, ui);
        });

        // egui::SidePanel::left("file_explorer")
        //     .default_width(width/2.0)
        //     .resizable(true)
        //     .max_width(width - 200.0)
        //     .show(ctx, |ui| {
        //     //self.text_width = ui.available_width();
        //     render_object_view(self, ui);
        // });
        
        let mut layouter = |ui: &egui::Ui, string: &dyn egui::TextBuffer, _wrap_width: f32| {
            let mut layout_job =  egui::text::LayoutJob::default();
            layout_job.append(string.as_str(), 0.0, egui::TextFormat {
                    font_id: FontId::new(25.0, FontFamily::Monospace),
                    ..Default::default()
                });
            ui.fonts_mut(|f| f.layout_job(layout_job))
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            //egui::widgets::global_theme_preference_buttons(ui);

            egui::TopBottomPanel::bottom("terminal")
                .resizable(true)
                .show(ctx, |ui| {
                    ui.add(Label::new("HELLO"));
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::Frame::default()
                    .stroke(Stroke::new(1.0, Color32::GRAY))
                    .corner_radius(3.0)
                    .inner_margin(Margin::symmetric(0, 0))
                    .show(ui, |ui| {

                        let mut scroll_style = egui::style::ScrollStyle::solid();
                        scroll_style.foreground_color = false;
                        
                        ui.style_mut().spacing.scroll = scroll_style;
                        egui::ScrollArea::both().auto_shrink([false; 2])
                            .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.label)
                                    .layouter(&mut layouter)
                                    .frame(false)
                                    .desired_width(f32::INFINITY)
                                    .code_editor()  
                            );
                        });
                });
            });
        });   
    }
}
