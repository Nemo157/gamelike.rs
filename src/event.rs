extern crate glutin;

use glutin::Event::{ Closed, KeyboardInput };
use glutin::ElementState::{ Pressed, Released };
use glutin::VirtualKeyCode as Key;

use camera::{ Action, Direction };
use camera::Action::{ Start, Stop };
use camera::Direction::{ Left, Forward, Right, Back, Up, Down };

use self::Event::{ Camera, Quit };

pub enum Event {
    Camera(Direction, Action),
    Quit,
}

impl Event {
    pub fn from_glfw(event: glutin::Event) -> Option<Event> {
        println!("{:?}", event);
        match event {
            Closed                                        => Some(Quit),
            KeyboardInput(Pressed,  _, Some(Key::Escape)) => Some(Quit),
            KeyboardInput(Pressed,  _, Some(Key::A))      => Some(Camera(Left, Start)),
            KeyboardInput(Released, _, Some(Key::A))      => Some(Camera(Left, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::W))      => Some(Camera(Forward, Start)),
            KeyboardInput(Released, _, Some(Key::W))      => Some(Camera(Forward, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::D))      => Some(Camera(Right, Start)),
            KeyboardInput(Released, _, Some(Key::D))      => Some(Camera(Right, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::S))      => Some(Camera(Back, Start)),
            KeyboardInput(Released, _, Some(Key::S))      => Some(Camera(Back, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::Space))  => Some(Camera(Up, Start)),
            KeyboardInput(Released, _, Some(Key::Space))  => Some(Camera(Up, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::Z))      => Some(Camera(Down, Start)),
            KeyboardInput(Released, _, Some(Key::Z))      => Some(Camera(Down, Stop)),
            _ => None,
        }
    }
}

