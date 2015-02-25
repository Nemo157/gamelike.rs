use clock_ticks::precise_time_ns;

use { view, world };

use event::Event;

const TICKS_PER_SECOND: u64 = 25;
const SKIP_TICKS: u64 = 1_000_000_000 / TICKS_PER_SECOND;
const RENDER_TICKS_PER_SECOND: u64 = 60;
const RENDER_SKIP_TICKS: u64 = 1_000_000_000 / RENDER_TICKS_PER_SECOND;
const MAX_FRAMESKIP: u8 = 2;

pub struct App {
    world: world::World,
    view: view::View,
}

impl App {
    pub fn new() -> App {
        let world = world::World::new();
        let view = view::View::new();

        App {
            world: world,
            view: view,
        }
    }

    pub fn run(&mut self) {
        let (mut next_game_tick, mut next_render_tick) = (precise_time_ns(), precise_time_ns());
        let mut should_close = false;
        let (mut horiz, mut vert) = (320, 240);

        while !should_close {
            let mut loops = 0;
            while precise_time_ns() > next_game_tick && loops < MAX_FRAMESKIP {
                for event in self.view.display.poll_events().filter_map(Event::from_glfw) {
                    match event {
                        Event::Quit
                            => should_close = true,
                        Event::MoveCamera(direction, action)
                            => self.view.camera.event(direction, action),
                        Event::TurnCamera(horizontal, vertical)
                            => { horiz = horizontal; vert = vertical; },
                    }
                }
                self.view.camera.turn((horiz - 320) as f64 / (32 as f64), ((vert - 240) as f64) / (24 as f64));
                self.world.tick();
                self.view.tick();
                next_game_tick += SKIP_TICKS;
                loops += 1;
            }
            if precise_time_ns() > next_render_tick {
                let interpolation = (precise_time_ns() + SKIP_TICKS - next_game_tick) as f64 / SKIP_TICKS as f64;
                self.view.render(interpolation, &self.world);
                next_render_tick += RENDER_SKIP_TICKS
            }
        }
    }
}
