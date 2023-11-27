mod timer_heap;

use bevy::{
    app::Update,
    ecs::{
        component::Component,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
    },
    prelude::{Added, App, Entity, Event, EventWriter, Local, Plugin, Query, ResMut},
};
use bevy_turborand::{DelegatedRng, GlobalRng};
use timer_heap::TimedQue;

use crate::GameState;

#[derive(Clone, Copy, Debug)]
pub enum TimerSettings {
    OnceExact(u32),
    OnceRandom(u32, u32),
    RepeatedExact(u32),
    RepeatedRandom(u32, u32),
}

impl TimerSettings {
    fn get_duration(&self, rng: &mut GlobalRng) -> u32 {
        match self {
            TimerSettings::OnceExact(period) => *period,
            TimerSettings::OnceRandom(min, max) => rng.u32(min..max),
            TimerSettings::RepeatedExact(period) => *period,
            TimerSettings::RepeatedRandom(min, max) => rng.u32(min..max),
        }
    }
}

pub trait Timed {
    fn get_timer_settings(&self) -> TimerSettings;
}

#[derive(Event)]
pub struct ElapsedEvent<T> {
    pub entity: Entity,
    _t: std::marker::PhantomData<T>,
}

pub struct TimerPlugin<T: Clone + std::marker::Sync + std::marker::Send + 'static> {
    _t: std::marker::PhantomData<T>,
}

impl<T: Clone + std::marker::Sync + std::marker::Send + 'static> TimerPlugin<T> {
    pub fn new() -> Self {
        TimerPlugin {
            _t: std::marker::PhantomData,
        }
    }
}

impl<T: Component + Timed + Clone> Plugin for TimerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<ElapsedEvent<T>>().add_systems(
            Update,
            (track_timers::<T>).run_if(in_state(GameState::Playing)),
        );
    }
}

fn track_timers<T: Component + Timed + Clone>(
    mut timed_que: Local<TimedQue<Option<Entity>>>,
    mut elapsed_writer: EventWriter<ElapsedEvent<T>>,
    mut query: Query<(Entity, &T), Added<T>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (entity, timed_component) in query.iter_mut() {
        timed_que.push(
            Some(entity),
            timed_component
                .get_timer_settings()
                .get_duration(&mut global_rng),
        );
    }

    let elapsed_items = timed_que.pop_elapsed();

    for elapsed_item in elapsed_items.iter() {
        let entity = elapsed_item.unwrap();
        elapsed_writer.send(ElapsedEvent {
            entity,
            _t: std::marker::PhantomData,
        });
    }
}
