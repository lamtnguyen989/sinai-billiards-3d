mod tangent;
mod physics;
mod ergodic;
mod lyapunov;
mod scene;

use tangent::*;
use physics::*;
use ergodic::*;
use lyapunov::*;
use scene::*;

use std::sync::Arc;
use winit::{ 
    event::*, 
    event_loop::{ActiveEventLoop, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey}, 
    window::{Window, WindowId, WindowAttributes},
    dpi::{PhysicalPosition}
};
use rand::{Rng, RngExt,SeedableRng, rngs::StdRng};
use glam::{Vec3, DVec3};
use wgpu::util::DeviceExt;

/* Constants */
const MAX_HISTORY: usize = 10;
const STEPS_PER_FRAME: usize = 1;   // Number of update steps per rendering frame

/***
*   Billiard System state
***/
struct BilliardsState
{
    traj:           Trajectory,
    stats:          ErgodicStats,
    start_time:     std::time::Instant,
    frame_counter:  u64,
    trail_length:   usize,
    paused:         bool
}

impl BilliardsState
{
    // Constructors
    fn new_random(seed: u64) -> Self {
        // Setting up state for random traj from seed
        let mut rng = StdRng::seed_from_u64(seed);
        let color = trajectory_palette()[0];

        return Self {
            traj:           random_trajectory(&mut rng, color),
            stats:          ErgodicStats::new(&[0.0; NUM_TANGENTS]),
            start_time:     std::time::Instant::now(),
            frame_counter:  0,
            trail_length:   MAX_HISTORY,
            paused:         true
        };
    }

    /*
    fn new() -> Self {

    }
    */

    // Update mechanisms
    fn update(&mut self) -> () {
        // Do nothing on paused
        if self.paused {return;}

        // Compute results between rendering frame
        for k in 0..STEPS_PER_FRAME {
            match self.traj.update(self.trail_length) {
                Ok(_)   => {},
                Err(e)  => eprintln!("Trajectory update failed. Error: {:?}", e),
            };
        };

        // Compute resulting stats
        self.stats = ErgodicStats::compute_from_trajectory(&self.traj);
        self.frame_counter += 1;
    }

    // Reset mechanisms
    fn reset(&mut self) -> () {
        todo!();
    }

    fn reset_from(&mut self, pos: Vec3, vel: Vec3) -> () {
        // Pick a new color from palllete
        let palette = trajectory_palette();
        let color = palette[(self.frame_counter as usize) % palette.len()];

        // Reset internal states
        self.traj = Trajectory::new(pos, vel, color);
        self.stats = ErgodicStats::new(&[0.0; NUM_TANGENTS]);
        self.frame_counter = 0;
    }
    
}

fn trajectory_palette() -> Vec<[f32; 4]> {
    // Set up a pre-determined color pallette for the trajectory
    return vec![
        [1.0, 0.35, 0.2,  0.9],
        [0.2, 0.8,  1.0,  0.9],
        [0.4, 1.0,  0.4,  0.9],
        [1.0, 0.85, 0.1,  0.9],
        [0.9, 0.3,  1.0,  0.9],
        [0.1, 1.0,  0.85, 0.9],
        [1.0, 0.6,  0.05, 0.9],
        [0.5, 0.5,  1.0,  0.9],
        [1.0, 0.4,  0.6,  0.9],
        [0.3, 0.9,  0.6,  0.9],
        [0.9, 0.9,  0.3,  0.9],
        [0.6, 0.3,  0.9,  0.9],
    ];
}

/***
*   Renderer data
***/
struct Renderer
{
    // GPU context
    surface:            wgpu::Surface<'static>,
    device:             wgpu::Device,
    queue:              wgpu::Queue,
    config:             wgpu::SurfaceConfiguration,
    size:               winit::dpi::PhysicalSize<u32>,

    // Texture Views (doing MSAA for the LOLs)
    depth_texture_view:     wgpu::TextureView,
    msaa_resolve_texture:   wgpu::TextureView,

    // Render pipelines
    line_pipeline:      wgpu::RenderPipeline,
    sphere_pipeline:    wgpu::RenderPipeline,
    box_pipeline:       wgpu::RenderPipeline,

    // Camera
    camera_buf:         wgpu::Buffer,
    camera_bgl:         wgpu::BindGroupLayout,
    camera_bind_group:  wgpu::BindGroup,

    // Buffers
    sphere_verts_buf:   wgpu::Buffer,
    sphere_index_buf:   wgpu::Buffer,
    box_vertex_buf:     wgpu::Buffer,

    // Buffer count (size) for draw range
    sphere_index_count: u32,
    box_vertex_count:   u32,

    // egui
    egui_ctx:           egui::Context,
    egui_renderer:      egui_wgpu::Renderer,
    egui_state:         egui_winit::State,
}

impl Renderer
{
    async fn new(window: std::sync::Arc<Window>) -> Self {
        // Creating wgpu instance 
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                backends:                   wgpu::Backends::PRIMARY,
                flags:                      wgpu::InstanceFlags::default(),
                memory_budget_thresholds:   wgpu::MemoryBudgetThresholds::default(),
                backend_options:            wgpu::BackendOptions::default(),
                display:                    None,
            }
        );
        let surface: wgpu::Surface = instance.create_surface(window.clone()).unwrap();
        let adapter: wgpu::Adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::HighPerformance,
                compatible_surface:     Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();

        // Creating a wgpu logical device and queue
        let (device, queue): (wgpu::Device, wgpu::Queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label:                  None,
                required_features:      wgpu::Features::default(),
                required_limits:        wgpu::Limits::default(),
                experimental_features:  wgpu::ExperimentalFeatures::default(),
                memory_hints:           wgpu::MemoryHints::Performance,
                trace:                  wgpu::Trace::Off,
            }
        ).await.unwrap();

        // Surface configuration
        let size = window.inner_size();
        let surface_capabilities: wgpu::SurfaceCapabilities = surface.get_capabilities(&adapter);
        let surface_texture_format: wgpu::TextureFormat = surface_capabilities.formats.iter()
                                        .find(|f| {**f == wgpu::TextureFormat::Rgba16Float}).copied()   // HDR rendering capabilities first
                                        .or_else(|| surface_capabilities.formats.iter()
                                                        .find(|f| {f.is_srgb()}).copied())              // Standard RGB if no HDR
                                        .unwrap_or(surface_capabilities.formats[0]);                    // Fall back to hardware capabilities

        let config = wgpu::SurfaceConfiguration {
            usage:                          wgpu::TextureUsages::RENDER_ATTACHMENT,
            format:                         surface_texture_format,
            width:                          size.width,
            height:                         size.height,
            present_mode:                   wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency:  2,
            alpha_mode:                     surface_capabilities.alpha_modes[0],
            view_formats:                   vec![],
        };
        surface.configure(&device, &config);

        // Depth texture view
        let depth_texture_view: wgpu::TextureView = make_depth_texture_view(&device, size.width, size.height);

        // MSAA texture view
        let msaa_resolve_texture: wgpu::TextureView = device.create_texture(
            &wgpu::TextureDescriptor {
                label:              Some("MSAA resolve texture"),
                size:               wgpu::Extent3d {width: size.width, height: size.height, depth_or_array_layers: 1},
                mip_level_count:    1,
                sample_count:       4,
                dimension:          wgpu::TextureDimension::D2,
                format:             surface_texture_format,
                usage:              wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats:       &[],
            }
        ).create_view(&wgpu::TextureViewDescriptor::default());

        // Load shader file as a module
        let shaders: wgpu::ShaderModule  = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label:  Some("WGSL shaders"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shaders.wgsl").into()),
            }
        );

        // Create bindings
        let camera_buf: wgpu::Buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label:              Some("Camera Buffer"),
                size:               std::mem::size_of::<CameraUniform>() as u64,
                usage:              wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM ,
                mapped_at_creation: false,
            }
        );

        let camera_bgl: wgpu::BindGroupLayout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding:    0,
                    visibility: wgpu::ShaderStages::VERTEX |wgpu::ShaderStages::FRAGMENT | 
                                wgpu::ShaderStages::RAY_GENERATION | wgpu::ShaderStages::COMPUTE, // Futureproofing
                    ty:         wgpu::BindingType::Buffer {
                                    ty:                 wgpu::BufferBindingType::Uniform,
                                    has_dynamic_offset: false,
                                    min_binding_size:   None
                                },
                    count:      None,
                }],
            }
        );

        let camera_bind_group: wgpu::BindGroup = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Camera Bind Group"),
                layout: &camera_bgl,
                entries: &[wgpu::BindGroupEntry{binding: 0, resource: camera_buf.as_entire_binding()}]
            }
        );

        // Layout of the rendering pipelines
        let pipeline_layout: wgpu::PipelineLayout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label:              Some("Pipeline Layout"),
                bind_group_layouts: &[Some(&camera_bgl)],
                immediate_size:     0
            }
        );

        // Line pipeline
        let line_pipeline: wgpu::RenderPipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label:          wgpu::Label::Some("Line render pipeline"),
                layout:         Some(&pipeline_layout),
                vertex:         wgpu::VertexState {
                                    module: &shaders,
                                    entry_point: Some("vertex_line"),
                                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                                    buffers: &[LineData::to_vertex_buffer_layout()]
                                },
                primitive:      wgpu::PrimitiveState {
                                    topology:           wgpu::PrimitiveTopology::LineList,
                                    strip_index_format: Option::<wgpu::IndexFormat>::default(),
                                    front_face:         wgpu::FrontFace::default(),
                                    cull_mode:          Option::<wgpu::Face>::default(),
                                    unclipped_depth:    false,
                                    polygon_mode:       wgpu::PolygonMode::default(),
                                    conservative:       false,
                                },
                depth_stencil:  Some(wgpu::DepthStencilState {
                                    format:                 wgpu::TextureFormat::Depth32Float,
                                    depth_write_enabled:    Some(true),
                                    depth_compare:          Some(wgpu::CompareFunction::Less),
                                    stencil:                wgpu::StencilState::default(),
                                    bias:                   wgpu::DepthBiasState::default(),
                                }),
                multisample:    wgpu::MultisampleState {count: 4, mask: !0, alpha_to_coverage_enabled: false,},
                fragment:       Some(wgpu::FragmentState {
                                    module:                  &shaders,
                                    entry_point:            Some("fragment_line"),
                                    compilation_options:    wgpu::PipelineCompilationOptions::default(),
                                    targets:                &[Some(wgpu::ColorTargetState {
                                                                    format:     surface_texture_format,
                                                                    blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
                                                                    write_mask: wgpu::ColorWrites::ALL,
                                                                }),
                                                            ],
                                }),
                multiview_mask: None,
                cache:          None,
            }
        );

        // Sphere pipeline
        let sphere_pipeline: wgpu::RenderPipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label:          wgpu::Label::Some("Sphere render pipeline"),
                layout:         Some(&pipeline_layout),
                vertex:         wgpu::VertexState {
                                    module: &shaders,
                                    entry_point: Some("vertex_sphere"),
                                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                                    buffers: &[SphereData::to_vertex_buffer_layout()]
                                },
                primitive:      wgpu::PrimitiveState {
                                    topology:           wgpu::PrimitiveTopology::TriangleList,
                                    strip_index_format: Option::<wgpu::IndexFormat>::default(),
                                    front_face:         wgpu::FrontFace::default(),
                                    cull_mode:          Some(wgpu::Face::Back), // We only view the front
                                    unclipped_depth:    false,
                                    polygon_mode:       wgpu::PolygonMode::default(),
                                    conservative:       false,
                                },
                depth_stencil:  Some(wgpu::DepthStencilState {
                                    format:                 wgpu::TextureFormat::Depth32Float,
                                    depth_write_enabled:    Some(false),    // This is because the sphere is translucent
                                    depth_compare:          Some(wgpu::CompareFunction::Less),
                                    stencil:                wgpu::StencilState::default(),
                                    bias:                   wgpu::DepthBiasState::default(),
                                }),
                multisample:    wgpu::MultisampleState {count: 4, mask: !0, alpha_to_coverage_enabled: false,},
                fragment:       Some(wgpu::FragmentState {
                                    module:                  &shaders,
                                    entry_point:            Some("fragment_sphere"),
                                    compilation_options:    wgpu::PipelineCompilationOptions::default(),
                                    targets:                &[Some(wgpu::ColorTargetState {
                                                                    format:     surface_texture_format,
                                                                    blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
                                                                    write_mask: wgpu::ColorWrites::ALL,
                                                                }),
                                                            ],
                                }),
                multiview_mask: None,
                cache:          None,
            }
        );

        // Box pipeline
        let box_pipeline: wgpu::RenderPipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label:          wgpu::Label::Some("Box render pipeline"),
                layout:         Some(&pipeline_layout),
                vertex:         wgpu::VertexState {
                                    module: &shaders,
                                    entry_point: Some("vertex_box"),
                                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                                    buffers: &[BoxData::to_vertex_buffer_layout()]
                                },
                primitive:      wgpu::PrimitiveState {
                                    topology:           wgpu::PrimitiveTopology::LineList,
                                    strip_index_format: Option::<wgpu::IndexFormat>::default(),
                                    front_face:         wgpu::FrontFace::default(),
                                    cull_mode:          Option::<wgpu::Face>::default(),
                                    unclipped_depth:    false,
                                    polygon_mode:       wgpu::PolygonMode::default(),
                                    conservative:       false,
                                },
                depth_stencil:  Some(wgpu::DepthStencilState {
                                    format:                 wgpu::TextureFormat::Depth32Float,
                                    depth_write_enabled:    Some(true),
                                    depth_compare:          Some(wgpu::CompareFunction::Less),
                                    stencil:                wgpu::StencilState::default(),
                                    bias:                   wgpu::DepthBiasState::default(),
                                }),
                multisample:    wgpu::MultisampleState {count: 4, mask: !0, alpha_to_coverage_enabled: false,},
                fragment:       Some(wgpu::FragmentState {
                                    module:                  &shaders,
                                    entry_point:            Some("fragment_box"),
                                    compilation_options:    wgpu::PipelineCompilationOptions::default(),
                                    targets:                &[Some(wgpu::ColorTargetState {
                                                                    format:     surface_texture_format,
                                                                    blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
                                                                    write_mask: wgpu::ColorWrites::ALL,
                                                                }),
                                                            ],
                                }),
                multiview_mask: None,
                cache:          None,
            }
        );

        // Build and upload sphere geometry data
        let (sph_stacks, sph_slices) = (64_u32, 64_u32);
        let (sph_vert, sph_idx): (Vec<SphereData>, Vec<u32>) = build_sphere(SPHERE_CENTER, SPHERE_RADIUS, sph_stacks, sph_slices);
        let sphere_verts_buf: wgpu::Buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:      Some("Sphere Vertex Buffer"),
                contents:   bytemuck::cast_slice(&sph_vert),
                usage:      wgpu::BufferUsages::VERTEX,
            }
        );

        let sphere_index_buf: wgpu::Buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:      Some("Sphere Index Buffer"),
                contents:   bytemuck::cast_slice(&sph_idx),
                usage:      wgpu::BufferUsages::INDEX,
            }
        );

        // Build and upload box data
        let box_verts: Vec<BoxData> = build_box(BOX_SIZE);
        let box_vertex_buf: wgpu::Buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:      Some("Box Vertex Buffer"),
                contents:   bytemuck::cast_slice(&box_verts),
                usage:      wgpu::BufferUsages::VERTEX,
            }
        );

        // egui
        let egui_ctx = egui::Context::default();
        let egui_renderer = egui_wgpu::Renderer::new(&device, surface_texture_format, 
                                                    egui_wgpu::RendererOptions {msaa_samples: 4, ..Default::default()});
        let egui_state = egui_winit::State::new(egui_ctx.clone(), egui::ViewportId::ROOT, &*window, None, None, None);

        return Self {
            surface, device, queue, config, size,
            depth_texture_view, msaa_resolve_texture,
            line_pipeline, sphere_pipeline, box_pipeline,
            camera_buf, camera_bgl, camera_bind_group,
            sphere_verts_buf, sphere_index_buf, box_vertex_buf,
            box_vertex_count: box_verts.len() as u32, sphere_index_count: sph_idx.len() as u32,
            egui_ctx, egui_renderer, egui_state,
        }
    }

    // Render passes (to be called after advacing simulation states)
    fn render(&mut self, state: &BilliardsState, cam: &OrbitCamera, window: &std::sync::Arc<Window>) -> anyhow::Result<()> {
        // Get and validate current texture
        let output: wgpu::SurfaceTexture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture)    => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {self.surface.configure(&self.device, &self.config); surface_texture},
            wgpu::CurrentSurfaceTexture::Outdated                    => {self.surface.configure(&self.device, &self.config); return Ok(());},
            wgpu::CurrentSurfaceTexture::Lost                        => { anyhow::bail!("Lost device!") },
            wgpu::CurrentSurfaceTexture::Timeout                     => { return Ok(());},
            wgpu::CurrentSurfaceTexture::Occluded                    => { return Ok(());},
            wgpu::CurrentSurfaceTexture::Validation                  => { return Ok(());},
        };

        // Create view from surface texture and command encoder
        let view: wgpu::TextureView = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Command encoder
        let mut encoder: wgpu::CommandEncoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default(),);

        // Update (uniform) camera information
        let elapsed_time: f32 = state.start_time.elapsed().as_secs_f32();
        let camera_uniform = cam.to_uniform(elapsed_time);
        self.queue.write_buffer(&self.camera_buf, 0, bytemuck::bytes_of(&camera_uniform));

        // Create trajectory lines vertex data and rendering buffers
        let traj: &Trajectory = &state.traj;
        let traj_positions: Vec<glam::Vec3> = traj.get_positions();
        let n = traj_positions.len();

        let mut traj_line_data: Vec<LineData> = vec![];
        if n > 1 {
            for k in 0..n-1 {
                traj_line_data.push(LineData {
                    position:   traj_positions[k].to_array(),
                    color:      traj.color,
                    age:        (k as f32) / ((n-1) as f32)
                });

                traj_line_data.push(LineData {
                    position:   traj_positions[k+1].to_array(),
                    color:      traj.color,
                    age:        ((k+1) as f32) / ((n-1) as f32)
                });
            }
        }

        let traj_line_buf: Option<wgpu::Buffer> = match traj_line_data[..] {
            []  => None,
            _   => Some(self.device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label:      Some("Trajectory lines buffer"),
                                contents:   bytemuck::cast_slice(&traj_line_data),
                                usage:      wgpu::BufferUsages:: VERTEX,
                            })),
        };

        // Scoped render pass is fine, if in the future I want `RenderPass::forget_lifetime()`, I'll refactor ig
        // Billiards render pass
        {
            // Setting up the pass
            let mut billiards_render_pass: wgpu::RenderPass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Billiards render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view:           &self.msaa_resolve_texture,
                            depth_slice:    Option::<u32>::default(),
                            resolve_target: None,
                            ops:            wgpu::Operations {
                                                load:   wgpu::LoadOp::Clear(wgpu::Color{ r: 0.04, g: 0.04, b: 0.07, a: 0.75 }),
                                                store:  wgpu::StoreOp::Store,
                                            },
                        })
                    ],
                    depth_stencil_attachment:   Some(wgpu::RenderPassDepthStencilAttachment {
                                                    view:           &self.depth_texture_view,
                                                    depth_ops:      Some(wgpu::Operations {
                                                                            load: wgpu::LoadOp::Clear(1.0),
                                                                            store:  wgpu::StoreOp::Store,
                                                                        }),
                                                    stencil_ops:    None,
                                                }),
                    timestamp_writes:           None,
                    occlusion_query_set:        Default::default(),
                    multiview_mask:             None,                    
                }
            );

            // Set camera bind group
            billiards_render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // Draw box
            billiards_render_pass.set_pipeline(&self.box_pipeline);
            billiards_render_pass.set_vertex_buffer(0, self.box_vertex_buf.slice(..));
            billiards_render_pass.draw(0..self.box_vertex_count, 0..1);

            // Draw sphere
            billiards_render_pass.set_pipeline(&self.sphere_pipeline);
            billiards_render_pass.set_vertex_buffer(0, self.sphere_verts_buf.slice(..));
            billiards_render_pass.set_index_buffer(self.sphere_index_buf.slice(..), wgpu::IndexFormat::Uint32);
            billiards_render_pass.draw_indexed(0..self.sphere_index_count, 0, 0..1);

            // Draw trajectory lines
            if traj_line_buf.is_some() {
                let buf: &wgpu::Buffer = traj_line_buf.as_ref().unwrap();
                billiards_render_pass.set_pipeline(&self.line_pipeline);
                billiards_render_pass.set_vertex_buffer(0, buf.slice(..));
                billiards_render_pass.draw(0..traj_line_data.len() as u32, 0..1);
            }
        }

        // egui analytics board buffer setup
        let egui_input: egui::RawInput = self.egui_state.take_egui_input(&window);
        let egui_output: egui::FullOutput = self.egui_ctx.run_ui(egui_input, |ui| {build_egui_ui(ui, state)});

        self.egui_state.handle_platform_output(&window, egui_output.platform_output.clone());
        let egui_primatives: Vec<egui::ClippedPrimitive> = self.egui_ctx.tessellate(egui_output.shapes, egui_output.pixels_per_point);
        let egui_screen_descriptor = egui_wgpu::ScreenDescriptor {
                                        size_in_pixels:     [self.size.width, self.size.height], 
                                        pixels_per_point:   egui_output.pixels_per_point,
                                    };
        
        for (texture_id, img_delta) in &egui_output.textures_delta.set {
            self.egui_renderer.update_texture(&self.device, &self.queue, *texture_id, img_delta);
        }
        self.egui_renderer.update_buffers(&self.device, &self.queue, &mut encoder, &egui_primatives, &egui_screen_descriptor);


        // egui render pass 
        {
            let mut egui_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Egui render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view:           &self.msaa_resolve_texture,
                            depth_slice:    Option::<u32>::default(),
                            resolve_target: Some(&view),
                            ops:            wgpu::Operations {
                                                load:   wgpu::LoadOp::Load,
                                                store:  wgpu::StoreOp::Store,
                                            },
                        })
                    ],
                    depth_stencil_attachment:   None,
                    timestamp_writes:           None,
                    occlusion_query_set:        Default::default(),
                    multiview_mask:             None,                    
                }
            ).forget_lifetime();

            self.egui_renderer.render(&mut egui_pass, &egui_primatives, &egui_screen_descriptor);
            drop(egui_pass);
        }

        // Cleaning up resources 
        for id in &egui_output.textures_delta.free {self.egui_renderer.free_texture(id);}

        // Submit and present 
        self.queue.submit([encoder.finish()]);
        output.present();

        Ok(())
    }
}

fn build_egui_ui(ui: &mut egui::Ui, state: &BilliardsState) {
    // Pulling rendering data
    let erg_data: &ErgodicStats = &state.stats;
    let time_elapsed = state.start_time.elapsed().as_secs_f32();

    // Create color pallete for the rendering panel
    let ctx: &egui::Context = ui.ctx();
    let mut style_ctx: egui::Style = (*ctx.global_style()).clone();
    style_ctx.visuals.window_fill = egui::Color32::from_rgba_premultiplied(8, 10, 20, 235);
    style_ctx.visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 70, 120));
    ctx.set_global_style(style_ctx);

    // Create UI window
    egui::Window::new("3D Sinai Billiards")
        .fixed_pos(egui::pos2(10.0, 10.0))
        .default_width(350.0)
        .resizable(egui::Vec2b::FALSE)
        .show(ctx, |ui| {
            // Metadata display
            // ui.colored_label(egui::Color32::from_rgb(120, 200, 255), format!("Runtime: {:.1}s", time_elapsed));
            // ui.separator();

            // Lyapunov Spectra display
            ui.colored_label(egui::Color32::from_rgb(200, 230, 255), "Lyapunov Spectra");
            ui.indent("Lya_spectra", |ui| {
                // Displaying live value and corresponding (relative) horizontal color bar scale for each exponent
                let eps: f64 = 0.005;
                for &lya_exp in state.stats.get_lyapunov_spectra().iter() {
                    let color = if      lya_exp > eps   {egui::Color32::from_rgb(80, 255, 100)}    // Postive exponent: GREEN
                                else if lya_exp < -eps  {egui::Color32::from_rgb(255, 80, 80)}     // Negative exponent: RED
                                else                    {egui::Color32::from_rgb(180, 180, 100)};  // Zero-threshold: YELLOW

                    let width_scale: f32 = 80.0;
                    let half_width = (width_scale * lya_exp.abs() as f32).min(width_scale);
                    ui.horizontal(|ui| {
                        let (rect, resp): (egui::Rect, egui::Response) = ui.allocate_exact_size(
                                                                            egui::Vec2{x: width_scale, y: 12.0},
                                                                            egui::Sense::HOVER
                                                                        );
                        let bar_mid: f32 = rect.left() + (width_scale / 2.0);
                        if lya_exp > 0.0 {
                            ui.painter().rect_filled(
                                egui::Rect::from_x_y_ranges(bar_mid..=bar_mid+half_width, rect.y_range()),
                                0.0, 
                                egui::Color32::from_rgb(60, 200, 80)
                            );
                        }
                        else {
                            ui.painter().rect_filled(
                                egui::Rect::from_x_y_ranges(bar_mid-half_width..=bar_mid, rect.y_range()),
                                0.0, 
                                egui::Color32::from_rgb(200, 60, 60)
                            );
                        }
                        ui.colored_label(color, egui::RichText::new(format!("{:+.4}", lya_exp)).size(12.0).monospace());
                    });
                }
            });
            ui.separator();

            // Statistics display
            ui.colored_label(egui::Color32::from_rgb(200, 230, 255), "Ergodic statistics");
            ui.indent("ergodic_stats", |ui| {
                stats_display(ui, "Metric entropy", format!("{:.4}", erg_data.get_ks_entropy()), 
                                    egui::Color32::from_rgb(255, 200, 80));
                stats_display(ui, "Kaplan-Yorke dimension", format!("{:.4}", erg_data.get_ky_dim()), 
                                    egui::Color32::from_rgb(255, 200, 80));
                stats_display(ui, "Lyapunov Time", format!("{:.4}", erg_data.get_lyapunov_time()), 
                                    egui::Color32::from_rgb(255, 200, 80));
                stats_display(ui, "Collisions count", format!("{}", state.traj.get_collision_count()), 
                                    egui::Color32::from_rgb(255, 200, 80));
                stats_display(ui, "Mean-Free path", format!("{:.4}", state.traj.get_mean_free_path()), 
                                    egui::Color32::from_rgb(255, 200, 80));
            });
            ui.separator();

            // Display viewing controls
            ui.colored_label(egui::Color32::from_rgb(200, 230, 255), "CONTROLS");
            ui.indent ("Controls", |ui| {
                controls_display(ui, "Spacebar", "Pause / Resume");
                controls_display(ui, "R", "Reset");
                controls_display(ui, "Drag", "Orbitting camera");
                controls_display(ui, "Scroll", "Zoom");
            });

            // Adding PAUSED indicator
            if state.paused {
                ui.separator();
                ui.centered_and_justified(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(255, 180, 50), "⏸  PAUSED");
                });
            }
        });
}

#[inline] 
fn stats_display(ui: &mut egui::Ui, stats_type: &str, value: String, color: egui::Color32) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(stats_type).size(12.0).color(egui::Color32::from_rgb(150, 160, 185)));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.colored_label(color, egui::RichText::new(value).size(12.0));
        });
    });
}

#[inline]
fn controls_display(ui: &mut egui::Ui, key: &str, description: &str) {
    ui.horizontal(|ui| {
        ui.colored_label(
            egui::Color32::from_rgb(255, 200, 80),
            egui::RichText::new(format!("{key:>12}")).size(12.0).monospace(),
        );
        ui.label(egui::RichText::new(description)
            .size(12.0)
            .color(egui::Color32::from_rgb(150, 160, 180)));
    });
}

fn make_depth_texture_view(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
    return device.create_texture(
        &wgpu::TextureDescriptor {
            label:              Some("Depth Texture View"),
            size:               wgpu::Extent3d {width: width, height: height, depth_or_array_layers: 1},
            mip_level_count:    1,
            sample_count:       4,
            dimension:          wgpu::TextureDimension::D2,
            format:             wgpu::TextureFormat::Depth32Float,
            usage:              wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats:       &[],
        }
    ).create_view(&wgpu::TextureViewDescriptor::default());
}

// App rendering struct
struct App
{
    // Rendering context
    window:     Option<Arc<Window>>,
    renderer:   Option<Renderer>,
    camera:     Option<OrbitCamera>,
    state:      BilliardsState,
    resolution: (u32, u32),

    // Behavior helper variables
    seed:           Option<u64>,
    mouse_pressed:  bool,
    last_mouse_pos: Option<PhysicalPosition<f64>>, 
}

impl App 
{
    // Base Constructors (only actually construct the state)
    fn new_random(seed: u64, resolution: (u32, u32)) -> Self {
        return Self {
            window:     None,
            renderer:   None,
            camera:     None,
            state:      BilliardsState::new_random(seed),
            resolution: resolution,

            seed:           Some(seed),
            mouse_pressed:  false,
            last_mouse_pos: None 
        }
    }
}

impl winit::application::ApplicationHandler for App
{   
    // Essentially constructor for windows and rendering context
    // Mainly winit 0.30+ convention
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Do nothing if the window is alrerady initialized
        if (self.window.is_some()) {
            return;
        }

        // Initalize windows and rendering objects
        let (width, height) = self.resolution;
        let window_attrs = WindowAttributes::default()
                                .with_title("3D Sinai Billiards Ergodic Dynamics")
                                .with_inner_size(winit::dpi::LogicalSize::new(width, height));

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        let renderer = pollster::block_on(Renderer::new(window.clone()));

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.camera = Some(OrbitCamera::new(BOX_SIZE, width as f32 / height as f32));
    }

    // About to wait handling
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    // Window event handler
    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let (Some(window), Some(renderer), Some(camera)) = (self.window.as_ref(), self.renderer.as_mut(), self.camera.as_mut())
                                                            else { return};

        let egui_consumed: bool = renderer.egui_state
                                        .on_window_event(window.as_ref(), &event)
                                        .consumed;
        match event {
            WindowEvent::CloseRequested => { /* Exit event */
                // Drop resources so no SEG_FAULT like a good citizen
                self.renderer = None;
                self.window = None;

                // Exit
                event_loop.exit()
            }
            WindowEvent::RedrawRequested => { /* Advancing simulation */
                // Step through the simulation
                self.state.update();

                // Render the simulation result
                match renderer.render(&self.state, camera, window) {
                    Ok(_) => {},
                    Err(e) => eprintln!("{e:?}"),
                }
            }
            WindowEvent::KeyboardInput {/* Keyboard input handling */
                device_id:      _,
                event:          KeyEvent {physical_key: PhysicalKey::Code(kc), state: ElementState::Pressed, ..},
                is_synthetic:   false,
            } => {
                match kc {
                    KeyCode::Space  => self.state.paused = !self.state.paused,
                    KeyCode::KeyR => self.state = BilliardsState::new_random(self.seed.unwrap()),
                    _ => {}
                }
            },
            WindowEvent::MouseInput {device_id: _, state: input_state, button: MouseButton::Left,} if !egui_consumed => { /* Left-click */
                // Update last mouse location on a new left-click that is not consumed by egui
                self.mouse_pressed = (input_state == ElementState::Pressed);
                if self.mouse_pressed {self.last_mouse_pos = None;}
            },
            WindowEvent::CursorMoved {device_id: _, position: cursor_pos} => { /* Dragging handled as orbitiing */
                // Orbitting if mouse is clicked not on the egui panel
                if self.mouse_pressed && !egui_consumed {
                    if self.last_mouse_pos.is_some() {
                        let curr_pos = self.last_mouse_pos.unwrap();
                        let (delta_x, delta_y) = ((cursor_pos.x - curr_pos.x) as f32, (cursor_pos.y - curr_pos.y) as f32);
                        camera.orbit(delta_x, delta_y);
                    }
                }
                // Update mouse last position
                self.last_mouse_pos = Some(cursor_pos);
            },
            WindowEvent::MouseWheel {device_id: _, delta: wheel_delta, phase: _} if !egui_consumed => { /* Scroll for Zoom */
                let delta = match wheel_delta { // Only zoom based on vertical data
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p)   => p.y as f32 * 0.1,
                }; 
                camera.zoom(delta);
            }
            _ => {}
        }
    }
}

fn main() {
    // Environment logger
    env_logger::init();

    // Setup app
    let (width, height): (u32, u32) = (1280, 800);
    let seed: u64 = 69;
    let mut app = App::new_random(seed, (width, height));

    // Event loop
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}

