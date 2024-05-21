pub mod goal_count;

use clap::ValueEnum;
use pddllib::{state::State, task::Task};

#[derive(ValueEnum, Clone, Debug)]
pub enum HeuristicKind {
    GoalCount,
}

pub trait Heuristic {
    fn estimate(&mut self, task: &Task, state: &State) -> usize;
}

pub fn generate<'a>(_: &'a Task, heuristic: &'a HeuristicKind) -> Box<dyn Heuristic> {
    match heuristic {
        HeuristicKind::GoalCount => Box::new(goal_count::GoalCount::new()),
    }
}
