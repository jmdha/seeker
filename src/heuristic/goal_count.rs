use pddllib::{state::State, task::Task};

use super::Heuristic;

#[derive(Debug, Default)]
pub struct GoalCount {
    best_estimate: usize,
    estimates: usize,
}

impl GoalCount {
    pub fn new() -> Self {
        Self {
            best_estimate: usize::max_value(),
            estimates: 0,
        }
    }
}

impl Heuristic for GoalCount {
    fn estimate(&mut self, task: &Task, state: &State) -> usize {
        let estimate = task
            .goal
            .iter()
            .filter(|(fact, value)| state.has_fact(task, fact) != *value)
            .count();
        if estimate < self.best_estimate {
            println!("New best heuristic estimate: {}", estimate);
            self.best_estimate = estimate;
        }
        self.estimates += 1;
        estimate
    }
}

impl Drop for GoalCount {
    fn drop(&mut self) {
        println!("Evaluated {} states", self.estimates);
    }
}
