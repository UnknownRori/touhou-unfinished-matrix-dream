use num_complex::Complex;
use raylib::prelude::*;

use crate::{
    math::ToVec2,
    utility::{get_sprite_coord, timer::Timer},
};

pub struct Player;
pub struct Controllable;
pub struct Enemy;
pub struct Boss;
pub struct Bullet;
pub struct DieOffScreen;

#[derive(Debug, Clone)]
pub struct BeenOnScreen(pub bool);
#[derive(Debug, Clone)]
pub struct Cooldown(pub Timer);

impl Cooldown {
    pub fn new(time: f32) -> Self {
        Self(Timer::new(time, true))
    }
}

pub struct Sprite {
    name: &'static str,
    src: Rectangle,
}

impl Sprite {
    pub fn new(name: &'static str, x: i32, y: i32, width: f32, height: f32) -> Self {
        Self {
            name,
            src: get_sprite_coord(x, y, width, height),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MoveParams {
    pub velocity: Complex<f32>,
    pub acceleration: Complex<f32>,
    pub retention: f32,
    pub attraction: Complex<f32>,
    pub attraction_point: Complex<f32>,
    pub attraction_exponent: f32,
}
impl MoveParams {
    pub fn update(&mut self, pos: &mut Complex<f32>, delta: f32) -> Complex<f32> {
        let orig_velocity = self.velocity;
        *pos += orig_velocity * delta;

        self.velocity = self.acceleration * delta + self.retention * self.velocity;

        if self.attraction.norm() != 0.0 {
            let av = self.attraction_point - *pos;

            if self.attraction_exponent == 1.0 {
                self.velocity += self.attraction * av * delta;
            } else {
                let m = av.norm().powf(self.attraction_exponent - 0.5);
                self.velocity += self.attraction * av * m * delta;
            }
        }

        orig_velocity
    }

    pub fn move_next(
        pos: Complex<f32>,
        mut move_params: MoveParams,
        delta_time: f32,
    ) -> MoveParams {
        move_params.update(&mut Complex::new(pos.re, pos.im), delta_time);
        move_params
    }

    pub fn move_asymptotic(
        vel0: Complex<f32>,
        vel1: Complex<f32>,
        retention: Complex<f32>,
    ) -> MoveParams {
        MoveParams {
            velocity: vel0,
            acceleration: vel1 * (Complex::new(1.0, 0.0) - retention),
            retention: retention.re,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }

    pub fn move_asymptotic_halflife(
        vel0: Complex<f32>,
        vel1: Complex<f32>,
        halflife: f32,
    ) -> MoveParams {
        let retention = Complex::new(2.0_f32.powf(-1.0 / halflife), 0.0);
        Self::move_asymptotic(vel0, vel1, retention)
    }

    pub fn move_asymptotic_simple(vel: Complex<f32>, boost_factor: f32) -> MoveParams {
        let retention = 0.8;
        Self::move_asymptotic(
            vel * (Complex::new(1.0 + boost_factor, 0.0)),
            vel,
            Complex::new(retention, 0.0),
        )
    }

    pub fn move_from_towards(
        origin: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
    ) -> MoveParams {
        let towards_move = Self::move_towards(Complex::new(0.0, 0.0), target, attraction);
        Self::move_next(origin, towards_move, 0.0)
    }

    pub fn move_towards_exp(
        vel: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
        exponent: f32,
    ) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention: 1.0,
            attraction,
            attraction_point: target,
            attraction_exponent: exponent,
        }
    }

    pub fn move_from_towards_exp(
        origin: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
        exponent: f32,
    ) -> MoveParams {
        let towards_exp_move =
            Self::move_towards_exp(Complex::new(0.0, 0.0), target, attraction, exponent);
        Self::move_next(origin, towards_exp_move, 0.0)
    }

    pub fn move_dampen(vel: Complex<f32>, retention: f32) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }

    pub fn move_towards(
        vel: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
    ) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention: 1.0,
            attraction,
            attraction_point: target,
            attraction_exponent: 1.0,
        }
    }

    pub fn move_linear(vel: Complex<f32>) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention: 1.0,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }
    pub fn move_accelerated(vel: Complex<f32>, accel: Complex<f32>) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: accel,
            retention: 1.0,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Transform2D {
    pub position: Complex<f32>,
    pub scale: Vector2,
    pub rotation: f32,
}

impl Transform2D {
    pub fn new(position: Complex<f32>, scale: Vector2, rotation: f32) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }

    pub fn position(&self) -> &Complex<f32> {
        &self.position
    }

    pub fn scale(&self) -> &Vector2 {
        &self.scale
    }

    pub fn rotation(&self) -> &f32 {
        &self.rotation
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CircleHitbox {
    pub radius: f32,
}

impl CircleHitbox {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn is_intersect(
        &self,
        current_pos: &Transform2D,
        target_pos: &Transform2D,
        target_hitbox: &Self,
    ) -> bool {
        check_collision_circles(
            current_pos.position.to_vec2(),
            self.radius,
            target_pos.position.to_vec2(),
            target_hitbox.radius,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hitpoint {
    pub hp: f32,
    pub max_hp: f32,
    pub invulnerable: bool, // INFO : Phase for invulnerable stuff
}

impl Hitpoint {
    pub fn new(hp: f32) -> Self {
        Self {
            hp,
            max_hp: hp,
            invulnerable: false,
        }
    }

    pub fn invulnerable() -> Self {
        Self {
            hp: 1.,
            max_hp: 1.,
            invulnerable: true,
        }
    }

    pub fn is_dead(&self) -> bool {
        if self.invulnerable {
            return false;
        }

        return self.hp < 0.;
    }

    pub fn damage(&mut self, damage: f32) -> bool {
        if !self.invulnerable {
            self.hp -= damage;
            return self.hp <= 0.0;
        }

        false
    }
}
