use pddllib::{state::State, task::Goal};

use super::Heuristic;

#[derive(Debug, Default)]
pub struct GoalCount {}

impl Heuristic for GoalCount {
    fn estimate(&self, state: &State, goal: &Goal) -> usize {
        goal.iter()
            .filter(|(fact, value)| state.has_nary(fact.predicate, &fact.args) != *value)
            .count()
    }
}
