use pddllib::{state::State, task::Goal};

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
    fn estimate(&mut self, state: &State, goal: &Goal) -> usize {
        let estimate = goal
            .iter()
            .filter(|(fact, value)| state.has_nary(fact.predicate, &fact.args) != *value)
            .count();
        if estimate < self.best_estimate {
            println!("new best heuristic estimate: {}", estimate);
            self.best_estimate = estimate;
        }
        self.estimates += 1;
        estimate
    }
}

impl Drop for GoalCount {
    fn drop(&mut self) {
        println!("evaluated {} states", self.estimates);
    }
}
