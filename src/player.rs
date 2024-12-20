use raylib::prelude::*;
use crate::special_functions::{Ξ, ξ, to_tile_coords, ΞM, ξM};
use crate::{TILE_WIDTH, TILE_HEIGHT};

// Directions
pub const MOVE_UP:Vector3 = Vector3::new(-1., -1., 0.);
pub const MOVE_DOWN:Vector3 = Vector3::new(1., 1., 0.);
pub const MOVE_RIGHT:Vector3 = Vector3::new(1., -1., 0.);
pub const MOVE_LEFT:Vector3 = Vector3::new(-1., 1., 0.);

#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

// Player
pub struct Player {
    /// Identifier
    name: String,
    /// Characteristics
    health: u8,
        // hand_item
        // inventory
    speed: f32,
    /// World
    pub position: Vector3,
    pub velocity: Vector3,
}
impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: String::from(name),
            health: 20,
            speed: 8.,
            position: Vector3::new(2., 2., 0.),
            velocity: Vector3::zero()
        }
    }
    pub fn update(&mut self, dt: f32) {
        let v = Vector2::new(self.velocity.x, self.velocity.y).normalized() * self.speed;
        self.position += Vector3::new(v.x, v.y, self.velocity.z) * dt;
    }
    pub fn draw(&mut self, dwh: &mut RaylibDrawHandle, zoom: f32, offset: Vector2) {
        let mut p = Ξ(self.position - Vector3::new(0., 1., 0.), zoom, offset);
        // Changing the anchor point of the drawing to bottom center
        dwh.draw_rectangle(p.x as i32, p.y as i32 - 2*TILE_HEIGHT, TILE_WIDTH, 2*TILE_HEIGHT, Color::WHITE);
    }
}