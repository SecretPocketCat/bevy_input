use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};
use itertools::Itertools;

use crate::{ActionMap, ActionMapInput, ButtonCode, action_map::{KeyActionBinding, PlayerData}, inputs_vec};

pub enum BindingError {
    Conflict(PlayerData<HashSet<ButtonCode>>),
}

// todo: return Res<AddToMapOrSmt, BindingError>
pub(crate) fn add_binding<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput, B: IntoIterator<Item = ButtonCode>>(
    map: &mut ActionMap<TKeyAction, TAxisAction>,
    player_id: Option<usize>,
    binding: B,
) -> Result<(), BindingError> {
    let iter = binding.into_iter();
    let len = iter.size_hint().1.unwrap();
    let binding: KeyActionBinding = iter.collect();

    if binding.len() != len {
        return Ok(());
    }
    
    let mut binding_key_combinations: Vec<HashSet<_>> = (1..=binding.len())
        .into_iter()
        // todo: replace itertools:combinations by own fn to get rid of the dep?
        .flat_map(|l| { 
            binding.iter()
                .copied()
                .combinations(l)
                .map(|c| c.iter().copied().collect())
        }).collect();

    let conflict = map.bound_key_combinations.iter().find(|(key, val)| {
        let stored_binding_len = key.value.len();
        
        if len == stored_binding_len {
            bindings_eq(player_id, &binding, key.id, &key.value)
        }
        else if len < stored_binding_len {
            // check binding against stored binding combinations
            val.iter().any(|c| {
                bindings_eq(player_id, &binding, key.id, c)
            })
        }
        else {
            // binding len > stored binding len
            // check binding combinations against stored binding
            binding_key_combinations.iter().any(|c| {
                bindings_eq(player_id, c, key.id, &key.value)
            })
        }
    });
    
    if let Some(conflict) = conflict {
        Err(BindingError::Conflict(conflict.0.clone()))
    }
    else {
        // todo: store the bindings...
        map.bound_key_combinations.push((
                PlayerData {
                id: player_id,
                value: binding
            },
            binding_key_combinations));

        Ok(())
    }
}

// todo: take hashsets directly? take IntoIterator?
pub(crate) fn bindings_eq(player_id_1: Option<usize>, binding_1: &HashSet<ButtonCode>, player_id_2: Option<usize>, binding_2: &HashSet<ButtonCode>) -> bool {
    if player_id_1 != player_id_2 {
        return false;
    }

    if binding_1.len() != binding_2.len() {
        return false;
    }

    binding_1.is_superset(binding_2)
}

#[cfg(test)]
mod tests {
    use super::{add_binding, bindings_eq, inputs_vec};
    use bevy::prelude::KeyCode;
    use test_case::test_case;
    use crate::{ActionMap, ButtonCode};

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub enum TestAction {}

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub enum TestAxis {}

    #[test_case(Some(1), inputs_vec![KeyCode::A], Some(1), inputs_vec![KeyCode::B] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A], Some(1), inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B], Some(1), inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C] => false)]
    #[test_case(None, inputs_vec![KeyCode::A], None, inputs_vec![KeyCode::A] => true)]
    #[test_case(Some(1), inputs_vec![KeyCode::A], Some(1), inputs_vec![KeyCode::A] => true)]
    #[test_case(Some(1), inputs_vec![KeyCode::A], None, inputs_vec![KeyCode::A] => false)]
    #[test_case(None, inputs_vec![KeyCode::A], Some(2), inputs_vec![KeyCode::A] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A], Some(2), inputs_vec![KeyCode::A] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B], Some(1), inputs_vec![KeyCode::A, KeyCode::B] => true)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B], Some(1), inputs_vec![KeyCode::B, KeyCode::A] => true)]
    fn bindings_equal(player_id_1: Option<usize>, binding_1: Vec<ButtonCode>, player_id_2: Option<usize>, binding_2: Vec<ButtonCode>) -> bool {
        bindings_eq(
                player_id_1,
                &binding_1.iter().copied().collect(),
                player_id_2,
                &binding_2.iter().copied().collect(),
            )
    }

    #[test_case(None)]
    #[test_case(Some(1))]
    fn validate_single(player_id: Option<usize>) {
        let mut map = ActionMap::<TestAction, TestAxis>::default();
        let actual = add_binding(&mut map, player_id, inputs_vec![KeyCode::A]);
        pretty_assertions::assert_eq!(true, actual.is_ok());
    }

    #[test_case(None)]
    #[test_case(Some(1))]
    fn validate_combo(player_id: Option<usize>) {
        let mut map = ActionMap::<TestAction, TestAxis>::default();
        let actual = add_binding(&mut map, player_id, inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C]);
        pretty_assertions::assert_eq!(true, actual.is_ok());
    }

    #[test_case(None)]
    #[test_case(Some(1))]
    fn validate_combo_duped(player_id: Option<usize>) {
        let mut map = ActionMap::<TestAction, TestAxis>::default();
        let actual = add_binding(&mut map, player_id,inputs_vec![KeyCode::A, KeyCode::A]);
        pretty_assertions::assert_eq!(false, actual.is_err());
    }

    #[test_case(None, inputs_vec![KeyCode::A], None, inputs_vec![KeyCode::B] => true)]
    #[test_case(Some(1), inputs_vec![KeyCode::A], Some(1), inputs_vec![KeyCode::B] => true)]
    #[test_case(Some(1), inputs_vec![KeyCode::A], Some(2), inputs_vec![KeyCode::B] => true)]
    #[test_case(None, inputs_vec![KeyCode::A], None, inputs_vec![KeyCode::A] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A], Some(1), inputs_vec![KeyCode::A] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A], Some(2), inputs_vec![KeyCode::A] => true)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B], Some(1), inputs_vec![KeyCode::B, KeyCode::C] => true)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B], Some(1), inputs_vec![KeyCode::A, KeyCode::B] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C], Some(1), inputs_vec![KeyCode::A] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C], Some(1), inputs_vec![KeyCode::A, KeyCode::B] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C], Some(1), inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C], Some(1), inputs_vec![KeyCode::B, KeyCode::C, KeyCode::D] => true)]
    #[test_case(Some(1), inputs_vec![KeyCode::A], Some(1), inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C] => false)]
    #[test_case(Some(1), inputs_vec![KeyCode::A, KeyCode::B], Some(1), inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C] => false)]
    fn validate_seq(player_id_1: Option<usize>, binding_1: Vec<ButtonCode>, player_id_2: Option<usize>, binding_2: Vec<ButtonCode>) -> bool {
        let mut map = ActionMap::<TestAction, TestAxis>::default();
        add_binding(&mut map, player_id_1, binding_1);
        add_binding(&mut map, player_id_2, binding_2).is_ok()
    }
}
