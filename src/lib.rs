use assets::AudioAssets;
use raylib::prelude::*;
use scenes::main_menu::MainMenu;
use state::State;

pub mod assets;
pub mod components;
pub mod controls;
pub mod difficulty;
pub mod renderer;
pub mod scenes;
pub mod score;
pub mod state;
pub mod systems;
pub mod ui;
pub mod utility;
pub mod window;

pub struct Game<'a> {
    rl: RaylibHandle,
    thread: RaylibThread,
    render: RenderTexture2D,

    state: State<'a>,
}

impl<'a> Game<'a> {
    pub fn new(audio: &'a RaylibAudio) -> Self {
        let mut binding = raylib::init();
        let rl = binding
            .size(900, 675)
            .title("Touhou Unfinished Matrix Dream");
        let (mut rl, thread) = rl.build();
        rl.set_target_fps(120);
        rl.set_exit_key(Some(KeyboardKey::KEY_F12));

        let audio_asset = AudioAssets::new(audio);
        let mut state = State::new(&mut rl, &thread, audio_asset);
        state.change_scene(Box::new(MainMenu::new()));

        let render = rl.load_render_texture(&thread, 640, 480).unwrap();
        render.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

        Self {
            rl,
            thread,
            state,
            render,
        }
    }

    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
            if *self.state.should_quit() {
                break;
            }

            let mut d = self.rl.begin_drawing(&self.thread);
            self.state.update(&d);

            {
                let mut dt = d.begin_texture_mode(&self.thread, &mut self.render);
                self.state.draw(&mut dt);
            }

            d.clear_background(Color::BLACK);
            d.draw_texture_pro(
                &self.render,
                Rectangle::new(0., 0., -640 as f32, 480 as f32),
                Rectangle::new(
                    0.,
                    0.,
                    d.get_screen_width() as f32,
                    d.get_screen_height() as f32,
                ),
                Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32),
                // Vector2::new(0., 0.),
                //  0.,
                180.,
                Color::WHITE,
            );
            d.draw_fps(0, 0);
        }
    }
}
