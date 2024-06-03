use crate::{evaluator::Evaluator, trace, FxIndexMap};

use super::{error::Error, SearchAlgorithm};
use indexmap::map::Entry::Vacant;
use pddllib::{state::State, successor_generation::successors};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Element {
    index: usize,
    estimate: usize,
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimate.cmp(&self.estimate)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct LGBFS {
    queue: BinaryHeap<Element>,
    parents: FxIndexMap<State, usize>,
    heuristic: Evaluator,
}

impl LGBFS {
    pub fn new(initial_state: &State, heuristic: Evaluator) -> Self {
        let mut parents = FxIndexMap::default();
        parents.insert(initial_state.clone(), 0);
        Self {
            queue: BinaryHeap::from(vec![Element { index: 0, estimate: 0 }]),
            parents,
            heuristic,
        }
    }
}

impl SearchAlgorithm for LGBFS {
    fn step(&mut self, task: &pddllib::task::Task) -> super::Result {
        let Element { index, estimate: _ } = self.queue.pop().ok_or(Error::Unsolvable)?;
        let (node, _) = self.parents.get_index(index).unwrap();
        if node.covers(task, &task.goal) {
            println!("States generated: {}", self.parents.len());
            return Ok(trace(&self.parents, index));
        }
        let estimate = self.heuristic.estimate(task, node);
        for successor in successors(task, node) {
            let s_index;
            if let Vacant(e) = self.parents.entry(successor) {
                s_index = e.index();
                e.insert(index);
            } else {
                continue;
            }
            self.queue.push(Element {
                index: s_index,
                estimate,
            })
        }
        Err(Error::Unfinished)
    }
}
