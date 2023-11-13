mod timer_heap;

use std::default;

use timer_heap::TimerHeap;

use bevy::prelude::{
    default, Added, App, Component, Entity, Event, EventWriter, FromWorld, Local, Plugin, Query,
    ResMut,
};
use bevy_turborand::{DelegatedRng, GlobalRng};

#[derive(Debug, Clone, Copy)]
enum TimerType {
    OnceExact(u32),
    OnceRandom(u32, u32),
    RepeatedExact(u32),
    RepeatedRandom(u32, u32),
}

impl Default for TimerType {
    fn default() -> Self {
        Self::OnceExact(0)
    }
}

impl TimerType {
    fn get_duration(&self, rng: &mut GlobalRng) -> u32 {
        match self {
            Self::OnceExact(duration) => *duration,
            Self::OnceRandom(min, max) => rng.u32(*min..*max),
            Self::RepeatedExact(duration) => *duration,
            Self::RepeatedRandom(min, max) => rng.u32(*min..*max),
        }
    }
}

#[derive(Component)]
struct Timer<T>(TimerType, T);

#[derive(Event)]
struct ProducedEvent<T> {
    pub entity: Entity,
    _t: std::marker::PhantomData<T>,
}

pub struct TimerPlugin<T: Clone + std::marker::Sync + std::marker::Send + 'static> {
    heap: TimerHeap<(T, TimerType)>,
    _t: std::marker::PhantomData<T>,
}

impl<T: Clone + std::marker::Sync + std::marker::Send + 'static> Plugin for TimerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<ProducedEvent<T>>();
    }
}

fn track_producer<
    T: Default + FromWorld + Clone + std::marker::Sync + std::marker::Send + 'static,
>(
    mut timer_heap: Local<TimerHeap<(Option<Entity>, TimerType)>>,
    mut elapsed_writer: EventWriter<ProducedEvent<T>>,
    mut query: Query<(Entity, &Timer<T>), Added<Timer<T>>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (entity, timer) in query.iter_mut() {
        timer_heap.push(
            (Some(entity), timer.0),
            timer.0.get_duration(&mut global_rng),
        );
    }

    let expired_items = timer_heap.try_produce();

    for elapsed_item in expired_items.iter() {
        let entity = elapsed_item.0.unwrap();
        elapsed_writer.send(ProducedEvent {
            entity,
            _t: std::marker::PhantomData,
        });
    }
}
