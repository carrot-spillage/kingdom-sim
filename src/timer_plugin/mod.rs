mod timer_heap;

use bevy::{
    app::Update,
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
    prelude::{
        Added, App, Component, Entity, Event, EventWriter, FromWorld, Local, Plugin, Query, ResMut,
    },
};
use bevy_turborand::{DelegatedRng, GlobalRng};
use std::ops::Range;
use timer_heap::TimerHeap;

use crate::GameState;

#[derive(Component, Debug, Clone)]
pub enum TimedComponent<T> {
    OnceExact { data: T, period: u32 },
    OnceRandom { data: T, period_range: Range<u32> },
    RepeatedExact { data: T, period: u32 },
    RepeatedRandom { data: T, period_range: Range<u32> },
}

impl<T> TimedComponent<T> {
    fn get_duration(&self, rng: &mut GlobalRng) -> u32 {
        match self {
            TimedComponent::OnceExact { period, .. } => *period,
            TimedComponent::OnceRandom { period_range, .. } => rng.u32(period_range.clone()),
            TimedComponent::RepeatedExact { period, .. } => *period,
            TimedComponent::RepeatedRandom { period_range, .. } => rng.u32(period_range.clone()),
        }
    }

    pub fn get_data(&self) -> &T {
        match self {
            TimedComponent::OnceExact { data, .. } => data,
            TimedComponent::OnceRandom { data, .. } => data,
            TimedComponent::RepeatedExact { data, .. } => data,
            TimedComponent::RepeatedRandom { data, .. } => data,
        }
    }
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

impl<T: Clone + std::marker::Sync + std::marker::Send + 'static> Plugin for TimerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<ElapsedEvent<T>>().add_systems(
            Update,
            (track_timers::<T>).run_if(in_state(GameState::Playing)),
        );
    }
}

fn track_timers<T: Clone + std::marker::Sync + std::marker::Send + 'static>(
    mut timer_heap: Local<TimerHeap<Option<Entity>>>,
    mut elapsed_writer: EventWriter<ElapsedEvent<T>>,
    mut query: Query<(Entity, &TimedComponent<T>), Added<TimedComponent<T>>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (entity, timed_component) in query.iter_mut() {
        timer_heap.push(Some(entity), timed_component.get_duration(&mut global_rng));
    }

    let expired_items = timer_heap.try_produce();

    for elapsed_item in expired_items.iter() {
        let entity = elapsed_item.unwrap();
        elapsed_writer.send(ElapsedEvent {
            entity,
            _t: std::marker::PhantomData,
        });
    }
}
