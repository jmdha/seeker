pub mod heuristic;
pub mod search;

use fxhash::FxBuildHasher;
use indexmap::IndexMap;
use pddllib::state::State;

type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;

fn trace(parents: &FxIndexMap<State, usize>, goal_index: usize) -> Vec<State> {
    let mut i = goal_index;
    let mut states = vec![];
    loop {
        let (state, parent) = parents.get_index(i).unwrap();
        states.push(state.to_owned());
        i = *parent;
        if i == 0 {
            states.push(parents.get_index(*parent).unwrap().0.to_owned());
            break;
        }
    }
    states.reverse();
    states
}
