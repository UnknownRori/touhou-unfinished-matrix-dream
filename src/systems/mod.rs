use hecs::World;
use raylib::prelude::*;

use crate::{
    cmpx,
    components::{
        BeenOnScreen, CircleHitbox, Controllable, DieOffScreen, Focusable, MoveParams,
        PlayerAttack, Sprite, Transform2D,
    },
    controls::Action,
    math::ToVec2,
    state::State,
    utility::get_sprite_coord,
};

pub fn draw_sprites_system(
    world: &World,
    state: &State,
    d: &mut RaylibMode2D<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
) {
    world
        .query::<(&Sprite, &Transform2D)>()
        .iter()
        .for_each(|(_, (s, t))| {
            d.draw_texture_pro(
                &state.assets.get(s.name),
                s.src,
                Rectangle::new(t.position().re, t.position().im, s.src.width, s.src.height),
                Vector2::new(0., 0.),
                t.rotation,
                Color::WHITE,
            );
        });
}

pub fn update_movement(world: &World, d: &RaylibDrawHandle<'_>) {
    world
        .query::<(&mut Transform2D, &mut MoveParams)>()
        .iter()
        .for_each(|(_, (transform, move_params))| {
            move_params.update(&mut transform.position, d.get_frame_time());
        });
}

pub fn draw_circle_hitbox(
    world: &World,
    d: &mut RaylibMode2D<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
) {
    world
        .query::<(&Transform2D, &CircleHitbox)>()
        .iter()
        .for_each(|(_, (t, h))| {
            d.draw_circle(
                t.position().re as i32 + h.offset.x as i32,
                t.position().im as i32 + h.offset.y as i32,
                h.radius,
                Color::new(255, 0, 0, 128),
            );
        });
}

pub fn rotate_focus(world: &World, d: &RaylibDrawHandle<'_>) {
    world.query::<&mut Focusable>().iter().for_each(|(_, f)| {
        f.0 += f.1 * d.get_frame_time();
    })
}

pub fn draw_focus(
    world: &World,
    state: &State,
    d: &mut RaylibMode2D<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
) {
    world
        .query::<(&Controllable, &Transform2D, &Focusable)>()
        .iter()
        .for_each(|(_, (_, t, f))| {
            if state.controls.is_down(Action::Focus, d) {
                d.draw_texture_pro(
                    &state.assets.get("commons_sprite"),
                    get_sprite_coord(0, 0, 32., 32.),
                    Rectangle::new(t.position().re + 15., t.position().im + 15., 32., 32.),
                    Vector2::new(16., 16.),
                    f.0,
                    Color::WHITE,
                );
            }
        });
}

pub fn delete_offscreen(world: &mut World) {
    let pending = world
        .query::<(&Transform2D, &DieOffScreen, Option<&BeenOnScreen>)>()
        .iter()
        .map(|(id, (transfrom, _, been_on_screen))| {
            (
                id.clone(),
                transfrom.clone(),
                match been_on_screen {
                    Some(a) => a.clone(),
                    None => BeenOnScreen(false),
                },
            )
        })
        .collect::<Vec<_>>();

    let container = Rectangle::new(-40., -40., 400., 488.);
    for i in pending {
        if container.check_collision_point_rec(i.1.position.to_vec2()) {
            let _ = world.insert_one(i.0, BeenOnScreen(true));
        } else if i.2 .0 {
            let _ = world.despawn(i.0);
        }
    }
}

pub fn player_control(world: &mut World, state: &mut State, d: &RaylibDrawHandle<'_>) {
    let mut pending = Vec::new();

    world
        .query::<(
            &Controllable,
            &Transform2D,
            &mut MoveParams,
            &mut PlayerAttack,
        )>()
        .iter()
        .for_each(|(_, (_, t, m, a))| {
            let mut new_pos = cmpx!(0.);
            let move_speed = 5000.; // TODO : Make sure this specific to char
            if state.controls.is_down(Action::Down, d) && t.position().im < 448. - 32. {
                new_pos += cmpx!(0., move_speed);
            }

            if state.controls.is_down(Action::Up, d) && t.position().im > 0. {
                new_pos += cmpx!(0., -move_speed);
            }

            if state.controls.is_down(Action::Left, d) && t.position().re > 0. {
                new_pos += cmpx!(-move_speed, 0.);
            }

            if state.controls.is_down(Action::Right, d) && t.position().re < 384. - 32. {
                new_pos += cmpx!(move_speed, 0.);
            }

            let move_speed = if state.controls.is_down(Action::Focus, d) {
                1. / 2.6 // Specific to char
            } else {
                1.
            };

            m.acceleration = new_pos * move_speed;

            if state.controls.is_down(Action::Attack, d) {
                a.basic.0 .0.update(d.get_frame_time());
                pending.push(a.basic.1.spawn(*t.position()));
            }
        });

    for i in pending {
        (i)(world, state);
    }
}
