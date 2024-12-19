use raylib::prelude::*;
use crate::special_functions::{Ξ, ξ, to_tile_coords, ΞM, ξM};
use crate::{TILE_WIDTH, TILE_HEIGHT};

pub struct Player {
    /// Identifier
    name: String,
    /// Characteristics
    health: u8,
        // hand_item
        // inventory
    speed: f32,
    /// World
    position: Vector3,
    pub velocity: Vector3,
}
impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: String::from(name),
            health: 20,
            speed: 8.,
            position: Vector3::new(0., 0., 0.),
            velocity: Vector3::zero()
        }
    }
    pub fn update(&mut self, dt: f32) {
        //let v = ξM(Vector2::new(self.velocity.x, self.velocity.y).normalized());
        //let v = Vector3::new(v.x, v.y, self.velocity.z);
        let v = Vector2::new(self.velocity.x, self.velocity.y).normalized() * self.speed;
        self.position += Vector3::new(v.x, v.y, self.velocity.z) * dt;
    }
    pub fn draw(&mut self, dwh: &mut RaylibDrawHandle, zoom: f32, offset: Vector2) {
        let p = Ξ(self.position, zoom, offset);
        dwh.draw_rectangle(p.x as i32, p.y as i32, TILE_WIDTH, 2*TILE_HEIGHT, Color::WHITE);
    }
}