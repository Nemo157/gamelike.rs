use glfw::{ Key, WindowEvent };
use glfw::Action::{ Press, Release };
use glfw::WindowEvent::Close;
use glfw::WindowEvent::Key as WKey;

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
            Close                            => Some(Quit),
            WKey(Key::Escape, _, Press,   _) => Some(Quit),
            WKey(Key::A,      _, Press,   _) => Some(Camera(Left, Start)),
            WKey(Key::A,      _, Release, _) => Some(Camera(Left, Stop)),
            WKey(Key::W,      _, Press,   _) => Some(Camera(Forward, Start)),
            WKey(Key::W,      _, Release, _) => Some(Camera(Forward, Stop)),
            WKey(Key::D,      _, Press,   _) => Some(Camera(Right, Start)),
            WKey(Key::D,      _, Release, _) => Some(Camera(Right, Stop)),
            WKey(Key::S,      _, Press,   _) => Some(Camera(Back, Start)),
            WKey(Key::S,      _, Release, _) => Some(Camera(Back, Stop)),
            WKey(Key::Space,  _, Press,   _) => Some(Camera(Up, Start)),
            WKey(Key::Space,  _, Release, _) => Some(Camera(Up, Stop)),
            WKey(Key::Z,      _, Press,   _) => Some(Camera(Down, Start)),
            WKey(Key::Z,      _, Release, _) => Some(Camera(Down, Stop)),
            _ => None,
        }
    }
}

