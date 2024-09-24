use hecs::World;
use raylib::prelude::*;

use crate::{
    cmpx,
    components::{
        Attack, BasicPlayerAttack, Boss, CircleHitbox, Controllable, Cooldown, Enemy, Focusable,
        MoveParams, Player, PlayerAttack, PlayerSpells, RotatingBgBoss, Sprite, Transform2D,
        Wanderable,
    },
    vec2,
};

pub fn reimu_a(world: &mut World) {
    world.spawn((
        Player,
        Controllable,
        Sprite::new("reimu_sprite", 0, 0, 32., 32.),
        Transform2D::new(cmpx!(150., 400.), vec2!(1.), 0.),
        MoveParams::move_dampen(cmpx!(0.), 0.85),
        Focusable(0., 300.),
        CircleHitbox::new(4., vec2!(16.)),
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
        CircleHitbox::new(12., vec2!(16., 32.)),
    ));
}
