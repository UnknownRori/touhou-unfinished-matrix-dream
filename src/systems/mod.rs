use hecs::World;
use raylib::prelude::*;

use crate::{
    cmpx,
    components::{
        BeenOnScreen, Boss, CircleHitbox, Controllable, DieOffScreen, Focusable, MoveParams,
        PlayerAttack, RotatingBgBoss, Sprite, Transform2D, Wanderable,
    },
    controls::Action,
    math::{ComplexExt, ToVec2},
    state::State,
    utility::get_sprite_coord,
    vec2,
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

pub fn draw_boss_bg(
    world: &World,
    state: &State,
    d: &mut RaylibMode2D<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
) {
    world
        .query::<(&Transform2D, &mut RotatingBgBoss, &Boss)>()
        .iter()
        .for_each(|(_, (t, b, _))| {
            b.0 += b.1 * d.get_frame_time();
            d.draw_texture_pro(
                &state.assets.get("commons_sprite"),
                get_sprite_coord(0, 6, 64., 64.),
                Rectangle::new(t.position().re + 16., t.position().im + 32., 64., 64.),
                Vector2::new(32., 32.),
                b.0,
                Color::WHITE,
            );
        })
}

pub fn wanderable_search(world: &World, d: &RaylibDrawHandle<'_>) {
    world
        .query::<(&mut MoveParams, &mut Wanderable, &Transform2D)>()
        .iter()
        .for_each(|(_, (m, w, t))| {
            if let Some(tgt) = w.target_pos {
                let vel = t.position.dir(&tgt) * w.speed;
                let movement = MoveParams::move_towards(vel, tgt, cmpx!(1., 1.));
                *m = movement;

                let rect = Rectangle::new(tgt.re, tgt.im, 32., 32.);
                if rect.check_collision_point_rec(t.position.to_vec2() + vec2!(16., 32.)) {
                    *m = MoveParams::move_linear(cmpx!(0.));
                    w.elapsed = 0.;
                    w.target_pos = None;
                }
                return;
            }

            if w.elapsed > w.wait {
                w.elapsed = 0.;
                let mut result = cmpx!(150., 50.);
                for _ in 0..4 {
                    let x = w.wander_size.x as i32;
                    let width = w.wander_size.width as i32;
                    let y = w.wander_size.y as i32;
                    let height = w.wander_size.height as i32;
                    let random_x = d.get_random_value::<i32>(x..width) as f32;
                    let random_y = d.get_random_value::<i32>(y..height) as f32;
                    if w.wander_size
                        .check_collision_point_rec(vec2!(random_x, random_y))
                    {
                        result = cmpx!(random_x, random_y);
                    }
                }
                w.target_pos = Some(result);
                return;
            }

            w.elapsed += d.get_frame_time();
        });
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
