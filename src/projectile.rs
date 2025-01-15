use raylib::prelude::*;
use crate::{G, TILE_HEIGHT, TILE_WIDTH};
use crate::special_functions::Ξ;

pub const GRAVITY:Vector3 = Vector3::new(0., 0., G);

pub enum ProjectileType {
    Ball
}
pub struct Projectile {
    pub r#type: ProjectileType,
    pub position: Vector3,
    pub velocity: Vector3,
    pub Δt: f32,
    pub elapsed_time: f32,
}

impl Projectile {
    pub fn new(r#type: ProjectileType, r_0:Vector3, r_f:Vector3) -> Projectile {
        let v0:Vector3;
        let Δt:f32;
        match r#type {
            ProjectileType::Ball => {
                Δt = 0.2;
                v0 = Vector3 {
                    x: (r_f.x - r_0.x)/Δt,
                    y: (r_f.y - r_0.y)/Δt,
                    z: (r_f.z - r_0.z)/Δt + 0.5*G*Δt
                };
            },
            _ => { v0 = Vector3::zero(); Δt = 1.; }
        }
        Projectile {
            r#type,
            position: r_0,
            velocity: v0,
            Δt,
            elapsed_time: 0.0,
        }
    }
    pub fn update(&mut self, dt:f32) {
        self.elapsed_time += dt;

        self.velocity.z -= G*dt;
        self.position += self.velocity*dt;
    }

    pub fn draw(&self, dwh: &mut RaylibDrawHandle, zoom: f32, offset: Vector2) {
        let mut p = Ξ(self.position, zoom, offset);
        match self.r#type {
            ProjectileType::Ball => {
                dwh.draw_circle(p.x as i32, p.y as i32, 8., Color::SKYBLUE);
            },
            _ => {}
        }
    }

    pub fn initial_velocity(f:Vector3, Δt:f32) -> Vector3 {
        Vector3 {
            x: f.x/Δt,
            y: f.y/Δt,
            z: f.z/Δt - 0.5*G*Δt
        }
    }
}