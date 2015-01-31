use std::error::FromError;

use glfw;
use clock_ticks::precise_time_ns;

use { view, world };

#[derive(Debug)]
pub enum InitError {
    GlfwError(glfw::InitError)
}

impl FromError<glfw::InitError> for InitError {
    fn from_error(err: glfw::InitError) -> Self {
        InitError::GlfwError(err)
    }
}

pub fn init() -> Result<super::App, InitError> {
    let mut glfw = try!(glfw::init(glfw::FAIL_ON_ERRORS));

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));

    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);

    let world = world::World::new();
    let view = view::View::new(&glfw, &world);

    Ok(super::App {
        start: precise_time_ns(),
        glfw: glfw,
        world: world,
        view: view,
    })
}
