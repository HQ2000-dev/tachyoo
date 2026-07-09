use std::{
    ops::Add,
    time::{Duration, Instant},
};

use bevy_ecs::resource::Resource;

#[derive(Resource, Debug, PartialEq, Eq, Clone)]
pub struct Ticks(u128);

impl Default for Ticks {
    fn default() -> Self {
        Self(0)
    }
}

impl Ticks {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn get(&self) -> u128 {
        self.0
    }
}

//maybe just store duration per tick?
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct Tps(u32);

impl Tps {
    /// nonnull duration!!
    /// panics if the duration is too small
    pub fn from_interval(duration: Duration) -> Self {
        Self(
            (Duration::from_secs(1).as_micros() / duration.as_micros())
                .try_into()
                .unwrap(),
        )
    }

    pub fn interval(&self) -> Duration {
        Duration::from_micros(
            Duration::from_secs(1)
                .as_micros()
                .checked_div(self.0 as u128)
                .unwrap()
                .try_into()
                .unwrap(),
        )
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct NextTickTime(Instant);

impl Default for NextTickTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl NextTickTime {
    //TODO surely, this can be implemented more efficiently
    // TODO: virtual ticks!
    pub fn advance_by_duration(&mut self, duration: Duration) {
        self.0 = Instant::now().add(duration);
    }
}
