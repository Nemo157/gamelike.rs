use std::f64::consts::{ PI, PI_2 };
use std::num::Float;
use std::collections::HashSet;

use nalgebra::{ Vec3, Pnt3, Cross, Norm };

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum Direction {
    Forward,
    Back,
    Right,
    Left,
    Up,
    Down,
}

pub struct Camera {
    position: Pnt3<f64>,
    lookat: Pnt3<f64>,
    viewup: Vec3<f64>,
    sensitivity: f64,
    speed: f64,

    pub proj: [[f32; 4]; 4],

    moving: HashSet<Direction>,
    to_move: HashSet<Direction>,
    to_stop: HashSet<Direction>,
}

impl Camera {
    pub fn new(width: u16, height: u16) -> Self {
        Camera {
            position: Pnt3::new(0.0, 0.0, 1.0),
            lookat: Pnt3::new(0.0, 0.0, 0.0),
            viewup: Vec3::new(0.0, 1.0, 0.0),
            sensitivity: 0.001,
            speed: 0.1,
            proj: calc_proj(
                50.0,
                width as f64 / height as f64,
                0.1,
                100.0),

            moving: HashSet::new(),
            to_move: HashSet::new(),
            to_stop: HashSet::new(),
        }
    }

    pub fn start_move(&mut self, direction: Direction) {
        self.to_move.insert(direction);
    }

    pub fn stop_move(&mut self, direction: Direction) {
        self.to_stop.insert(direction);
    }

    pub fn turn(&mut self, horizontal: f64, vertical: f64) {
        unimplemented!();
    }

    pub fn view(&self, interpolation: f64) -> [[f32; 4]; 4] {
        let facing = (self.lookat - self.position).normalize_cpy();
        let side = facing.cross(&self.viewup).normalize_cpy();
        let up = side.cross(&facing).normalize_cpy();

        let position = self.position + self.movement(interpolation);

        [ [     side.x as f32,        up.x as f32,     -facing.x as f32, 0.0],
          [     side.y as f32,        up.y as f32,     -facing.y as f32, 0.0],
          [     side.z as f32,        up.z as f32,     -facing.z as f32, 0.0],
          [-position.x as f32, -position.y as f32, -position.z as f32, 1.0] ]
    }

    pub fn tick(&mut self) {
        let movement = self.movement(1.0);

        self.position = self.position + movement;
        self.lookat = self.lookat + movement;

        for direction in self.to_move.drain() {
            self.moving.insert(direction);
        }
        for direction in self.to_stop.drain() {
            self.moving.remove(&direction);
        }
    }

    fn forward(&self, amount: f64) -> Vec3<f64> {
        let mut facing = self.lookat - self.position;
        facing.y = 0.0;
        facing.normalize_cpy() * amount
    }

    fn right(&self, amount: f64) -> Vec3<f64> {
        self.forward(1.0).cross(&self.viewup).normalize_cpy() * amount
    }

    fn up(&self, amount: f64) -> Vec3<f64> {
        self.viewup * amount
    }

    fn movement(&self, interpolation: f64) -> Vec3<f64> {
        let mut movement = Vec3::new(0.0, 0.0, 0.0);
        for direction in self.moving.iter() {
            movement = movement + match direction {
                &Direction::Forward => self.forward(self.speed * interpolation),
                &Direction::Back => self.forward(-self.speed * interpolation),
                &Direction::Right => self.right(self.speed * interpolation),
                &Direction::Left => self.right(-self.speed * interpolation),
                &Direction::Up => self.up(self.speed * interpolation),
                &Direction::Down => self.up(-self.speed * interpolation),
            }
        }
        movement
    }
}

fn calc_proj(fov: f64, aspect: f64, near: f64, far: f64) -> [[f32; 4]; 4] {
    let f = -Float::tan(PI_2 - fov * (PI  / 180.0) / 2.0);

    let a = f / aspect;
    let b = -(far + near) / (near - far);
    let c = -(2.0 * far * near) / (near - far);

    [ [a as f32,      0.0,      0.0,  0.0],
      [     0.0, f as f32,      0.0,  0.0],
      [     0.0,      0.0, b as f32,  -1.0],
      [     0.0,      0.0, c as f32,  0.0] ]
}

