use std::collections::VecDeque;

use bevy_ecs::resource::Resource;

use crate::events::Event;

#[derive(Resource)]
pub(crate) struct EventQueue {
    //just a deque for now
    inner: VecDeque<Event>,
}

impl EventQueue {
    pub(crate) fn inner_mut(&mut self) -> &mut VecDeque<Event> {
        &mut self.inner
    }

    pub(crate) fn inner(&self) -> &VecDeque<Event> {
        &self.inner
    }

    pub(crate) fn new() -> Self {
        Self(VecDeque::new())
    }
}
