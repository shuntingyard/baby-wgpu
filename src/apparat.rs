//! Wrapping up the interaction with adapters, devices, surfaces, queues...

use tracing::{debug, info};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{event::*, window::Window};

/// The data structure holding it all
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    // Window is to be declared after surface so it gets dropped after it
    // as the surface contains unsafe refs to windows resources!
    pub window: Window,

    clear_color: wgpu::Color,
}

/// And its implementation
impl State {
    // Creation of some wgpu types is async.
    pub async fn new(window: Window) -> Self {
        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        let size = window.inner_size();

        // Handle to GPU where
        // `Backends::all` will give us Vulkan, Metal, DX12 and Browser WebGPU.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // SAFETY !
        // Surface needs to live as long as the window that created it.
        // Because `State` owns the window we should be Ok.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // This is a handle to the actual graphics card.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface), // Select an adapter able to deal with the
                // surface.
                force_fallback_adapter: false, // Usually means falling back to software.
            })
            .await
            .unwrap();

        // Show selected:
        debug!("Adapter {:#?}", &adapter.features());

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        info!("Downlevel to webgl2 defaults applied");
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        // debug!("{:?}", &device.features());

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.

        // Show present modes:
        debug!(
            "Present[ation] modes supported by surface: {:?}",
            &surface_caps.present_modes
        );

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format, // Define how surface textures will be stored on the GPU.
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            // <- surface guards
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        #[allow(deprecated)]
        if let WindowEvent::CursorMoved {
            device_id: _,
            position,
            modifiers: _,
        } = event
        {
            let r = position.x / self.size.width as f64;
            let g = position.y / self.size.height as f64;
            self.clear_color = wgpu::Color {
                r,
                g,
                b: 0.3,
                a: 1.0,
            };
        }
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        // `begin_render_pass()` borrows encoder mutably (aka &mut self). We can't call
        // `encoder.finish()` until we release that mutable borrow.
        drop(render_pass);

        // Submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
