#![feature(core)]
#![feature(plugin)]

extern crate gfx;
extern crate glfw;
extern crate clock_ticks;
extern crate nalgebra;
#[macro_use] #[plugin] extern crate gfx_macros;

mod app;
mod view;
mod world;
mod event;
mod camera;

fn main() {
    let mut app = app::init().unwrap();
    app.run();
}
