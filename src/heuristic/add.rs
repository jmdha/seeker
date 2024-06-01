use super::Heuristic;
use pddllib::{state::State, task::Task};

pub struct Add {}

impl Add {
    pub fn new() -> Self {
        Self {}
    }
}

impl Heuristic for Add {
    fn estimate(&self, task: &Task, state: &State) -> usize {
        task.goal
            .iter()
            .filter(|(fact, value)| state.has_fact(task, fact) != *value)
            .count()
    }
}
