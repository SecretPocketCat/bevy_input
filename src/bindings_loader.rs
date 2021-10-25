use std::marker::PhantomData;
use bevy::{asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset}, prelude::{AssetEvent, AssetServer, Assets, Commands, EventReader, Res, ResMut}, tasks::AsyncComputeTaskPool};
use serde::Deserialize;
use crate::{ActionMap, ActionMapInput, action_map::SerializedActionMap};

#[derive(Default)]
pub(crate) struct BindingsLoader<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput>(
    PhantomData<TKeyAction>,
    PhantomData<TAxisAction>,
);

impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> BindingsLoader<TKeyAction, TAxisAction> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<TKeyAction, TAxisAction> AssetLoader for BindingsLoader<TKeyAction, TAxisAction>
where
    for<'de> TKeyAction: ActionMapInput + Deserialize<'de> + Send + Sync + 'static,
    for<'de> TAxisAction: ActionMapInput + Deserialize<'de> + Send + Sync + 'static,
{
    fn load<'a>
     (
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let map = ron::de::from_bytes::<SerializedActionMap<TKeyAction, TAxisAction>>(bytes)?;
            load_context.set_labeled_asset("bindings", LoadedAsset::new(map));
            println!("bindings loaded");
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["bindings"];
        EXTENSIONS
    }
}

pub(crate) fn process_binding_assets<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut map_events: EventReader<AssetEvent<SerializedActionMap<TKeyAction, TAxisAction>>>,
    mut map_assets: ResMut<Assets<SerializedActionMap<TKeyAction, TAxisAction>>>,
    mut map: ResMut<ActionMap<TKeyAction, TAxisAction>>,
    asset_server: Res<AssetServer>,
) {
    for event in map_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                println!("bindings asset created");
                if let Some(serialized_map) = map_assets.get_mut(handle) {
                    println!("bindings set: {:?}", serialized_map);
                    let serialized_map = serialized_map.clone();
                    map.set_bindings(serialized_map.key_action_bindings, serialized_map.axis_action_bindings);
                }
            }
            AssetEvent::Removed { .. } => {
                println!("bindings rmvd");
                // map.clear_bindings();
            },
            _ => {
                println!("bindings updated");
            }
        }
    }
}





fn spawn_tasks<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput>(
    mut commands: Commands,
    thread_pool: Res<AsyncComputeTaskPool>,
)
where
    for<'de> TKeyAction: ActionMapInput + Deserialize<'de> + Send + Sync,
    for<'de> TAxisAction: ActionMapInput + Deserialize<'de> + Send + Sync,
{
    let task = thread_pool.spawn(async move {
        let map = ron::de::from_bytes::<SerializedActionMap<TKeyAction, TAxisAction>>(bytes)?
    });

    commands.spawn().insert(task);
}
