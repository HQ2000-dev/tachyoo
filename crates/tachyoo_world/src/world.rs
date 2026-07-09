use crate::config::WorldConfig;
use crate::event_queue::EventQueue;
use crate::events::Event;
use crate::resources::Ticks;
use crate::systems::systems;
use bevy_ecs::world::World as EcsWorld;

use bevy_ecs::schedule::{Schedule, ScheduleLabel};

#[derive(Debug, Clone, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct Tick;

pub struct World {
    ecs_world: EcsWorld,
    tick_schedule: Schedule,
}

impl World {
    fn load(config: WorldConfig, data: ()) -> World {
        let mut ecs_world = EcsWorld::new();
        ecs_world.insert_resource(EventQueue::new());
        ecs_world.insert_resource(Ticks::default());

        let mut tick_schedule = Schedule::new(Tick);
        tick_schedule.add_systems(systems());

        World {
            ecs_world,
            tick_schedule,
        }
    }

    fn send_events(&mut self, events: impl IntoIterator<Item = Event>) {
        //span?
        (*self.ecs_world.resource_mut::<EventQueue>()).inner_mut();
    }

    fn tick(&mut self) {
        let _tick_span = tracing::info_span!("tick");
        self.tick_schedule.run(&mut self.ecs_world);
    }
}
