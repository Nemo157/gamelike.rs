use glfw;
use clock_ticks::precise_time_ns;

use { view, world, camera };

pub use self::init::init;

mod init;

const TICKS_PER_SECOND: u64 = 25;
const SKIP_TICKS: u64 = 1000_000_000 / TICKS_PER_SECOND;
const MAX_FRAMESKIP: u8 = 2;

pub struct App {
    start: u64,
    glfw: glfw::Glfw,
    world: world::World,
    view: view::View,
}

impl App {
    pub fn run(&mut self) {
        let mut next_game_tick = precise_time_ns();
        let mut should_close = false;

        while !should_close {
            let mut loops = 0;
            while precise_time_ns() > next_game_tick && loops < MAX_FRAMESKIP {
                self.glfw.poll_events();
                for (_, event) in glfw::flush_messages(&self.view.events) {
                    match event {
                        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => should_close = true,
                        glfw::WindowEvent::Close => should_close = true,
                        glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Press, _) =>
                            self.view.camera.start_move(camera::Direction::Left),
                        glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Release, _) =>
                            self.view.camera.stop_move(camera::Direction::Left),
                        glfw::WindowEvent::Key(glfw::Key::W, _, glfw::Action::Press, _) =>
                            self.view.camera.start_move(camera::Direction::Forward),
                        glfw::WindowEvent::Key(glfw::Key::W, _, glfw::Action::Release, _) =>
                            self.view.camera.stop_move(camera::Direction::Forward),
                        glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Press, _) =>
                            self.view.camera.start_move(camera::Direction::Right),
                        glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Release, _) =>
                            self.view.camera.stop_move(camera::Direction::Right),
                        glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Press, _) =>
                            self.view.camera.start_move(camera::Direction::Back),
                        glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Release, _) =>
                            self.view.camera.stop_move(camera::Direction::Back),
                        glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) =>
                            self.view.camera.start_move(camera::Direction::Up),
                        glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Release, _) =>
                            self.view.camera.stop_move(camera::Direction::Up),
                        glfw::WindowEvent::Key(glfw::Key::Z, _, glfw::Action::Press, _) =>
                            self.view.camera.start_move(camera::Direction::Down),
                        glfw::WindowEvent::Key(glfw::Key::Z, _, glfw::Action::Release, _) =>
                            self.view.camera.stop_move(camera::Direction::Down),
                        _ => {},
                    }
                }
                self.world.tick();
                self.view.tick();
                next_game_tick += SKIP_TICKS;
                loops += 1;
            }
            let interpolation = (precise_time_ns() + SKIP_TICKS - next_game_tick) as f64 / SKIP_TICKS as f64;
            self.view.render(interpolation, &self.world);
        }
    }
}
