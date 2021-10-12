#![feature(destructuring_assignment)]
#![feature(if_let_guard)]

mod plugin;
mod action_map;

use action_map::{ActionInput, ActionMap, ActionMapInput};
use bevy::prelude::*;
use plugin::ActionInputPlugin;

use crate::action_map::AxisBinding;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputAction {
    Jump,
    Shoot,
}

impl ActionMapInput for InputAction {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputAxis {
    Horizontal,
}

impl ActionMapInput for InputAxis {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ActionInputPlugin::<InputAction, InputAxis>::new())
        .add_startup_system(setup)
        .add_system(debug_actions)
        .run();
}

fn setup(
    mut map: ResMut<ActionMap<InputAction, InputAxis>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    map
        .bind_key_action(InputAction::Jump, vec![KeyCode::Space.into()])
        .bind_key_action(InputAction::Jump, vec![KeyCode::W.into(), GamepadButtonType::North.into()])
        .bind_key_action(InputAction::Jump, vec![GamepadButtonType::South.into()])
        .bind_key_action(InputAction::Shoot, vec![KeyCode::LShift.into()])
        .bind_key_action(InputAction::Shoot, vec![MouseButton::Left.into(), KeyCode::LControl.into()])
        .bind_key_action(InputAction::Shoot, vec![MouseButton::Left.into(), KeyCode::RControl.into()])
        .bind_axis(InputAxis::Horizontal, AxisBinding::GamepadAxis(GamepadAxisType::LeftStickX))
        .bind_axis(InputAxis::Horizontal, AxisBinding::GamepadAxis(GamepadAxisType::DPadX))
        .bind_axis(InputAxis::Horizontal, AxisBinding::Buttons(KeyCode::Left.into(), KeyCode::Right.into()))
        .bind_axis(InputAxis::Horizontal, AxisBinding::Buttons(KeyCode::A.into(), KeyCode::D.into()))
        .bind_axis(InputAxis::Horizontal, AxisBinding::Buttons(MouseButton::Left.into(), MouseButton::Right.into()));

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
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: materials.add(Color::DARK_GRAY.into()),
                ..Default::default()
            }).with_children(|builder| {
                builder.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "!",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Light.ttf"),
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
                });
            });
}

fn debug_actions(
    input: Res<ActionInput<InputAction, InputAxis>>,
    mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        // println!("{:?} => {:?}", InputAction::Jump, input.get_key_action_state(&InputAction::Jump));
        // println!("{:?} => {:?}", InputAction::Shoot, input.get_key_action_state(&InputAction::Shoot));

        text.sections[0].value = format!("{:?}\n{:?}\n\n{:?}\n{:?}\n\n{:?}\n{:?}\n", InputAction::Jump, input.get_key_action_state(&InputAction::Jump), InputAction::Shoot, input.get_key_action_state(&InputAction::Shoot), InputAxis::Horizontal, input.get_axis(&InputAxis::Horizontal));
    }
}
