use super::SearchAlgorithm;
use crate::{search::Error, trace, FxIndexMap};
use indexmap::map::Entry::Vacant;
use pddllib::{state::State, successor_generation::successors};

pub struct BFS {
    index: usize,
    parents: FxIndexMap<State, usize>,
}

impl BFS {
    pub fn new(initial_state: &State) -> Self {
        let mut parents = FxIndexMap::default();
        parents.insert(initial_state.clone(), 0);
        Self { index: 0, parents }
    }
}

impl SearchAlgorithm for BFS {
    fn step(&mut self, task: &pddllib::task::Task) -> super::Result {
        let (node, _) = self.parents.get_index(self.index).ok_or(Error::Unsolvable)?;
        for successor in successors(task, node) {
            if successor.covers(task, &task.goal) {
                let mut path = trace(&self.parents, self.index);
                path.push(successor);
                return Ok(path);
            }
            if let Vacant(e) = self.parents.entry(successor) {
                e.insert(self.index);
            }
        }
        self.index += 1;
        Err(Error::Unfinished)
    }
}
