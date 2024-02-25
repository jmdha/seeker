use super::{Result, SearchAlgorithm};
use crate::{
    heuristic::Heuristic,
    search::{trace, Error},
    FxIndexMap,
};
use indexmap::map::Entry::{Occupied, Vacant};
use pddllib::{state::State, successor_generation::successors, task::Task};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Element {
    index: usize,
    estimate: usize,
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .estimate
            .cmp(&self.estimate)
            .then_with(|| self.index.cmp(&other.index))
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
    heuristic: &'static dyn Heuristic,
}

impl LGBFS {
    pub fn new(initial_state: &State, heuristic: &'static dyn Heuristic) -> Self {
        let mut parents = FxIndexMap::default();
        parents.insert(initial_state.clone(), 0);
        Self {
            queue: BinaryHeap::from(vec![Element {
                index: 0,
                estimate: 0,
            }]),
            parents,
            heuristic,
        }
    }
}

impl<'a> SearchAlgorithm<'a> for LGBFS {
    fn step(&mut self, task: &'a Task) -> Option<Result<'a>> {
        let Element { index, estimate: _ } = match self.queue.pop() {
            Some(e) => e,
            None => return Some(Err(Error::Unsolvable)),
        };

        let (node, successors) = {
            let (node, _) = self.parents.get_index(index).unwrap();
            if node.covers(&task.goal) {
                return Some(Ok(trace(&self.parents, index)));
            }
            (node, successors(task, node))
        };

        let estimate = self.heuristic.estimate(node, &task.goal);
        for successor in successors.into_iter() {
            let successor_index = match self.parents.entry(successor) {
                Occupied(_) => continue,
                Vacant(e) => {
                    let n = e.index();
                    e.insert(index);
                    n
                }
            };
            self.queue.push(Element {
                index: successor_index,
                estimate,
            })
        }

        None
    }
}
