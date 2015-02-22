#![feature(core)]
#![feature(plugin)]

#![plugin(glium)]
#![plugin(glium_macros)]

#[macro_use] extern crate glium;
extern crate glutin;
extern crate clock_ticks;
extern crate nalgebra;
#[macro_use] extern crate glium_macros;

mod app;
mod view;
mod world;
mod event;
mod camera;

fn main() {
    let mut app = app::init().unwrap();
    app.run();
}
