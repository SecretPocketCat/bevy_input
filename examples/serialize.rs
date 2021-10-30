use bevy::prelude::*;
use bevy_extensions::panic_on_error;
use bevy_input::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum InputAction {
    Jump,
    Load,
    Save,
    Reset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum InputAxis {
    Horizontal,
    Vertical,
}

#[derive(Component)]
struct InputStateText;

static BINDINGS_PATH: &str = "assets\\bindings\\example.bindings";

type Input = ActionInput<InputAction, InputAxis>;
type Map = ActionMap<InputAction, InputAxis>;

#[derive(Debug)]
struct Ui {
    binding_list_entity: Entity,
    font_light: Handle<Font>,
    font_medium: Handle<Font>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ActionInputPlugin::<InputAction, InputAxis>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system(reset_map.chain(panic_on_error))
        .add_system(update_bindings_ui)
        .add_system(actions_state_ui)
        .run();
}

fn setup(
    mut commands: Commands,
    mut col_materials: ResMut<Assets<ColorMaterial>>,
    mut input_map_io_w: EventWriter<MapIoRequest>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let panel_bg_col = col_materials.add(Color::DARK_GRAY.into());
    let font_light = asset_server.load("fonts/FiraSans-Light.ttf");
    let font_medium = asset_server.load("fonts/FiraSans-Medium.ttf");

    // bindings panel
    let bindings_e = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position: Rect {
                    left: Val::Percent(5.),
                    top: Val::Percent(5.),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Percent(90.),
                    width: Val::Percent(40.),
                    ..Default::default()
                },
                flex_direction: FlexDirection::ColumnReverse,
                padding: Rect::all(Val::Px(35.)),
                ..Default::default()
            },
            material: panel_bg_col.clone(),
            ..Default::default()
        }).id();

    // state panel
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Percent(5.),
                    top: Val::Percent(5.),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Percent(90.),
                    width: Val::Percent(40.),
                    ..Default::default()
                },
                padding: Rect::all(Val::Px(35.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: panel_bg_col.clone(),
            ..Default::default()
        }).with_children(|b| {
            b.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: font_light.clone(),
                        font_size: 30.0,
                        color: Color::ANTIQUE_WHITE,
                        ..Default::default()
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            })
            .insert(InputStateText);
        });

    let ui = Ui {
        binding_list_entity: bindings_e,
        font_light,
        font_medium,
    };

    commands.insert_resource(ui);

    input_map_io_w.send(MapIoRequest::Load(BINDINGS_PATH.into()));
}

fn handle_input(
    input: Res<Input>,
    mut map_ev_w: EventWriter<MapIoRequest>
) {
    if input.just_released(InputAction::Save) {
        map_ev_w.send(MapIoRequest::Save(BINDINGS_PATH.into()));
    }
    else if input.just_released(InputAction::Load) {
        map_ev_w.send(MapIoRequest::Load(BINDINGS_PATH.into()));
    }
}

fn reset_map(
    mut map: ResMut<Map>,
    mut map_ev_w: EventWriter<MapIoRequest>,
    input: Res<Input>,
) -> Result<(), BindingError> {
    if input.just_released(InputAction::Reset) {
        map.clear_bindings();
        map
            .bind_button_action(InputAction::Jump, KeyCode::Space)?
            .bind_button_action(InputAction::Jump, GamepadButtonType::South)?
            .bind_button_combination_action(InputAction::Load, inputs_vec!(KeyCode::LControl, KeyCode::L))?
            .bind_button_combination_action(InputAction::Save, inputs_vec!(KeyCode::LControl, KeyCode::S))?
            .bind_button_combination_action(InputAction::Reset, inputs_vec!(KeyCode::LControl, KeyCode::R))?
            .bind_axis(InputAxis::Horizontal, AxisBinding::Buttons(KeyCode::A.into(), KeyCode::D.into()))
            .bind_axis(InputAxis::Vertical, AxisBinding::Buttons(KeyCode::S.into(), KeyCode::W.into()))
            .bind_axis(InputAxis::Horizontal, AxisBinding::GamepadAxis(GamepadAxisType::LeftStickX))
            .bind_axis(InputAxis::Vertical, AxisBinding::GamepadAxis(GamepadAxisType::LeftStickY));
    
        map_ev_w.send(MapIoRequest::Save(BINDINGS_PATH.into()))
    }

    Ok(())
}

fn update_bindings_ui(
    mut ev: EventReader<MapIoEvent>,
    mut commands: Commands,
    ui: Res<Ui>,
    children_q: Query<&Children>,
    map: Res<Map>,
) {
    for ev in ev.iter() {
        if let MapIoEvent::Loaded = ev {
            if let Ok(children) = children_q.get(ui.binding_list_entity) {
                for e in children.iter() {
                    commands.entity(*e).despawn_recursive();
                }
            }

            commands.entity(ui.binding_list_entity)
                .with_children(|builder| {
                    for action in map.get_key_bindings().iter().sorted_unstable_by_key(|b| format!("{:?}", b.0.value())) {
                        builder
                            .spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect {
                                        top: Val::Px(50.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    format!("{:?}:", action.0.value()),
                                    TextStyle {
                                        font: ui.font_medium.clone(),
                                        font_size: 30.0,
                                        color: Color::ANTIQUE_WHITE,
                                        ..Default::default()
                                    },
                                    TextAlignment {
                                        horizontal: HorizontalAlign::Left,
                                        vertical: VerticalAlign::Center,
                                        ..Default::default()
                                    },
                                ),
                                ..Default::default()
                            });
                        
                        for binding in action.1 {
                            builder
                                .spawn_bundle(TextBundle {
                                    style: Style {
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        format!("{:?}", binding),
                                        TextStyle {
                                            font: ui.font_light.clone(),
                                            font_size: 30.0,
                                            color: Color::ANTIQUE_WHITE,
                                            ..Default::default()
                                        },
                                        TextAlignment {
                                            horizontal: HorizontalAlign::Left,
                                            vertical: VerticalAlign::Center,
                                            ..Default::default()
                                        },
                                    ),
                                    ..Default::default()
                                });
                        }
                    }
                });
        }
    }
}

fn actions_state_ui(input: Res<ActionInput<InputAction, InputAxis>>, mut query: Query<&mut Text, With<InputStateText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "{:?}\n{:?}\n\n{:?}\n{:?}\n\n{:?}\n{:?}\n\n{:?}\n{:?}\n\n{:?}\n{:?}",
            InputAction::Jump,
            input.get_button_action_state(InputAction::Jump),
            InputAction::Load,
            input.get_button_action_state(InputAction::Load),
            InputAction::Save,
            input.get_button_action_state(InputAction::Save),
            InputAction::Reset,
            input.get_button_action_state(InputAction::Reset),
            InputAxis::Horizontal,
            input.get_xy_axes(&InputAxis::Horizontal, &InputAxis::Vertical)
        );
    }
}
