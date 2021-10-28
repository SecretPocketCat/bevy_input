use bevy::{ prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use crate::{ActionMap, ActionMapInput};
use futures_lite::future;

pub enum MapIoEvent {
    Load,
    Save
}

pub struct ActionMapPath(pub(crate) String);

pub(crate) struct ActionMapLoad<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput>(pub(crate) Option<Task<anyhow::Result<ActionMap<TKeyAction, TAxisAction>>>>);

pub(crate) struct ActionMapSave(pub(crate) Option<Task<anyhow::Result<()>>>);

pub(crate) fn setup_loader(mut events: EventWriter<MapIoEvent>) {
    events.send(MapIoEvent::Load);
}

pub(crate) fn process_map_event<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput>(
    mut events: EventReader<MapIoEvent>,
    mut load: ResMut<ActionMapLoad<TKeyAction, TAxisAction>>,
    mut save: ResMut<ActionMapSave>,
    thread_pool: Res<AsyncComputeTaskPool>,
    path: Res<ActionMapPath>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>,
)
where
    for<'de> TKeyAction: ActionMapInput + Serialize + Deserialize<'de> + 'static,
    for<'de> TAxisAction: ActionMapInput + Serialize + Deserialize<'de> + 'static,
{
    // loading or saving already
    if load.0.is_some() || save.0.is_some() {
        return;
    }

    for ev in events.iter() {
        match ev {
            MapIoEvent::Load => {
                let path = path.0.clone();
                let task: Task<anyhow::Result<_>> = thread_pool.spawn(async move {
                    // todo: async file read
                    let bytes = std::fs::read(path)?;
                    let map = ron::de::from_bytes::<ActionMap<TKeyAction, TAxisAction>>(&bytes)?;
                    Ok(map)
                });
            
                load.0 = Some(task);
                return;
            },
            MapIoEvent::Save => {
                let map = map.clone();
                let path = path.0.clone();
                let task: Task<Result<_, anyhow::Error>> = thread_pool.spawn(async move {
                    let map_str = ron::ser::to_string_pretty(&map, PrettyConfig::default())?;
                    // todo: async file write
                    std::fs::write(path, map_str)?;
                    Ok(())
                });
            
                save.0 = Some(task);
                return;
            },
        }
    }
}

pub(crate) fn load_map<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput>(
    mut map: ResMut<ActionMap<TKeyAction, TAxisAction>>,
    mut load: ResMut<ActionMapLoad<TKeyAction, TAxisAction>>,
) -> anyhow::Result<()>
where
    for<'de> TKeyAction: ActionMapInput + 'static,
    for<'de> TAxisAction: ActionMapInput + 'static
{
    if let Some(ref mut task) = load.0 {
        if let Some(serialized_map_res) = future::block_on(future::poll_once(&mut *task)) {
            let serialized_map = serialized_map_res?;
            map.set_bindings(serialized_map.key_action_bindings,  serialized_map. axis_action_bindings);
            load.0 = None;
        }
    }

    Ok(())
}

pub(crate) fn save_map(
    mut save: ResMut<ActionMapSave>,
) -> anyhow::Result<()>
{
    if let Some(ref mut task) = save.0 {
        if let Some(res) = future::block_on(future::poll_once(&mut *task)) {
            res?;
            save.0 = None;
        }
    }

    Ok(())
}
