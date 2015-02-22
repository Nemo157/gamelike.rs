extern crate glium;
extern crate glutin;

use std::default::Default;
use std::num::Float;
use std::sync::mpsc;

use glium::DisplayBuild;
use glium::Surface;

use { world, camera };

#[vertex_format]
#[derive(Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

static VERTEX_SRC: &'static str = "
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
";

static FRAGMENT_SRC: &'static str = "
    #version 150 core

    in vec4 Color;

    out vec4 outColor;

    void main() {
        outColor = Color;
    }
";

pub struct View {
    vertices: glium::vertex::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    pub display: glium::Display,
    program: glium::program::Program,
    pub camera: camera::Camera,
}

impl View {
    pub fn new(world: &world::World) -> View {

        let display = glutin::WindowBuilder::new()
            .with_dimensions(640, 480)
            .with_title(format!("Triangle example."))
            .with_gl_version((4, 1))
            .build_glium()
            .unwrap();

        let program = glium::Program::from_source(&display, VERTEX_SRC.clone(), FRAGMENT_SRC.clone(), None).unwrap();

        let vertices = glium::VertexBuffer::new(&display, vec![
            Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
        ]);
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let camera = camera::Camera::new(640, 480);

        View {
            vertices: vertices,
            indices: indices,
            display: display,
            program: program,
            camera: camera,
        }
    }

    pub fn tick(&mut self) {
        self.camera.tick();
    }

    pub fn render(&mut self, interpolation: f64, world: &world::World) {
        let uniforms = uniform! {
            model: [ [1.0, 0.0, 0.0, 0.0],
                     [0.0, 1.0, 0.0, 0.0],
                     [0.0, 0.0, 1.0, 0.0],
                     [0.0, 0.0, 0.0, 1.0] ],
            view: self.camera.view(interpolation),
            proj: self.camera.proj
        };

        let mut target = self.display.draw();

        target.clear_color(Float::abs(((world.count % 100) as f32) / 50.0 - 1.0), 0.3, 0.3, 1.0);
        target.draw(&self.vertices, &self.indices, &self.program, &uniforms, &Default::default()).unwrap();
        target.finish();
    }
}
