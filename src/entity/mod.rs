use std::collections::VecDeque;

use hecs::World;
use raylib::prelude::*;

use crate::{
    cmpx,
    components::{
        Attack, AttackMove, BasicPlayerAttack, Boss, BossMove, BossMoves, Bullet, BulletSetup,
        CircleHitbox, Controllable, Cooldown, DieOffScreen, Enemy, Focusable, Hitpoint, MoveParams,
        Player, PlayerAttack, PlayerSpells, RotatingBgBoss, Sprite, Transform2D, Wanderable,
    },
    utility::timer::Timer,
    vec2,
};
pub fn create_enemy_bullet(
    world: &mut World,
    transform: Transform2D,
    sprite: Sprite,
    movement: MoveParams,
    hitbox: CircleHitbox,
) {
    world.spawn((
        Enemy,
        Bullet,
        DieOffScreen,
        movement,
        transform,
        sprite,
        hitbox,
    ));
}

pub fn reimu_a(world: &mut World) {
    world.spawn((
        Player,
        Controllable,
        Sprite::new("reimu_sprite", 0, 0, 32., 32.),
        Transform2D::new(cmpx!(150., 400.), vec2!(1.), 0.),
        MoveParams::move_dampen(cmpx!(0.), 0.85),
        Focusable(0., 300.),
        CircleHitbox::new(4.5, vec2!(0.)),
        PlayerAttack::new(
            Attack::new(Cooldown::new(1.), BasicPlayerAttack::ReimuA),
            Attack::new(Cooldown::new(1.), PlayerSpells::ReimuA),
        ),
    ));
}

pub fn miko(world: &mut World) {
    world.spawn((
        Enemy,
        Boss,
        RotatingBgBoss(0., 300.),
        Sprite::new("miko_sprite", 0, 0, 32., 64.),
        Transform2D::new(cmpx!(150., 50.), vec2!(1.), 0.),
        Wanderable::new(
            Rectangle::new(10., 10., 364., 200.),
            cmpx!(150., 50.),
            400.,
            4.,
        ),
        MoveParams::move_linear(cmpx!(0.)),
        CircleHitbox::new(12., vec2!(0.)),
        BossMoves(VecDeque::from([
            BossMove::NonSpells {
                timeout: Timer::new(120., false),
                hp: Hitpoint::new(25000.),
                attack: AttackMove::Multiple(Vec::from([
                    AttackMove::Circle {
                        sides: 32,
                        speed: 140.,
                        rotation_per_fire: 2.,
                        rotation: 12.,
                        cooldown: Cooldown(Timer::new(1.9, true)),
                        setup: BulletSetup(
                            Sprite::new("miko_sprite", 0, 3, 32., 32.),
                            CircleHitbox::new(2.5, vec2!(0.)),
                        ),
                    },
                    AttackMove::Circle {
                        sides: 24,
                        speed: 120.,
                        rotation_per_fire: 2.,
                        rotation: 8.,
                        cooldown: Cooldown(Timer::new(1.0, true)),
                        setup: BulletSetup(
                            Sprite::new("miko_sprite", 0, 3, 32., 32.),
                            CircleHitbox::new(2.5, vec2!(0.)),
                        ),
                    },
                    AttackMove::Circle {
                        sides: 24,
                        speed: 150.,
                        rotation_per_fire: 2.,
                        rotation: 8.,
                        cooldown: Cooldown(Timer::new(2.0, true)),
                        setup: BulletSetup(
                            Sprite::new("miko_sprite", 0, 3, 32., 32.),
                            CircleHitbox::new(2.5, vec2!(0.)),
                        ),
                    },
                    AttackMove::AtPlayer {
                        num: 12,
                        speed: 145.,
                        spread: 12.,
                        total_shoot: 12,
                        cooldown: Cooldown(Timer::new(2.0, true)),
                        setup: BulletSetup(
                            Sprite::new("miko_sprite", 0, 4, 32., 32.),
                            CircleHitbox::new(2.5, vec2!(0.)),
                        ),
                    },
                ])),
            },
            BossMove::Spells {
                name: "Hermit Sign 'Taoist of the Land of the Rising Sun'".to_owned(),
                timeout: Timer::new(240., false),
                hp: Hitpoint::new(50000.),
                attack: AttackMove::Multiple(
                    [
                        AttackMove::Circle {
                            sides: 32,
                            speed: 240.,
                            rotation_per_fire: 2.,
                            rotation: 12.,
                            cooldown: Cooldown(Timer::new(1.9, true)),
                            setup: BulletSetup(
                                Sprite::new("miko_sprite", 0, 3, 32., 32.),
                                CircleHitbox::new(2.5, vec2!(0.)),
                            ),
                        },
                        AttackMove::Circle {
                            sides: 24,
                            speed: 220.,
                            rotation_per_fire: 2.,
                            rotation: 8.,
                            cooldown: Cooldown(Timer::new(1.0, true)),
                            setup: BulletSetup(
                                Sprite::new("miko_sprite", 0, 3, 32., 32.),
                                CircleHitbox::new(2.5, vec2!(0.)),
                            ),
                        },
                        AttackMove::Circle {
                            sides: 24,
                            speed: 250.,
                            rotation_per_fire: 2.,
                            rotation: 8.,
                            cooldown: Cooldown(Timer::new(2.0, true)),
                            setup: BulletSetup(
                                Sprite::new("miko_sprite", 0, 3, 32., 32.),
                                CircleHitbox::new(2.5, vec2!(0.)),
                            ),
                        },
                        AttackMove::Circle {
                            sides: 12,
                            speed: 240.,
                            rotation_per_fire: 2.,
                            rotation: 3.,
                            cooldown: Cooldown(Timer::new(2.5, true)),
                            setup: BulletSetup(
                                Sprite::new("miko_sprite", 0, 4, 32., 32.),
                                CircleHitbox::new(2.5, vec2!(0.)),
                            ),
                        },
                        AttackMove::AtPlayer {
                            num: 24,
                            speed: 250.,
                            spread: 20.,
                            total_shoot: 12,
                            cooldown: Cooldown(Timer::new(5., true)),
                            setup: BulletSetup(
                                Sprite::new("reimu_sprite", 0, 6, 64., 64.),
                                CircleHitbox::new(10., vec2!(0.)),
                            ),
                        },
                    ]
                    .to_vec(),
                ),
            },
        ])),
    ));
}
