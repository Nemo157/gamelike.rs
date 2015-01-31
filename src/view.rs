extern crate gfx;
extern crate glfw;
extern crate device;
extern crate render;

use std::num::Float;
use std::sync::mpsc;

use gfx::{ DeviceHelper, ToSlice };
use glfw::Context;

use { world, camera };

#[vertex_format]
#[derive(Copy)]
struct Vertex {
    #[name = "position"]
    pos: [f32; 2],

    #[name = "color"]
    color: [f32; 3],
}

#[shader_param]
struct MyParam {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
}

static VERTEX_SRC: gfx::ShaderSource<'static> = shaders! {
glsl_150: b"
    #version 150 core

    in vec2 position;
    in vec3 color;

    out vec4 Color;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 proj;

    void main() {
        Color = vec4(color, 1.0);
        gl_Position = proj * view * model * vec4(position, 0.0, 1.0);
    }
"
};

static FRAGMENT_SRC: gfx::ShaderSource<'static> = shaders! {
glsl_150: b"
    #version 150 core

    in vec4 Color;

    out vec4 outColor;

    void main() {
        outColor = Color;
    }
"
};

pub struct View {
    graphics: gfx::Graphics<gfx::GlDevice>,
    pub window: glfw::Window,
    frame: render::target::Frame,
    batch: render::batch::RefBatch<MyParam>,
    pub events: mpsc::Receiver<(f64, glfw::WindowEvent)>,
    pub camera: camera::Camera,
}

impl View {
    pub fn new(glfw: &glfw::Glfw, world: &world::World) -> View {
        let (mut window, events) = glfw
            .create_window(640, 480, "Triangle example.", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_all_polling(true);

        let (w, h) = window.get_framebuffer_size();
        let frame = gfx::Frame::new(w as u16, h as u16);

        let mut device = gfx::GlDevice::new(|s| window.get_proc_address(s));

        let program = device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone())
                            .unwrap();

        let vertex_data = [
            Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
            Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
            Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] },
        ];
        let mesh = device.create_mesh(&vertex_data);
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);

        let mut graphics = gfx::Graphics::new(device);
        let batch: gfx::batch::RefBatch<MyParam> = graphics.make_batch(
            &program, &mesh, slice, &gfx::DrawState::new()).unwrap();

        let camera = camera::Camera::new(w as u16, h as u16);

        View {
            graphics: graphics,
            window: window,
            frame: frame,
            batch: batch,
            events: events,
            camera: camera,
        }
    }

    pub fn tick(&mut self) {
        self.camera.tick();
    }

    pub fn render(&mut self, interpolation: f64, world: &world::World) {
        let clear_data = gfx::ClearData {
            color: [Float::abs(((world.count % 100) as f32) / 50.0 - 1.0), 0.3, 0.3, 1.0],
            depth: 1.0,
            stencil: 0,
        };

        let data = MyParam {
            model: [ [1.0, 0.0, 0.0, 0.0],
                     [0.0, 1.0, 0.0, 0.0],
                     [0.0, 0.0, 1.0, 0.0],
                     [0.0, 0.0, 0.0, 1.0] ],
            view: self.camera.view(interpolation),
            proj: self.camera.proj,
        };

        self.graphics.clear(clear_data, gfx::COLOR, &self.frame);
        self.graphics.draw(&self.batch, &data, &self.frame);
        self.graphics.end_frame();
        self.window.swap_buffers();
    }
}
