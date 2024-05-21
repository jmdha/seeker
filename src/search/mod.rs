pub mod lgbfs;

use memory_stats::memory_stats;
use pddllib::{state::State, task::Task};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::FxIndexMap;

#[derive(Debug)]
pub enum Error {
    Unsolvable,
    OutOfTime,
    OutOfMemory,
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

pub fn solve<'a>(
    task: &'a Task,
    time_limit: Option<Duration>,
    memory_limit: Option<usize>,
    searcher: &mut Box<impl SearchAlgorithm<'a>>,
) -> Result<'a> {
    let start = Instant::now();
    let result: Result<'a>;
    let mut steps = 0;
    loop {
        steps += 1;
        if let Some(time_limit) = time_limit {
            let elapsed = start.elapsed();
            if elapsed > time_limit {
                result = Err(Error::OutOfTime);
                break;
            }
        }
        if steps % 256 == 0 {
            if let Some(memory_limit) = memory_limit {
                if let Some(usage) = memory_stats() {
                    if usage.physical_mem > memory_limit * 1000000 {
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
    }
    println!("steps: {}", steps);
    result
}

fn trace(parents: &FxIndexMap<State, usize>, goal_index: usize) -> Vec<State> {
    let mut i = goal_index;
    let mut states = vec![];
    loop {
        let (state, parent) = parents.get_index(i).unwrap();
        states.push(state.to_owned());
        i = *parent;
        if i == 0 {
            states.push(parents.get_index(*parent).unwrap().0.to_owned());
            break;
        }
    }
    states.reverse();
    states
}
