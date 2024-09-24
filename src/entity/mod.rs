use hecs::World;

use crate::{
    cmpx,
    components::{
        Attack, BasicPlayerAttack, CircleHitbox, Controllable, Cooldown, Focusable, MoveParams,
        Player, PlayerAttack, PlayerSpells, Sprite, Transform2D,
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
