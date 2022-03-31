use bevy::prelude::*;
use bevy_extensions::panic_on_error;
use bevy_input::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputAction {
    Jump,
    Shoot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputAxis {
    Horizontal,
    Vertical,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_action_input_systems_with_axis::<InputAction, InputAxis>()
        .add_startup_system(setup.chain(panic_on_error))
        .add_system(debug_actions)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) -> Result<(), BindingError> {
    let mut map = ActionMap::<InputAction, InputAxis>::new();
    map.bind_button_action(InputAction::Jump, KeyCode::Space)?
        .bind_button_combination_action(
            InputAction::Jump,
            inputs_vec![KeyCode::W, GamepadButtonType::North],
        )?
        .bind_button_action(InputAction::Jump, GamepadButtonType::South)?
        .bind_button_action(InputAction::Shoot, KeyCode::LShift)?
        .bind_button_combination_action(
            InputAction::Shoot,
            inputs_vec![MouseButton::Left, KeyCode::LControl],
        )?
        .bind_button_combination_action(
            InputAction::Shoot,
            inputs_vec![MouseButton::Left, KeyCode::RControl],
        )?
        .bind_axis_with_deadzone(
            InputAxis::Horizontal,
            AxisBinding::GamepadAxis(GamepadAxisType::LeftStickX),
            0.1,
        )
        .bind_axis_with_deadzone(
            InputAxis::Vertical,
            AxisBinding::GamepadAxis(GamepadAxisType::LeftStickY),
            0.25,
        )
        .bind_axis(
            InputAxis::Horizontal,
            AxisBinding::GamepadAxis(GamepadAxisType::DPadX),
        )
        .bind_axis(
            InputAxis::Horizontal,
            AxisBinding::Buttons(KeyCode::Left.into(), KeyCode::Right.into()),
        )
        .bind_axis(
            InputAxis::Horizontal,
            AxisBinding::Buttons(KeyCode::A.into(), KeyCode::D.into()),
        )
        .bind_axis(
            InputAxis::Horizontal,
            AxisBinding::Buttons(MouseButton::Left.into(), MouseButton::Right.into()),
        );

    // // uncomment to triger a mapping validation error
    // map.bind_button_combination_action(InputAction::Shoot, inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C])?;
    // map.bind_button_combination_action(InputAction::Shoot, inputs_vec![KeyCode::B, KeyCode::A])?;
    // map.bind_button_combination_action(InputAction::Shoot, inputs_vec![KeyCode::B, KeyCode::D])?;

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            color: Color::DARK_GRAY.into(),
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn_bundle(TextBundle {
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
                })
                .insert(map);
        });

    Ok(())
}

fn debug_actions(mut query: Query<(&mut Text, &ActionInput<InputAction, InputAxis>)>) {
    for (mut text, input) in query.iter_mut() {
        text.sections[0].value = format!(
            "{:?}\n{:?}\n\n{:?}\n{:?}\n\n{:?}\n{:?}\n\n{:?}\n{:?}\n",
            InputAction::Jump,
            input.get_button_action_state(InputAction::Jump),
            InputAction::Shoot,
            input.get_button_action_state(InputAction::Shoot),
            InputAxis::Horizontal,
            input.get_axis(&InputAxis::Horizontal),
            InputAxis::Vertical,
            input.get_axis(&InputAxis::Vertical)
        );
    }
}
