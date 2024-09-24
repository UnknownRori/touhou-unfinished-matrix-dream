use crate::event::EventManager;

pub fn stage1() -> EventManager {
    let mut event = EventManager::default();

    event.add(5., |stage, state| {
        state.audio.play_bgm(4, state.bgm_volume);
    });

    event.add(20., |stage, state| {
        state.audio.stop_bgm();
    });

    event
}
