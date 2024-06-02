use crate::heuristic::{self, Heuristic, HeuristicKind};
use pddllib::{state::State, task::Task};
use std::time::{Duration, Instant};

pub struct Evaluator {
    heuristic: Box<dyn Heuristic>,
    estimates: usize,
    best_estimate: usize,
    time: Duration,
}

impl Evaluator {
    pub fn new(task: &Task, kind: HeuristicKind) -> Self {
        println!("Generating evaluator...");
        let t = Instant::now();
        let heuristic = heuristic::generate(task, kind);
        println!("Heuristic init time: {}s", t.elapsed().as_secs_f64());
        Self {
            heuristic,
            estimates: 0,
            best_estimate: usize::MAX,
            time: Duration::default(),
        }
    }

    pub fn estimate(&mut self, task: &Task, state: &State) -> usize {
        let t = Instant::now();
        let estimate = self.heuristic.estimate(task, state);
        self.time += t.elapsed();
        self.estimates += 1;
        if estimate < self.best_estimate {
            println!("New best heuristic estimate: {}", estimate);
            self.best_estimate = estimate;
        }
        estimate
    }
}

impl Drop for Evaluator {
    fn drop(&mut self) {
        println!(
            "Heuristic estimates: {} ({:.2}s) ({:.2}/s)",
            self.estimates,
            self.time.as_secs_f64(),
            self.estimates as f64 / self.time.as_secs_f64()
        );
    }
}
