#[macro_export]
macro_rules! inputs_vec {
    ($($x:expr),+ $(,)?) => (
        ::std::vec![$(bevy_input::ButtonCode::from($x)),+]
    );
}

// todo: test coverage
// test
