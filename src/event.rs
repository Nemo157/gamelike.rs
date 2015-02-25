extern crate glutin;

use glutin::Event::{ Closed, KeyboardInput, MouseMoved };
use glutin::ElementState::{ Pressed, Released };
use glutin::VirtualKeyCode as Key;

use camera::{ Action, Direction };
use camera::Action::{ Start, Stop };
use camera::Direction::{ Left, Forward, Right, Back, Up, Down };

use self::Event::{ MoveCamera, TurnCamera, Quit };

pub enum Event {
    MoveCamera(Direction, Action),
    TurnCamera(i32, i32),
    Quit,
}

impl Event {
    pub fn from_glfw(event: glutin::Event) -> Option<Event> {
        println!("{:?}", event);
        match event {
            Closed                                        => Some(Quit),
            KeyboardInput(Pressed,  _, Some(Key::Escape)) => Some(Quit),
            KeyboardInput(Pressed,  _, Some(Key::A))      => Some(MoveCamera(Left, Start)),
            KeyboardInput(Released, _, Some(Key::A))      => Some(MoveCamera(Left, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::W))      => Some(MoveCamera(Forward, Start)),
            KeyboardInput(Released, _, Some(Key::W))      => Some(MoveCamera(Forward, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::D))      => Some(MoveCamera(Right, Start)),
            KeyboardInput(Released, _, Some(Key::D))      => Some(MoveCamera(Right, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::S))      => Some(MoveCamera(Back, Start)),
            KeyboardInput(Released, _, Some(Key::S))      => Some(MoveCamera(Back, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::Space))  => Some(MoveCamera(Up, Start)),
            KeyboardInput(Released, _, Some(Key::Space))  => Some(MoveCamera(Up, Stop)),
            KeyboardInput(Pressed,  _, Some(Key::Z))      => Some(MoveCamera(Down, Start)),
            KeyboardInput(Released, _, Some(Key::Z))      => Some(MoveCamera(Down, Stop)),
            MouseMoved((x, y))                            => Some(TurnCamera(x, y)),
            _ => None,
        }
    }
}

