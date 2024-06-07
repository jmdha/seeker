pub mod bfs;
pub mod error;
pub mod gbfs;
pub mod lgbfs;

use crate::{evaluator::Evaluator, heuristic::HeuristicKind};

use self::error::Error;
use clap::Subcommand;
use metered::{hdr_histogram::AtomicHdrHistogram, measure, time_source::StdInstantMicros, HitCount, ResponseTime};
use pddllib::{state::State, task::Task};

/// Search Algorithm
#[derive(Subcommand, Debug, Clone)]
pub enum SearchKind {
    /// Breadth First Search
    BFS,
    /// Greedy Best First Search
    GBFS {
        #[command(subcommand)]
        heuristic: HeuristicKind,
    },
    /// Lazy Greedy Best First Search
    LGBFS {
        #[command(subcommand)]
        heuristic: HeuristicKind,
    },
}

pub type Result = std::result::Result<Vec<State>, Error>;

pub trait SearchAlgorithm {
    fn step(&mut self, task: &Task) -> Result;
}

pub fn generate<'a>(task: &'a Task, search: SearchKind) -> Box<dyn SearchAlgorithm> {
    match search {
        SearchKind::BFS => Box::new(bfs::BFS::new(&task.init)),
        SearchKind::GBFS { heuristic } => Box::new(gbfs::GBFS::new(&task.init, Evaluator::new(&task, heuristic))),
        SearchKind::LGBFS { heuristic } => Box::new(lgbfs::LGBFS::new(&task.init, Evaluator::new(&task, heuristic))),
    }
}

pub fn solve(task: &Task, searcher: &mut Box<dyn SearchAlgorithm>) -> Result {
    let hits: HitCount = HitCount::default();
    let hit_time: ResponseTime<AtomicHdrHistogram, StdInstantMicros> =
        ResponseTime::<AtomicHdrHistogram, StdInstantMicros>::default();
    let mut result: Result;
    loop {
        measure!(&hits, {
            measure!(&hit_time, {
                result = searcher.step(task);
            });
        });
        if result.is_ok() {
            break;
        }
    }
    println!("Steps: {}", hits.0.get());
    println!(
        "Step time: mean {:.2}us min {}us max {}us stdev {:.2}",
        hit_time.histogram().mean(),
        hit_time.histogram().min(),
        hit_time.histogram().max(),
        hit_time.histogram().stdev()
    );
    result
}
