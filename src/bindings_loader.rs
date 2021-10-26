use std::{marker::PhantomData};
use bevy::{asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset}, prelude::{AssetEvent, AssetServer, Assets, Commands, Entity, EventReader, EventWriter, Query, Res, ResMut}, tasks::{AsyncComputeTaskPool, Task}};
use serde::Deserialize;
use crate::{ActionMap, ActionMapInput, action_map::SerializedActionMap};
use futures_lite::future;

pub enum MapIoEvent {
    Load,
    Save
}

pub struct MapPath(pub(crate) String);

pub(crate) fn setup_loader(mut events: EventWriter<MapIoEvent>) {
    events.send(MapIoEvent::Load);
}

pub(crate) fn process_map_event<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput>(
    mut commands: Commands,
    thread_pool: Res<AsyncComputeTaskPool>,
    mut events: EventReader<MapIoEvent>,
    path: Res<MapPath>,
)
where
    for<'de> TKeyAction: ActionMapInput + Deserialize<'de> + 'static,
    for<'de> TAxisAction: ActionMapInput + Deserialize<'de> + 'static,
{
    for ev in events.iter() {
        match ev {
            MapIoEvent::Load => {
                let path = path.0.clone();
                let task = thread_pool.spawn(async move {
                    // todo: error handling
                    let bytes = std::fs::read(path).unwrap();
                    ron::de::from_bytes::<SerializedActionMap<TKeyAction, TAxisAction>>(&bytes).unwrap()
                });
            
                commands.spawn().insert(task);
                return;
            },
            MapIoEvent::Save => {
                // todo: save task
                // let task = thread_pool.spawn(async move {
                //     let bytes = std::fs::read("").unwrap();
                //     ron::de::from_bytes::<SerializedActionMap<TKeyAction, TAxisAction>>(&bytes).unwrap()
                // });
            
                // commands.spawn().insert(task);
                return;
            },
        }
    }
}

pub(crate) fn load_map<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput>(
    mut commands: Commands,
    mut load_q: Query<(Entity, &mut Task<SerializedActionMap<TKeyAction, TAxisAction>>)>,
    mut map: ResMut<ActionMap<TKeyAction, TAxisAction>>,
)
where
    for<'de> TKeyAction: ActionMapInput + Deserialize<'de> + 'static,
    for<'de> TAxisAction: ActionMapInput + Deserialize<'de> + 'static
{
    for (entity, mut task) in load_q.iter_mut() {
        if let Some(serialized_map) = future::block_on(future::poll_once(&mut *task)) {
            map.set_bindings(serialized_map.key_action_bindings,  serialized_map. axis_action_bindings);
            commands.entity(entity).remove::<Task<SerializedActionMap<TKeyAction, TAxisAction>>>();
        }
    }
}
