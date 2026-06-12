mod tangent;
mod physics;
mod ergodic;
mod lyapunov;
mod scene;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler, 
    event::*, 
    event_loop::{ActiveEventLoop, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey}, 
    window::{Window}
};
use rand::{
    Rng, 
    RngExt, 
    SeedableRng, 
    rngs::StdRng
};

use glam::{Vec3, DVec3};
use wgpu::util::DeviceExt;

use tangent::*;
use physics::*;
use ergodic::*;
use lyapunov::*;
use scene::*;

/* Constants */
const MAX_HISTORY: usize = 5;
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

    // Buffers
    camera_buf:         wgpu::Buffer,
    sphere_verts_buf:   wgpu::Buffer,
    sphere_index_buf:   wgpu::Buffer,
    box_vertex_buf:     wgpu::Buffer,

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
                label:              Some("MSAA intermediate texture"),
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

        let cam_bgl: wgpu::BindGroupLayout = device.create_bind_group_layout(
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
                layout: &cam_bgl,
                entries: &[wgpu::BindGroupEntry{binding: 0, resource: camera_buf.as_entire_binding()}]
            }
        );

        // Layout of the rendering pipelines
        let pipeline_layout: wgpu::PipelineLayout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label:              Some("Pipeline Layout"),
                bind_group_layouts: &[Some(&cam_bgl)],
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
            camera_buf, sphere_verts_buf, sphere_index_buf, box_vertex_buf,
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
            let mut render_pass: wgpu::RenderPass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Billiards render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view:           &view,
                            depth_slice:    Option::<u32>::default(),
                            resolve_target: Some(&self.msaa_resolve_texture),
                            ops:            wgpu::Operations {
                                                load:   wgpu::LoadOp::Clear(wgpu::Color{ r: 0.04, g: 0.04, b: 0.07, a: 1.0 }),
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
        }

        // TODO: egui analytics board render pass
        let egui_input: egui::RawInput = self.egui_state.take_egui_input(&window);
        let egui_output: egui::FullOutput = self.egui_ctx.run_ui(egui_input, |ui| {build_egui_ui(ui, state)});
        {

        }

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
    ctx.set_style(style_ctx);

    // Create UI window
    egui::Window::new("3D Sinai Billiards")
        .fixed_pos(egui::pos2(10.0, 10.0))
        .default_width(350.0)
        .resizable(egui::Vec2b::FALSE)
        .show(ctx, |ui| {
            // Metadata display
            ui.colored_label(egui::Color32::from_rgb(120, 200, 255), format!("Runtime: {:.1}s", time_elapsed));
            ui.separator();

            // Lyapunov Spectra display
            ui.colored_label(egui::Color32::from_rgb(200, 230, 255), "Lyapunov Spectra");
            ui.indent("Lya_spectra", |ui| {
                // Displaying live value and corresponding (relative) horizontal color bar scale for each exponent
                let eps: f64 = 0.002;
                for (idx, &lya_exp) in state.stats.get_lyapunov_spectra().iter().enumerate() {
                    let color = if      lya_exp > eps   {egui::Color32::from_rgb(80, 255, 100)}    // Postive exponent: GREEN
                                else if lya_exp < -eps  {egui::Color32::from_rgb(255, 80, 80)}     // Negative exponent: RED
                                else                    {egui::Color32::from_rgb(180, 180, 100)};  // Zero-threshold: YELLOW
                    
                    let width_scale: f32 = 80.0;
                    let half_width = (width_scale * lya_exp.abs() as f32).min(width_scale);
                    ui.horizontal(|ui| {
                        let (rect, resp): (egui::Rect, egui::Response) = ui.allocate_exact_size(
                                                                            egui::Vec2{x: width_scale, y: 10.0},
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
                                egui::Color32::from_rgb(00, 60, 60)
                            );
                        }
                    });
                }
            });
            ui.separator();

            // Statistics display
            ui.colored_label(egui::Color32::from_rgb(200, 230, 255), "Ergodic statistics");
            ui.indent("ergodic_stats", |ui| {
                stats_display(ui, "KS entropy", format!("{:.4}", erg_data.get_ks_entropy()), 
                                    egui::Color32::from_rgb(255, 200, 80));
                stats_display(ui, "Kaplan-Yorke dimension", format!("{:.4}", erg_data.get_ky_dim()), 
                                    egui::Color32::from_rgb(180, 120, 255));
                stats_display(ui, "Lyapunov Time", format!("{:.4}", erg_data.get_lyapunov_time()), 
                                    egui::Color32::from_rgb(180, 120, 255));
                stats_display(ui, "Collisions count", format!("{}", state.traj.get_collision_count()), 
                                    egui::Color32::from_rgb(180, 120, 255));
                stats_display(ui, "Mean-Free path", format!("{:.4}", state.traj.get_mean_free_path()), 
                                    egui::Color32::from_rgb(180, 120, 255));
            });
            ui.separator();
        });
}

#[inline] 
fn stats_display(ui: &mut egui::Ui, stats_type: &str, value: String, color: egui::Color32) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(stats_type).size(11.0).color(egui::Color32::from_rgb(150, 160, 185)));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.colored_label(color, egui::RichText::new(value).size(12.0));
        });
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
    
}

fn main() {
    // Environment logger
    env_logger::init();

    // Setup event loop and window
    let (width, height): (u32, u32) = (1920, 1080);
    let event_loop = EventLoop::new().unwrap();
    let window = std::sync::Arc::new(
        event_loop.create_window(
            Window::default_attributes()
                .with_title("3D Sinai Billiards Ergodic Dynamics")
                .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        ).unwrap()
    );

    // Setup render pipelines
    let mut renderer = pollster::block_on(Renderer::new(window.clone()));

    // Setup program states (random)
    let seed: u64 = 69;
    let mut program_state = BilliardsState::new_random(seed);

    // Event loop
    // event_loop.run();
}

