pub mod bfs;
pub mod dfs;
pub mod lgbfs;

use clap::Subcommand;
use memory_stats::memory_stats;
use pddllib::{state::State, task::Task};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::heuristic::{self, HeuristicKind};

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
    /// Lazy Greedy Best First Search
    LGBFS {
        #[arg(default_value = "goal-count")]
        heuristic: HeuristicKind,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsolvable => write!(f, "unsolvable"),
            Error::OutOfTime => write!(f, "out of time"),
            Error::OutOfMemory => write!(f, "out of memory"),
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
        SearchKind::LGBFS { heuristic: h } => {
            Box::new(lgbfs::LGBFS::new(&task.init, heuristic::generate(task, h)))
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
    let mut steps: u128 = 0;
    loop {
        if let Some(time_limit) = time_limit {
            let elapsed = start.elapsed();
            if elapsed > time_limit {
                result = Err(Error::OutOfTime);
                break;
            }
        }
        if let Some(memory_limit) = memory_limit {
            if let Some(usage) = memory_stats() {
                if usage.physical_mem > memory_limit * 1000000 {
                    result = Err(Error::OutOfMemory);
                    break;
                }
            }
        }
        if let Some(search_result) = searcher.step(task) {
            result = search_result;
            break;
        }
        steps += 1;
    }
    println!("steps: {}", steps);
    result
}
