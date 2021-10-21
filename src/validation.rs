use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};
use itertools::Itertools;

use crate::{ActionMap, ActionMapInput, ButtonCode, action_map::{KeyActionBinding, PlayerData}, inputs_vec};


// todo: return Res<AddToMapOrSmt, BindingError>
pub(crate) fn validate<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput, B: IntoIterator<Item = ButtonCode>>(
    map: &mut ActionMap<TKeyAction, TAxisAction>,
    player_id: Option<usize>,
    binding: B,
) -> bool {
    let iter = binding.into_iter();
    let len = iter.size_hint().1.unwrap();
    let binding: KeyActionBinding = iter.collect();

    if binding.len() != len {
        return false;
    }
    
    let mut binding_key_combinations: Vec<_> = (1..=binding.len())
        .into_iter()
        // todo: replace itertools:combinations by own fn to get rid of the dep?
        .flat_map(|l| { 
            binding.iter()
                .cloned()
                .combinations(l)
                .map(|c| PlayerData { id: player_id, value: c })
        }).collect();

    let combination = binding_key_combinations.iter().last().unwrap().clone();

    
    let conflict = map.bound_key_combinations.iter().find(|(key, val)| {
        let stored_binding_len = key.value.len();
        
        // if binding len == stored binding len
        //      are the combos the same
        if len == stored_binding_len {
            binding.iter().eq(key.value.iter())
        }
        // if binding len < stored binding len
        //      check binding against stored binding combinations
        else if len < stored_binding_len {
            val.iter().all(|c| {
                !binding.iter().eq(binding.iter())
            })
        }
        // if binding len > stored binding len
        //      check binding combinations against stored binding
        else {
            binding_key_combinations.iter().all(|c| {
                !key.value.iter().eq(c.value.iter())
            })            
        }
    });
    
    if let Some(conflict) = conflict {
        false
    }
    else {
        true
    }

    // if binding_key_combinations.iter().all(|c| map.bound_key_combinations.get(&c).is_none()) {
    //     // println!("{:?}", combination);
    //     // println!("{:?}", binding_key_combinations);
    //     map.bound_key_combinations.insert(combination);
    //     // map.potential_bound_key_combinations.extend(binding_key_combinations.iter().cloned().take(binding_key_combinations.len() - 1));

    //     true
    // }
    // else {
    //     false
    // }
}

// fn bind_button_combination_action_internal<K: Into<TKeyAction>, B: IntoIterator<Item = ButtonCode>>(&mut self, action: K, binding: B, player_id: Option<usize>) -> &mut Self {

#[cfg(test)]
mod tests {
    use super::{validate, inputs_vec};
    use bevy::prelude::KeyCode;
    use test_case::test_case;
    use crate::{ActionMap, ButtonCode};

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub enum TestAction {}

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub enum TestAxis {}

    // todo: binding (hash) comparison tests - order shouldn't matter

    #[test_case(None)]
    #[test_case(Some(1))]
    fn validate_single(player_id: Option<usize>) {
        let mut map = ActionMap::<TestAction, TestAxis>::default();
        let actual = validate(&mut map, player_id, inputs_vec![KeyCode::A]);
        pretty_assertions::assert_eq!(true, actual);
    }

    #[test_case(None)]
    #[test_case(Some(1))]
    fn validate_combo(player_id: Option<usize>) {
        let mut map = ActionMap::<TestAction, TestAxis>::default();
        let actual = validate(&mut map, player_id, inputs_vec![KeyCode::A, KeyCode::B, KeyCode::C]);
        pretty_assertions::assert_eq!(true, actual);
    }

    #[test_case(None)]
    #[test_case(Some(1))]
    fn validate_combo_duped(player_id: Option<usize>) {
        let mut map = ActionMap::<TestAction, TestAxis>::default();
        let actual = validate(&mut map, player_id,inputs_vec![KeyCode::A, KeyCode::A]);
        pretty_assertions::assert_eq!(false, actual);
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
        validate(&mut map, player_id_1, binding_1);
        validate(&mut map, player_id_2, binding_2)
    }
}
