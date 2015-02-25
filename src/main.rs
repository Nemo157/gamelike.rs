#![feature(core)]
#![feature(plugin)]
#![feature(std_misc)]

#![plugin(glium)]
#![plugin(glium_macros)]

#[macro_use] extern crate glium;
extern crate glutin;
extern crate clock_ticks;
extern crate nalgebra;

mod app;
mod view;
mod world;
mod event;
mod camera;

fn main() {
    let mut app = app::App::new();
    app.run();
}
