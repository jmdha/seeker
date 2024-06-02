use super::Heuristic;
use pddllib::{state::State, task::Task};

pub struct Add {}

impl Add {
    pub fn new() -> Self {
        Self {}
    }
}

impl Heuristic for Add {
    fn estimate(&self, _: &Task, _: &State) -> usize {
        todo!()
    }
}
