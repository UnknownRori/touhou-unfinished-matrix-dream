use crate::{
    entity::{miko, reimu_a},
    event::EventManager,
};

pub fn stage1() -> EventManager {
    let mut event = EventManager::default();

    event.add(0., |stage, state| {
        reimu_a(&mut stage.world);
        // state.audio.play_bgm(2, state.bgm_volume);
    });

    event.add(0., |stage, state| {
        miko(&mut stage.world);
        state.audio.play_bgm(8, state.bgm_volume);
    });

    event
}
