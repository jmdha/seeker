use super::Heuristic;
use pddllib::{state::State, task::Task};

#[derive(Debug)]
pub struct Constant {
    value: usize,
}

impl Constant {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

impl Heuristic for Constant {
    fn estimate(&self, _: &Task, _: &State) -> usize {
        self.value
    }
}
