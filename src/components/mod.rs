use hecs::World;
use num_complex::Complex;
use raylib::prelude::*;

use crate::{
    cmpx,
    math::ToVec2,
    state::State,
    utility::{get_sprite_coord, timer::Timer},
    vec2,
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

pub struct Focusable(pub f32, pub f32);
pub struct RotatingBgBoss(pub f32, pub f32);

impl Cooldown {
    pub fn new(time: f32) -> Self {
        Self(Timer::new(time, true))
    }
}

pub struct Wanderable {
    pub wander_size: Rectangle,
    pub last_pos: Complex<f32>,
    pub target_pos: Option<Complex<f32>>,
    pub elapsed: f32,
    pub wait: f32,
    pub speed: f32,
}

impl Wanderable {
    pub fn new(wander_size: Rectangle, last_pos: Complex<f32>, speed: f32, wait: f32) -> Self {
        Self {
            wander_size,
            wait,
            elapsed: 0.,
            last_pos,
            target_pos: None,
            speed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BulletSetup(pub Sprite);

#[derive(Debug, Clone)]
pub enum AttackMove {
    AtPlayer {
        num: u16,
        speed: f32,
        spread: f32,
        total_shoot: u16,
        cooldown: Cooldown,
        setup: BulletSetup,
    },
    Circle {
        sides: u16,
        rotation_per_fire: f32,
        rotation: f32,
        cooldown: Cooldown,
        setup: BulletSetup,
    },
    Multiple(Vec<AttackMove>),
}

pub enum BasicPlayerAttack {
    ReimuA,
}

impl BasicPlayerAttack {
    pub fn spawn(&self, pos: Complex<f32>) -> impl FnOnce(&mut World, &mut State) {
        move |world, state| {
            world.spawn((
                Player,
                Bullet,
                DieOffScreen,
                Transform2D {
                    position: pos,
                    scale: vec2!(1.),
                    rotation: 0.,
                },
                Sprite::new("reimu_sprite", 0, 4, 32., 32.),
                MoveParams::move_linear(cmpx!(0., -5000.)),
                CircleHitbox::new(2., vec2!(16.)),
            ));
        }
    }
}

pub enum PlayerSpells {
    ReimuA,
}

pub struct Attack<T: Send + Sync>(pub Cooldown, pub T);

impl<T: Send + Sync> Attack<T> {
    pub fn new(cooldown: Cooldown, t: T) -> Self {
        Self(cooldown, t)
    }
}

pub struct PlayerAttack {
    pub basic: Attack<BasicPlayerAttack>,
    pub spells: Attack<PlayerSpells>,
}

impl PlayerAttack {
    pub fn new(basic: Attack<BasicPlayerAttack>, spells: Attack<PlayerSpells>) -> Self {
        Self { basic, spells }
    }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub name: &'static str,
    pub src: Rectangle,
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
    pub offset: Vector2,
}

impl CircleHitbox {
    pub fn new(radius: f32, offset: Vector2) -> Self {
        Self { radius, offset }
    }

    pub fn is_intersect(
        &self,
        current_pos: &Transform2D,
        target_pos: &Transform2D,
        target_hitbox: &Self,
    ) -> bool {
        let self_pos = current_pos.position.to_vec2();
        let tgt_pos = target_pos.position.to_vec2();
        check_collision_circles(
            Vector2::new(self_pos.x + self.offset.x, self_pos.y + self.offset.y),
            self.radius,
            Vector2::new(
                tgt_pos.x + target_hitbox.offset.x,
                tgt_pos.y + target_hitbox.offset.y,
            ),
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
