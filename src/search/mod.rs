pub mod bfs;
pub mod dfs;
pub mod gbfs;
pub mod lgbfs;

use crate::heuristic::{Heuristic, HeuristicKind};
use clap::Subcommand;
use memory_stats::memory_stats;
use pddllib::{state::State, task::Task};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub enum Error {
    Unsolvable,
    OutOfTime,
    OutOfMemory,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SearchKind {
    /// Breadth First Search
    BFS,
    /// Depth First Search
    DFS,
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsolvable => write!(f, "Unsolvable"),
            Error::OutOfTime => write!(f, "Out of time"),
            Error::OutOfMemory => write!(f, "Out of memory"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<'a> = std::result::Result<Vec<State>, Error>;

pub trait SearchAlgorithm<'a> {
    fn step(&mut self, task: &'a Task) -> Option<Result<'a>>;
}

pub fn generate<'a>(task: &'a Task, search: &'a SearchKind) -> Box<dyn SearchAlgorithm<'a>> {
    match search {
        SearchKind::BFS => Box::new(bfs::BFS::new(&task.init)),
        SearchKind::DFS => Box::new(dfs::DFS::new(&task.init)),
        SearchKind::GBFS { heuristic } => {
            Box::new(gbfs::GBFS::new(&task.init, Heuristic::new(*heuristic)))
        }
        SearchKind::LGBFS { heuristic } => {
            Box::new(lgbfs::LGBFS::new(&task.init, Heuristic::new(*heuristic)))
        }
    }
}

pub fn solve<'a>(
    task: &'a Task,
    time_limit: Option<Duration>,
    memory_limit: Option<usize>,
    searcher: &mut Box<dyn SearchAlgorithm<'a>>,
) -> Result<'a> {
    let start = Instant::now();
    let result: Result<'a>;
    let mut peak_memory = 0;
    let mut steps = 0;
    loop {
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
        if let Some(search_result) = searcher.step(task) {
            result = search_result;
            break;
        }
        steps += 1;
    }
    println!("Peak memory: {}MB", peak_memory / 1000000);
    println!("Steps: {}", steps);
    result
}
