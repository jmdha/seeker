use std::collections::VecDeque;

use super::SearchAlgorithm;
use crate::{
    search::{trace, Error},
    FxIndexMap,
};
use indexmap::map::Entry::{Occupied, Vacant};
use pddllib::{state::State, successor_generation::successors};

pub struct BFS {
    stack: VecDeque<usize>,
    parents: FxIndexMap<State, usize>,
}

impl BFS {
    pub fn new(initial_state: &State) -> Self {
        let mut parents = FxIndexMap::default();
        parents.insert(initial_state.clone(), 0);
        Self {
            stack: VecDeque::from([0]),
            parents,
        }
    }
}

impl<'a> SearchAlgorithm<'a> for BFS {
    fn step(&mut self, task: &'a pddllib::task::Task) -> Option<super::Result<'a>> {
        let index = match self.stack.pop_front() {
            Some(index) => index,
            None => return Some(Err(Error::Unsolvable)),
        };
        let (_, successors) = {
            let (node, _) = self.parents.get_index(index).unwrap();
            if node.covers(&task.goal) {
                return Some(Ok(trace(&self.parents, index)));
            }
            (node, successors(task, node))
        };
        for successor in successors.into_iter() {
            let successor_index = match self.parents.entry(successor) {
                Occupied(_) => continue,
                Vacant(e) => {
                    let n = e.index();
                    e.insert(index);
                    n
                }
            };
            self.stack.push_back(successor_index);
        }
        None
    }
}
