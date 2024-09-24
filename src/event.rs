use crate::{scenes::stage_view::StageView, state::State};

pub struct Event {
    pub start: f32,
    pub is_spawned: bool,
    pub action: Option<Box<dyn FnOnce(&mut StageView, &mut State)>>,
}

#[derive(Default)]
pub struct EventManager {
    pub timer: f32,
    pub lists: Vec<Event>,
}

impl EventManager {
    pub fn add(&mut self, start: f32, action: impl FnOnce(&mut StageView, &mut State) + 'static) {
        self.lists.push(Event {
            start,
            is_spawned: false,
            action: Some(Box::new(action)),
        });
    }
    pub fn update(&mut self, stage: &mut StageView, state: &mut State, time: f32) {
        self.timer += time;
        self.lists
            .iter_mut()
            .filter(|event| !event.is_spawned && event.start < self.timer)
            .for_each(|event| match event.action.take() {
                Some(action) => (action)(stage, state),
                None => {}
            });

        self.lists.retain(|event| !event.is_spawned);
    }
}
