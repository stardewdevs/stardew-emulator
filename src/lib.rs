pub mod renderer;
pub mod gpu;
pub mod glyph;
pub mod shader;
pub mod surface;
pub mod pipeline;
pub mod buffer;
pub mod texture;
pub mod color;
pub mod font;
pub mod effect;
pub mod jni;
pub mod ffi;
pub mod error;
pub mod logger;
pub mod utils;
pub mod platform;
pub mod snapshot;

use renderer::Renderer;
use sugarloaf::Sugarloaf;
use wgpu::Device;
use snapshot::{Cell, Color, TerminalSnapshot, TerminalMode};

pub struct StardewView {
    renderer: Renderer,
    device: Device,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
}

impl StardewView {
    pub fn new(device: Device, surface: wgpu::Surface, config: wgpu::SurfaceConfiguration) -> Self {
        Self {
            renderer: Renderer::new(device.clone()),
            device,
            surface,
            config,
        }
    }

    pub fn render(&mut self, terminal_state: &alacritty_terminal::Term) {
        self.renderer.render(terminal_state);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn present(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.renderer.draw(&mut encoder, &view);
        self.device.queue().submit(Some(encoder.finish()));
        frame.present();
    }
}
