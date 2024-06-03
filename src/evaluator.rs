use crate::heuristic::{self, Heuristic, HeuristicKind};
use metered::{
    clear::Clear, hdr_histogram::AtomicHdrHistogram, metered, time_source::StdInstantMicros, HitCount, ResponseTime,
};
use pddllib::{state::State, task::Task};

pub struct Evaluator {
    metrics: EvaluatorMetrics,
    heuristic: Box<dyn Heuristic>,
}

#[metered(registry = EvaluatorMetrics)]
impl Evaluator {
    pub fn new(task: &Task, kind: HeuristicKind) -> Self {
        let heuristic = heuristic::generate(task, kind);
        Self {
            heuristic,
            metrics: EvaluatorMetrics::default(),
        }
    }

    pub fn clear(&self) {
        self.metrics.clear();
    }

    #[measure([HitCount, ResponseTime<AtomicHdrHistogram, StdInstantMicros>])]
    pub fn estimate(&self, task: &Task, state: &State) -> usize {
        self.heuristic.estimate(task, state)
    }
}

impl Drop for Evaluator {
    fn drop(&mut self) {
        println!("Evaluations: {}", self.metrics.estimate.hit_count.0.get());
        println!(
            "Evaluation time: mean {:.2}us min {}us max {}us stdev {:.2}",
            self.metrics.estimate.response_time.histogram().mean(),
            self.metrics.estimate.response_time.histogram().min(),
            self.metrics.estimate.response_time.histogram().max(),
            self.metrics.estimate.response_time.histogram().stdev(),
        );
    }
}
