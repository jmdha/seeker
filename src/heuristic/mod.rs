pub mod goal_count;

use clap::ValueEnum;
use pddllib::{state::State, task::Task};

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum HeuristicKind {
    GoalCount,
}

pub struct Heuristic {
    kind: HeuristicKind,
    best_estimate: usize,
    estimates: usize,
}

impl Heuristic {
    pub fn new(kind: HeuristicKind) -> Self {
        Self {
            kind,
            best_estimate: usize::MAX,
            estimates: 0,
        }
    }

    pub fn estimate(&mut self, task: &Task, state: &State) -> usize {
        let estimate = match self.kind {
            HeuristicKind::GoalCount => goal_count::estimate(task, state),
        };
        if estimate < self.best_estimate {
            println!("New best heuristic estimate: {}", estimate);
            self.best_estimate = estimate;
        }
        self.estimates += 1;
        estimate
    }
}

impl Drop for Heuristic {
    fn drop(&mut self) {
        println!("Heuristic estimates: {}", self.estimates);
    }
}
