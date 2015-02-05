use glfw::{ Key, WindowEvent };
use glfw::Action::{ Press, Release };
use glfw::WindowEvent::Close;
use glfw::WindowEvent::Key;

use camera::{ Action, Direction };
use camera::Action::{ Start, Stop };
use camera::Direction::{ Left, Forward, Right, Back, Up, Down };

use self::Event::{ Camera, Quit };

pub enum Event {
    Camera(Direction, Action),
    Quit,
}

impl Event {
    pub fn from_glfw(event: WindowEvent) -> Option<Event> {
        match event {
            Close                           => Some(Quit),
            Key(Key::Escape, _, Press,   _) => Some(Quit),
            Key(Key::A,      _, Press,   _) => Some(Camera(Left, Start)),
            Key(Key::A,      _, Release, _) => Some(Camera(Left, Stop)),
            Key(Key::W,      _, Press,   _) => Some(Camera(Forward, Start)),
            Key(Key::W,      _, Release, _) => Some(Camera(Forward, Stop)),
            Key(Key::D,      _, Press,   _) => Some(Camera(Right, Start)),
            Key(Key::D,      _, Release, _) => Some(Camera(Right, Stop)),
            Key(Key::S,      _, Press,   _) => Some(Camera(Back, Start)),
            Key(Key::S,      _, Release, _) => Some(Camera(Back, Stop)),
            Key(Key::Space,  _, Press,   _) => Some(Camera(Up, Start)),
            Key(Key::Space,  _, Release, _) => Some(Camera(Up, Stop)),
            Key(Key::Z,      _, Press,   _) => Some(Camera(Down, Start)),
            Key(Key::Z,      _, Release, _) => Some(Camera(Down, Stop)),
            _ => None,
        }
    }
}

