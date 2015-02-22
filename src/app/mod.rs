use clock_ticks::precise_time_ns;

use { view, world, camera };

use event::Event;

pub use self::init::init;

mod init;

const TICKS_PER_SECOND: u64 = 25;
const SKIP_TICKS: u64 = 1000_000_000 / TICKS_PER_SECOND;
const MAX_FRAMESKIP: u8 = 2;

pub struct App {
    start: u64,
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
                for event in self.view.display.poll_events().filter_map(Event::from_glfw) {
                    match event {
                        Event::Quit
                            => should_close = true,
                        Event::Camera(direction, action)
                            => self.view.camera.event(direction, action),
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
