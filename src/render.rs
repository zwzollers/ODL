use std::sync::{Arc, RwLock};

use eframe::{
    egui_wgpu::{self, wgpu},
    wgpu::{BufferDescriptor, util::DeviceExt},
};
use egui::{Margin, PointerButton, Pos2, Vec2};
use egui_wgpu::CallbackTrait;

use opencascade::mesh::Mesh;
use rand::prelude::*;

use crate::App;

use super::stl::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
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
                },
            ],
        }
    }
}

#[derive(PartialEq, Clone)]
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
    mouse_position: Pos2,
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
            mouse_position: Pos2 { x: 0.0, y: 0.0 },
            mouse_offset: (0.0, 0.0),
            mouse_scroll_delta: 0.0,
            is_rotating: false,
            is_translating: false,
        }
    }

    pub fn clear_clicked(&mut self) {
        self.is_rotating = false;
        self.is_translating = false;
    }

    pub fn handle_mouse_click(&mut self, code: PointerButton, is_pressed: bool) {
        self.clear_clicked();
        match code {
            PointerButton::Secondary => {
                self.is_rotating = is_pressed;
                self.mouse_offset = (0.0, 0.0);
            }
            PointerButton::Primary => {
                self.is_translating = is_pressed;
                self.mouse_offset = (0.0, 0.0);
            }
            _ => (),
        }
    }

    pub fn handle_mouse_scroll(&mut self, delta: Vec2) {
        self.mouse_scroll_delta += delta.y.clamp(-1.0, 1.0);
    }

    pub fn handle_mouse_move(&mut self, pos: Pos2) {
        let offset = Pos2::new(pos.x - self.mouse_position.x, pos.y - self.mouse_position.y);

        self.mouse_offset = (
            self.mouse_offset.0 + offset.x,
            self.mouse_offset.1 + offset.y,
        );

        self.mouse_position = pos;
    }

    pub fn update_camera(&mut self, camera: &mut Camera) -> bool {
        let prev_camera = camera.clone();

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
            camera.eye = camera.target
                - (forward + (right * self.speed * self.mouse_offset.0)).normalize() * forward_mag;
            let forward = camera.target - camera.eye;
            camera.eye = camera.target
                - (forward - (camera.up * self.speed * self.mouse_offset.1)).normalize()
                    * forward_mag;

            self.mouse_offset = (0.0, 0.0);
        }

        if self.is_translating {
            camera.target -= right * self.tspeed * self.mouse_offset.0 * forward_mag;
            camera.target += camera.up * self.tspeed * self.mouse_offset.1 * forward_mag;

            camera.eye -= right * self.tspeed * self.mouse_offset.0 * forward_mag;
            camera.eye += camera.up * self.tspeed * self.mouse_offset.1 * forward_mag;

            self.mouse_offset = (0.0, 0.0);
        }

        *camera != prev_camera
    }
}

struct ObjectViewResources {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub camera_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub depth_texture: Texture,
    pub size: [u32; 2],
}

#[derive(Clone, Default)]
pub struct ObjectView {
    camera: CameraUniform,
    camera_moved: bool,
    object: Arc<RwLock<Object>>,
    object_changed: bool,
    size: [u32; 2],
}

impl CallbackTrait for ObjectView {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        Vec::new()
    }
    fn finish_prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &mut ObjectViewResources = callback_resources.get_mut().unwrap();

        {
            let obj = self.object.read().unwrap();
            let (idx_size, vtx_size) = (
                (obj.indicies.len() * 2) as u64,
                (obj.verticies.len() * 4 * 6) as u64,
            );

            if resources.index_buffer.size() < idx_size {
                resources.index_buffer = device.create_buffer(&BufferDescriptor {
                    label: Some("Index Buffer"),
                    size: idx_size,
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
            }

            if resources.vertex_buffer.size() < vtx_size {
                resources.vertex_buffer = device.create_buffer(&BufferDescriptor {
                    label: Some("Vertex Buffer"),
                    size: vtx_size,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
            }

            if self.camera_moved {
                queue.write_buffer(
                    &resources.camera_buffer,
                    0,
                    bytemuck::cast_slice(&[self.camera]),
                );
            }

            if self.object_changed {
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
        }

        if resources.size != self.size {
            resources.size = self.size;
            resources.depth_texture =
                Texture::create_depth_texture(&device, resources.size, "depth_texture");
        }
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
            let obj = self.object.read().unwrap();
            let num_indicies = obj.indicies.len();
            render_pass.set_pipeline(&resources.pipeline);
            render_pass.set_bind_group(0, &resources.bind_group, &[]);
            render_pass.set_vertex_buffer(0, resources.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(resources.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(num_indicies as u32), 0, 0..1); // 3.
        }
    }
}

pub fn render_object_view(app: &mut App, ui: &mut egui::Ui, object_changed: bool) {
    egui::Frame::canvas(ui.style())
        .fill(ui.visuals().text_edit_bg_color())
        .corner_radius(3.0)
        .outer_margin(Margin::symmetric(0, 5))
        .show(ui, |ui| {
            let (rect, resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::all());

            if resp.hovered() {
                ui.input(|i| {
                    for event in &i.events {
                        match event {
                            egui::Event::MouseWheel { delta, .. } => {
                                app.camera_controller.handle_mouse_scroll(*delta)
                            }
                            egui::Event::PointerMoved(pos) => {
                                app.camera_controller.handle_mouse_move(*pos)
                            }
                            egui::Event::PointerButton {
                                button, pressed, ..
                            } => app.camera_controller.handle_mouse_click(*button, *pressed),
                            _ => (),
                        };
                    }
                });
            } else {
                app.camera_controller.clear_clicked();
            }

            let new_aspect = rect.width() / rect.height();

            let camera_moved = app.camera_controller.update_camera(&mut app.camera);

            let mut camera_uniform = CameraUniform::new();
            camera_uniform.update_view_proj(&app.camera);

            let object_view = ObjectView {
                camera: camera_uniform,
                camera_moved: camera_moved || app.camera.aspect != new_aspect,
                object: app.object.clone(),
                object_changed,
                size: [rect.size()[0] as u32, rect.size()[1] as u32],
            };

            app.camera.aspect = new_aspect;

            let callback = egui_wgpu::Callback::new_paint_callback(rect, object_view);

            ui.painter().add(callback);
        });
}

pub fn init_object_view<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<()> {
    let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();

    let device = &wgpu_render_state.device;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("obj"),
        source: wgpu::ShaderSource::Wgsl(include_str!("test.wgsl").into()),
    });

    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[[[0.0_f32; 4]; 4]]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

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
            resource: camera_buffer.as_entire_binding(),
        }],
    });

    let depth_texture = Texture::create_depth_texture(&device, [0, 0], "depth_texture");

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
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less, // 1.
            stencil: wgpu::StencilState::default(),     // 2.
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let vertex_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("Vertex Buffer"),
        size: 10000 * 4 * 6,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let index_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("Index Buffer"),
        size: 10000 * 2 * 3,
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    wgpu_render_state
        .renderer
        .write()
        .callback_resources
        .insert(ObjectViewResources {
            pipeline,
            bind_group,
            camera_buffer,
            vertex_buffer,
            index_buffer,
            depth_texture,
            size: [0, 0],
        });

    Some(())
}

#[derive(Default)]
pub struct Object {
    pub verticies: Vec<Vertex>,
    pub indicies: Vec<u16>,
}

impl From<STL> for Object {
    fn from(value: STL) -> Self {
        let mut verticies: Vec<Vertex> = Vec::new();
        let mut indicies: Vec<u16> = Vec::new();

        let mut rng = rand::rng();

        for tri in value.triangles {
            for point in tri.points {
                let index = match verticies.iter().position(|x| x.position == point) {
                    None => {
                        verticies.push(Vertex {
                            position: (point),
                            color: ([rng.random(), rng.random(), rng.random()]),
                        });
                        verticies.len() - 1
                    }
                    Some(index) => index,
                };

                indicies.push(index as u16);
            }
        }
        
        if indicies.len() % 2 != 0 {
            indicies.push(0);
        }

        Object {
            verticies,
            indicies,
        }
    }
}

impl From<Mesh> for Object {
    fn from(value: Mesh) -> Self {
        let mut rng = rand::rng();

        let verticies: Vec<Vertex> = value
            .vertices
            .iter()
            .map(|v| Vertex {
                position: [v.x as f32, v.y as f32, v.z as f32],
                color: ([rng.random(), rng.random(), rng.random()]),
            })
            .collect();
        let indicies: Vec<u16> = value.indices.iter().map(|i| *i as u16).collect();

        Object {
            verticies,
            indicies,
        }
    }
}

pub struct Texture {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    #[allow(unused)]
    pub fn create_depth_texture(device: &wgpu::Device, size: [u32; 2], label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: size[0].max(1),
            height: size[1].max(1),
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[Self::DEPTH_FORMAT],
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}
