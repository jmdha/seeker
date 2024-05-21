use super::{Result, SearchAlgorithm};
use crate::{heuristic::Heuristic, search::Error, trace, FxIndexMap};
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

pub struct GBFS {
    queue: BinaryHeap<Element>,
    parents: FxIndexMap<State, usize>,
    heuristic: Box<dyn Heuristic>,
}

impl GBFS {
    pub fn new(initial_state: &State, heuristic: Box<dyn Heuristic>) -> Self {
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

impl<'a> SearchAlgorithm<'a> for GBFS {
    fn step(&mut self, task: &'a Task) -> Option<Result<'a>> {
        let Element { index, estimate: _ } = match self.queue.pop() {
            Some(e) => e,
            None => return Some(Err(Error::Unsolvable)),
        };

        let successors = {
            let (node, _) = self.parents.get_index(index).unwrap();
            if node.covers(&task.goal) {
                return Some(Ok(trace(&self.parents, index)));
            }
            successors(task, node)
        };

        for successor in successors.into_iter() {
            let estimate = self.heuristic.estimate(task, &successor);
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
