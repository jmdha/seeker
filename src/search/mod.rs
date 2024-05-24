mod bfs;
pub mod error;
mod gbfs;
mod lgbfs;

use self::error::Error;
use crate::heuristic::{Heuristic, HeuristicKind};
use clap::Subcommand;
use memory_stats::memory_stats;
use pddllib::{state::State, task::Task};
use std::time::{Duration, Instant};

#[derive(Subcommand, Debug, Clone)]
pub enum SearchKind {
    BFS,
    /// Greedy Best First Search
    GBFS {
        #[arg(default_value = "goal-count")]
        heuristic: HeuristicKind,
    },
    /// Lazy Greedy Best First Search
    LGBFS {
        #[arg(default_value = "goal-count")]
        heuristic: HeuristicKind,
    },
}

pub type Result<'a> = std::result::Result<Vec<State>, Error>;

pub trait SearchAlgorithm<'a> {
    fn step(&mut self, task: &'a Task) -> Result<'a>;
}

pub fn generate<'a>(task: &'a Task, search: &'a SearchKind) -> Box<dyn SearchAlgorithm<'a>> {
    match search {
        SearchKind::BFS => Box::new(bfs::BFS::new(&task.init)),
        SearchKind::GBFS { heuristic } => Box::new(gbfs::GBFS::new(&task.init, Heuristic::new(*heuristic))),
        SearchKind::LGBFS { heuristic } => Box::new(lgbfs::LGBFS::new(&task.init, Heuristic::new(*heuristic))),
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
