use std::{collections::HashSet, mem::offset_of, sync::Arc, time::Instant};

use bytemuck::{NoUninit, cast_slice};
use ggmath::{Affine2, Vec2, Vec2U, Vec3, Vec3U};
use pollster::block_on;
use wgpu::{
    Buffer, BufferUsages, Color, ColorTargetState, ColorWrites, CurrentSurfaceTexture, Device,
    DeviceDescriptor, FragmentState, FrontFace, Instance, LoadOp, MultisampleState, Operations,
    PipelineCompilationOptions, PolygonMode, PrimitiveState, PrimitiveTopology, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration, TextureViewDescriptor,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode, include_wgsl,
    wgt::{BufferDescriptor, CommandEncoderDescriptor},
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::Window,
};

pub use winit::keyboard::KeyCode;

const MAX_TRIANGLES: usize = 1000;
const TIME_STEP: f32 = 0.01;
const DEFAULT_BACKGROUND_COLOR: Vec3<f32> = Vec3::new(0.05, 0.05, 0.2);

pub fn run(update: impl FnMut(&mut Context)) {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    let mut runner = Runner {
        resources: None,
        held_keys: HashSet::new(),
        previously_held_keys: HashSet::new(),
        update,
        last_instant: None,
        time_accumulator: 0.0,
        triangles: Vec::with_capacity(MAX_TRIANGLES),
        camera_position: Vec2::ZERO,
        camera_height: 16.0,
        background_color: DEFAULT_BACKGROUND_COLOR,
    };
    event_loop.run_app(&mut runner).expect("failed to run app");
}

pub struct Context<'a> {
    aspect_ratio: f32,
    held_keys: &'a HashSet<KeyCode>,
    previously_held_keys: &'a HashSet<KeyCode>,
    camera_position: &'a mut Vec2<f32>,
    camera_height: &'a mut f32,
    background_color: &'a mut Vec3<f32>,
    triangles: &'a mut Vec<Triangle>,
    world_to_screen: Affine2<f32>,
}

impl<'a> Context<'a> {
    pub fn key_held(&self, keycode: KeyCode) -> bool {
        self.held_keys.contains(&keycode)
    }

    pub fn key_pressed(&self, keycode: KeyCode) -> bool {
        self.held_keys.contains(&keycode) && !self.previously_held_keys.contains(&keycode)
    }

    pub fn set_camera_position(&mut self, position: Vec2<f32>) {
        *self.camera_position = position;
        self.world_to_screen = Affine2::from_scale(Vec2::new(
            2.0 / *self.camera_height / self.aspect_ratio,
            2.0 / *self.camera_height,
        )) * Affine2::from_translation(-position);
    }

    pub fn set_camera_height(&mut self, height: f32) {
        *self.camera_height = height;
        self.world_to_screen =
            Affine2::from_scale(Vec2::new(2.0 / height / self.aspect_ratio, 2.0 / height))
                * Affine2::from_translation(-*self.camera_position);
    }

    pub fn set_background_color(&mut self, color: Vec3<f32>) {
        *self.background_color = color;
    }

    pub fn draw_rectangle(
        &mut self,
        color: Vec3<f32>,
        extents: Vec2<f32>,
        center: Vec2<f32>,
        angle: f32,
    ) {
        let local_to_screen =
            self.world_to_screen * Affine2::from_scale_angle_translation(extents, angle, center);

        let top_right = local_to_screen.transform_point(Vec2::new(1.0, 1.0));
        let top_left = local_to_screen.transform_point(Vec2::new(-1.0, 1.0));
        let bottom_right = local_to_screen.transform_point(Vec2::new(1.0, -1.0));
        let bottom_left = local_to_screen.transform_point(Vec2::new(-1.0, -1.0));

        self.triangles.extend([
            Triangle {
                vertex_1: top_right.unalign(),
                vertex_2: top_left.unalign(),
                vertex_3: bottom_left.unalign(),
                color: color.unalign(),
            },
            Triangle {
                vertex_1: bottom_left.unalign(),
                vertex_2: bottom_right.unalign(),
                vertex_3: top_right.unalign(),
                color: color.unalign(),
            },
        ]);
    }

    pub fn draw_line(&mut self, color: Vec3<f32>, start: Vec2<f32>, end: Vec2<f32>) {
        self.draw_rectangle(
            color,
            Vec2::new(start.distance(end) / 2.0, 0.1),
            start.midpoint(end),
            (start.y - end.y).atan2(start.x - end.x),
        );
    }
}

struct Runner<UpdateFn> {
    resources: Option<Resources>,
    held_keys: HashSet<KeyCode>,
    previously_held_keys: HashSet<KeyCode>,
    update: UpdateFn,
    last_instant: Option<Instant>,
    time_accumulator: f32,
    triangles: Vec<Triangle>,
    camera_position: Vec2<f32>,
    camera_height: f32,
    background_color: Vec3<f32>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, NoUninit)]
struct Triangle {
    vertex_1: Vec2U<f32>,
    vertex_2: Vec2U<f32>,
    vertex_3: Vec2U<f32>,
    color: Vec3U<f32>,
}

struct Resources {
    window: Arc<Window>,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    triangle_buffer: Buffer,
    pipeline: RenderPipeline,
}

impl<UpdateFn> ApplicationHandler for Runner<UpdateFn>
where
    UpdateFn: FnMut(&mut Context),
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.resources.is_some() {
            return;
        }

        let window_attributes = Window::default_attributes()
            .with_title("Slope2D Testbed")
            .with_visible(false);
        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("failed to create window"),
        );

        let instance = Instance::default();
        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions::default()))
            .expect("failed to get adapter");
        let (device, queue) = block_on(adapter.request_device(&DeviceDescriptor::default()))
            .expect("failed to get device");

        let surface = instance
            .create_surface(window.clone())
            .expect("failed to create surface");
        let window_inner_size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter, window_inner_size.width, window_inner_size.height)
            .expect("surface is not supported by adapter");
        surface.configure(&device, &surface_config);

        let triangle_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: (size_of::<Triangle>() * MAX_TRIANGLES) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader_module = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            cache: None,
            depth_stencil: None,
            multisample: MultisampleState::default(),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            multiview_mask: None,
            layout: None,
            vertex: VertexState {
                module: &shader_module,
                entry_point: None,
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[VertexBufferLayout {
                    array_stride: size_of::<Triangle>() as u64,
                    step_mode: VertexStepMode::Instance,
                    attributes: &[
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: offset_of!(Triangle, vertex_1) as u64,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: offset_of!(Triangle, vertex_2) as u64,
                            shader_location: 1,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: offset_of!(Triangle, vertex_3) as u64,
                            shader_location: 2,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x3,
                            offset: offset_of!(Triangle, color) as u64,
                            shader_location: 3,
                        },
                    ],
                }],
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: None,
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: surface_config.format,
                    blend: None,
                    write_mask: ColorWrites::all(),
                })],
            }),
        });

        window.set_visible(true);

        self.resources = Some(Resources {
            window,
            device,
            queue,
            surface,
            surface_config,
            triangle_buffer,
            pipeline,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(Resources {
            device,
            queue,
            surface,
            surface_config,
            triangle_buffer,
            pipeline,
            ..
        }) = &mut self.resources
        else {
            return;
        };

        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(keycode),
                        state,
                        ..
                    },
                ..
            } => match state {
                ElementState::Pressed => {
                    self.held_keys.insert(keycode);
                }
                ElementState::Released => {
                    self.held_keys.remove(&keycode);
                }
            },
            WindowEvent::RedrawRequested => 'redraw: {
                let mut reconfigure_surface = false;
                let surface_texture = loop {
                    match surface.get_current_texture() {
                        CurrentSurfaceTexture::Success(surface_texture) => break surface_texture,
                        CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                            reconfigure_surface = true;
                            break surface_texture;
                        }
                        CurrentSurfaceTexture::Timeout => break 'redraw,
                        CurrentSurfaceTexture::Occluded => break 'redraw,
                        CurrentSurfaceTexture::Outdated => {
                            surface.configure(device, surface_config);
                            continue;
                        }
                        CurrentSurfaceTexture::Lost => panic!("surface lost"),
                        CurrentSurfaceTexture::Validation => panic!("validation error"),
                    }
                };
                let output = surface_texture
                    .texture
                    .create_view(&TextureViewDescriptor::default());

                if self.triangles.len() > MAX_TRIANGLES {
                    panic!(
                        "cannot draw {} triangles (maximum is {MAX_TRIANGLES})",
                        self.triangles.len()
                    );
                }
                queue.write_buffer(triangle_buffer, 0, cast_slice(self.triangles.as_slice()));

                let mut encoder =
                    device.create_command_encoder(&CommandEncoderDescriptor::default());

                let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &output,
                        depth_slice: None,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color {
                                r: self.background_color.x as f64,
                                g: self.background_color.y as f64,
                                b: self.background_color.z as f64,
                                a: 1.0,
                            }),
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });

                if !self.triangles.is_empty() {
                    pass.set_pipeline(pipeline);
                    pass.set_vertex_buffer(
                        0,
                        triangle_buffer
                            .slice(..(size_of::<Triangle>() * self.triangles.len()) as u64),
                    );
                    pass.draw(0..3, 0..self.triangles.len() as u32);
                }

                drop(pass);
                queue.submit([encoder.finish()]);

                surface_texture.present();
                if reconfigure_surface {
                    surface.configure(device, surface_config);
                }
            }
            WindowEvent::Resized(new_size) => {
                surface_config.width = new_size.width;
                surface_config.height = new_size.height;
                surface.configure(device, surface_config);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let Some(Resources { window, .. }) = &self.resources else {
            return;
        };

        window.request_redraw();

        let now = Instant::now();
        let delta_time = if let Some(last_instant) = self.last_instant {
            now.duration_since(last_instant).as_secs_f32()
        } else {
            0.0
        };
        self.last_instant = Some(now);

        self.time_accumulator += delta_time;
        if self.time_accumulator < 0.0 {
            return;
        }
        self.time_accumulator -= TIME_STEP;

        let aspect_ratio = window.inner_size().width as f32 / window.inner_size().height as f32;

        self.triangles.clear();
        (self.update)(&mut Context {
            aspect_ratio,
            held_keys: &self.held_keys,
            previously_held_keys: &self.previously_held_keys,
            world_to_screen: Affine2::from_scale(Vec2::new(
                2.0 / self.camera_height / aspect_ratio,
                2.0 / self.camera_height,
            )) * Affine2::from_translation(-self.camera_position),
            camera_position: &mut self.camera_position,
            camera_height: &mut self.camera_height,
            background_color: &mut self.background_color,
            triangles: &mut self.triangles,
        });

        self.previously_held_keys.clone_from(&self.held_keys);
    }
}
