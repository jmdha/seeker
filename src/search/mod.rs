mod bfs;
pub mod error;
mod gbfs;
mod lgbfs;

use crate::{
    evaluator::{self, Evaluator},
    heuristic::HeuristicKind,
};

use self::error::Error;
use clap::Subcommand;
use memory_stats::memory_stats;
use pddllib::{state::State, task::Task};
use std::time::{Duration, Instant};

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

pub type Result<'a> = std::result::Result<Vec<State>, Error>;

pub trait SearchAlgorithm<'a> {
    fn step(&mut self, task: &'a Task) -> Result<'a>;
}

pub fn generate<'a>(task: &'a Task, search: SearchKind) -> Box<dyn SearchAlgorithm<'a>> {
    match search {
        SearchKind::BFS => Box::new(bfs::BFS::new(&task.init)),
        SearchKind::GBFS { heuristic } => Box::new(gbfs::GBFS::new(&task.init, Evaluator::new(&task, heuristic))),
        SearchKind::LGBFS { heuristic } => Box::new(lgbfs::LGBFS::new(&task.init, Evaluator::new(&task, heuristic))),
    }
}

pub fn solve<'a>(
    task: &'a Task,
    time_limit: Option<Duration>,
    memory_limit: Option<usize>,
    searcher: &mut Box<dyn SearchAlgorithm<'a>>,
) -> Result<'a> {
    let start = Instant::now();
    let mut result: Result<'a>;
    let mut peak_memory = 0;
    let mut steps = 0;
    loop {
        result = searcher.step(task);
        steps += 1;
        if result.is_ok() {
            break;
        }
        if let Some(time_limit) = time_limit {
            let elapsed = start.elapsed();
            if elapsed > time_limit {
                result = Err(Error::OutOfTime);
                break;
            }
        }
        if steps % 16 == 0 {
            if let Some(usage) = memory_stats() {
                let usage = usage.physical_mem;
                if usage > peak_memory {
                    peak_memory = usage;
                }
                if let Some(memory_limit) = memory_limit {
                    if usage > memory_limit * 1000000 {
                        result = Err(Error::OutOfMemory);
                        break;
                    }
                }
            }
        }
    }
    println!("Peak memory: {}MB", peak_memory / 1000000);
    println!("Steps: {}", steps);
    result
}
