pub mod constant;
pub mod goal_count;

use std::time::{Duration, Instant};

use clap::ValueEnum;
use pddllib::{state::State, task::Task};

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum HeuristicKind {
    Constant,
    GoalCount,
}

pub struct Heuristic {
    kind: HeuristicKind,
    estimates: usize,
    time: Duration,
}

impl Heuristic {
    pub fn new(kind: HeuristicKind) -> Self {
        Self {
            kind,
            estimates: 0,
            time: Duration::default(),
        }
    }

    pub fn estimate(&mut self, task: &Task, state: &State) -> usize {
        let t = Instant::now();
        let estimate = match self.kind {
            HeuristicKind::Constant => constant::estimate(task, state),
            HeuristicKind::GoalCount => goal_count::estimate(task, state),
        };
        self.time += t.elapsed();
        self.estimates += 1;
        estimate
    }
}

impl Drop for Heuristic {
    fn drop(&mut self) {
        println!(
            "Heuristic estimates: {} ({:.2}s) ({:.2}/s)",
            self.estimates,
            self.time.as_secs_f64(),
            self.estimates as f64 / self.time.as_secs_f64()
        );
    }
}
