#[macro_export]
macro_rules! inputs_vec {
    ($($x:expr),+ $(,)?) => (
        ::std::vec![$($crate::ButtonCode::from($x)),+]
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{GamepadButtonType, KeyCode, MouseButton};
    use crate::ButtonCode;

    #[test]
    fn mixed_inputs() {
        let actual = inputs_vec![MouseButton::Left, KeyCode::Left, GamepadButtonType::DPadLeft];
        let expected = vec![ButtonCode::Mouse(MouseButton::Left), ButtonCode::Kb(KeyCode::Left), ButtonCode::Gamepad(GamepadButtonType::DPadLeft)];
        assert_eq!(expected, actual);
    }
}
