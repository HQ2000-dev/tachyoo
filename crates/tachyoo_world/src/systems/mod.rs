use std::ops::DerefMut;

use bevy_ecs::{
    prelude::*,
    schedule::{IntoScheduleConfigs, Schedule},
    system::{ResMut, ScheduleSystem},
};

use crate::resources::{NextTickTime, Ticks, Tps};

pub(crate) fn add_systems(schedule: &mut Schedule) {
    schedule.add_systems((tick_compute).chain());
}

fn tick_compute(mut next_tick: ResMut<NextTickTime>, tps: Res<Tps>, mut ticks: ResMut<Ticks>) {
    //TODO: Reset next tick time if lagging 2s behind expected tick time
    next_tick.advance_by_duration(tps.interval());
    ticks.increment();
}

/*
//all three just one system?
fn reset_next_tick_time_if_2s_behind(next_tick_time: ResMut<NextTickTime>) {
    //TODO
}

fn compute_next_tick_time(next_tick_time: ResMut<NextTickTime>, tps: Res<Tps>) {

}

fn increment_ticks_res(mut ticks: ResMut<Ticks>) {
    ticks.increment();
}*/
