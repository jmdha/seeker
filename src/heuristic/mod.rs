pub mod goal_count;

use pddllib::{state::State, task::Goal};

pub trait Heuristic {
    fn estimate(&self, state: &State, goal: &Goal) -> usize;
}
