use touhou_unfinished_matrix_dream::Game;

fn main() {
    let audio = raylib::audio::RaylibAudio::init_audio_device().unwrap();

    let mut game = Game::new(&audio);
    game.run();
}
