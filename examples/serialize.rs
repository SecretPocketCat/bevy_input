use bevy::prelude::*;
use bevy_input::*;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum InputAction {
    Dodge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum InputAxis {
    Horizontal,
    Vertical,
}

#[derive(Component)]
struct Player(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ActionInputPlugin::<InputAction, InputAxis>::new())
        .add_startup_system(setup)
        .add_system(debug_player_actions)
        .run();
}

fn setup(
    mut map: ResMut<ActionMap<InputAction, InputAxis>>,
    mut gamepad_map: ResMut<GamepadMap>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // todo: load this properly using the asset server?
    let bindings_ron = std::fs::read_to_string("assets/bindings/example_action_bindings.ron").unwrap();
    let bindings = ron::de::from_str::<ActionMap<InputAction, InputAxis>>(&bindings_ron).unwrap();
    map.set_bindings(bindings);

    // map gamepads to players 1 & 2
    for id in 1..=2 {
        gamepad_map.map_gamepad(id - 1, id);
    }

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size{
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            material: materials.add(Color::DARK_GRAY.into()),
            ..Default::default()
        }).with_children(|builder| {
            for id in 1..=4 {
                builder.spawn_bundle(TextBundle {
                    style: Style {
                        size: Size{
                            width: Val::Percent(50.),
                            height: Val::Percent(50.),
                        },
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "!",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Light.ttf"),
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
                }).insert(Player(id));
            }
        });
}

fn debug_player_actions(
    input: Res<ActionInput<InputAction, InputAxis>>,
    mut query: Query<(&mut Text, &Player)>) {
    for (mut text, player) in query.iter_mut() {
        text.sections[0].value = format!("Player {:?}\n\n{:?}\n{:?}\n\nMovement\n{:?}", player.0, InputAction::Dodge, input.get_button_action_state(player.0, &InputAction::Dodge), input.get_xy_axes(player.0, &InputAxis::Horizontal, &InputAxis::Vertical));
    }
}
