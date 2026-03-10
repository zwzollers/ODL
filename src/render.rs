use std::{num::NonZeroU64, sync::{Arc, RwLock}};

use bytemuck::NoUninit;
use eframe::{
    egui_wgpu::{self, wgpu}, wgpu::util::DeviceExt,
};
use egui::{PointerButton, Vec2};
use egui_wgpu::CallbackTrait;
use log::info;

use crate::TemplateApp;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Default, Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct CameraController {
    speed: f32,
    tspeed: f32,
    sspeed: f32,
    mouse_position: Vec2,
    mouse_offset: (f32, f32),
    mouse_scroll_delta: f32,
    is_rotating: bool,
    is_translating: bool,
}

impl CameraController {
    pub fn new(speed: f32, tspeed: f32, sspeed: f32) -> Self {
        Self {
            speed,
            tspeed,
            sspeed,
            mouse_position: Vec2 { x: 0.0, y: 0.0 },
            mouse_offset: (0.0, 0.0),
            mouse_scroll_delta: 0.0,
            is_rotating: false,
            is_translating: false,
        }
    }

    // fn handle_key(&mut self, code: KeyCode, is_pressed: bool) -> bool {
    //     true
    // }

    pub fn handle_mouse_click(&mut self, code: PointerButton, is_pressed: bool) {
        match code {
            PointerButton::Middle => {
                self.is_rotating = is_pressed;
                self.mouse_offset = (0.0, 0.0);
            }
            PointerButton::Primary => {
                self.is_translating = is_pressed;
                self.mouse_offset = (0.0, 0.0);
            }
            _ => ()
        }
    }
    
    pub fn handle_mouse_scroll(&mut self, delta: Vec2) {
        
            self.mouse_scroll_delta += delta.y;
    }

    pub fn handle_mouse_move(&mut self, offset: Vec2) {
        self.mouse_offset = (self.mouse_offset.0 + offset.x, self.mouse_offset.1 + offset.y);
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();
        
        camera.eye += forward_norm * self.sspeed * self.mouse_scroll_delta * forward_mag;
        
        self.mouse_scroll_delta = 0.0;

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        
        let forward_mag = forward.magnitude();

        if self.is_rotating {
            camera.eye = camera.target - (forward + (right * self.speed * self.mouse_offset.0 as f32)).normalize() * forward_mag;
            let forward = camera.target - camera.eye;
            camera.eye = camera.target - (forward - (camera.up * self.speed * self.mouse_offset.1 as f32)).normalize() * forward_mag;

            self.mouse_offset = (0.0, 0.0);
        }

        if self.is_translating {
            camera.target -= right * self.tspeed * self.mouse_offset.0 as f32 * forward_mag;
            camera.target += camera.up * self.tspeed * self.mouse_offset.1 as f32 * forward_mag;

            camera.eye -= right * self.tspeed * self.mouse_offset.0 as f32 * forward_mag;
            camera.eye += camera.up * self.tspeed * self.mouse_offset.1 as f32 * forward_mag;

            self.mouse_offset = (0.0, 0.0);
        }
    }
}

struct ObjectViewResources {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub camera_buffer: wgpu::Buffer,
    pub object: Arc<RwLock<Object>>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

#[derive(Clone, Default)]
pub struct ObjectView {
    camera: CameraUniform,
}

impl CallbackTrait for ObjectView {
    fn prepare(
            &self,
            _device: &wgpu::Device,
            queue: &wgpu::Queue,
            _screen_descriptor: &egui_wgpu::ScreenDescriptor,
            _egui_encoder: &mut wgpu::CommandEncoder,
            callback_resources: &mut egui_wgpu::CallbackResources,
        ) -> Vec<wgpu::CommandBuffer> {
        let resources: &ObjectViewResources = callback_resources.get().unwrap();
        {
            let obj = resources.object.read().unwrap();
            
            queue.write_buffer(
                &resources.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera]),
            );
            queue.write_buffer(
                &resources.vertex_buffer,
                0,
                bytemuck::cast_slice(obj.verticies.as_slice()),
            );
            queue.write_buffer(
                &resources.index_buffer,
                0,
                bytemuck::cast_slice(obj.indicies.as_slice()),
            );
        }
        
        Vec::new()
    }
    fn finish_prepare(
            &self,
            _device: &wgpu::Device,
            _queue: &wgpu::Queue,
            _egui_encoder: &mut wgpu::CommandEncoder,
            _callback_resources: &mut egui_wgpu::CallbackResources,
        ) -> Vec<wgpu::CommandBuffer> {
        Vec::new()
    }
    fn paint(
            &self,
            _info: egui::PaintCallbackInfo,
            render_pass: &mut wgpu::RenderPass<'static>,
            callback_resources: &egui_wgpu::CallbackResources,
        ) {
        let resources: &ObjectViewResources = callback_resources.get().unwrap();
        {
            let obj = resources.object.read().unwrap();
            let num_indicies = obj.indicies.len();
            render_pass.set_pipeline(&resources.pipeline);
            render_pass.set_bind_group(0, &resources.bind_group, &[]);
            render_pass.set_vertex_buffer(0, resources.vertex_buffer.slice(..));
            render_pass.set_index_buffer(resources.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(num_indicies as u32), 0, 0..1); // 3.
        }
    }
}


pub fn render_object_view (app: &mut TemplateApp, ui: &mut egui::Ui) {
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        let (rect, resp) = ui.allocate_at_least(ui.available_size(), egui::Sense::all());

        if resp.hovered() {
            ui.input(|i| {
                for event in &i.events {
                    info!("{event:?}");
                    match event { 
                        egui::Event::MouseWheel { delta, ..} => app.camera_controller.handle_mouse_scroll(*delta),
                        egui::Event::MouseMoved(pos) => app.camera_controller.handle_mouse_move(*pos),
                        egui::Event::MouseMoved(pos) => app.camera_controller.handle_mouse_move(*pos),
                        egui::Event::PointerButton { button, pressed, ..} => app.camera_controller.handle_mouse_click(*button, *pressed),
                        _ => ()
                    };
                }
            });
        }

        app.camera_controller.update_camera(&mut app.camera);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&app.camera);

        let object_view = ObjectView {
            camera: camera_uniform,
        };

        let callback = egui_wgpu::Callback::new_paint_callback(rect, object_view);
        
        ui.painter().add(callback);
    });
}

pub fn init_object_View<'a>(cc: &'a eframe::CreationContext<'a>, object: Arc<RwLock<Object>>) -> Option<()> {
    let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();

    let device = &wgpu_render_state.device;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("obj"),
        source: wgpu::ShaderSource::Wgsl(include_str!("test.wgsl").into()),
    });

    // let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("custom3d"),
    //     contents: bytemuck::cast_slice(&[0.0_f32; 4]), // 16 bytes aligned!
    //     usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
    // });

    let camera_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[[[0.0_f32; 4]; 4]]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    );


    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("obj"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("obj"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding()
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("custom3d"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("custom3d"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc()],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu_render_state.target_format.into())],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None
    });

    let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        }
    );

    
    wgpu_render_state
        .renderer
        .write()
        .callback_resources
        .insert(ObjectViewResources {
            pipeline,
            bind_group,
            camera_buffer,
            object,
            vertex_buffer,
            index_buffer,
    });

    Some(())

}

#[derive(Default)]
pub struct Object {
    verticies: Vec<Vertex>,
    indicies: Vec<u16>
}

impl Object {
    pub fn test() -> Self {
        Self {
            verticies: vec![
                Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [1.5, 0.0, 0.5] }, // A
                Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
                Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
                Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
                Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
            ],
            indicies: vec![
                0, 1, 4,
                1, 2, 4,
                2, 3, 4, 0
            ]
        }
    }
}