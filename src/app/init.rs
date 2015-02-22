use std::error::FromError;

use clock_ticks::precise_time_ns;

use { view, world };

#[derive(Debug)]
pub enum InitError { }

pub fn init() -> Result<super::App, InitError> {
    let world = world::World::new();
    let view = view::View::new(&world);

    Ok(super::App {
        start: precise_time_ns(),
        world: world,
        view: view,
    })
}
