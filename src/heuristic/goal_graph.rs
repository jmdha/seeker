use std::collections::HashMap;

use super::Heuristic;
use itertools::Itertools;
use pddllib::{
    state::{Fact, State},
    task::{
        action::{Action, Argument, Atom},
        parameter::Parameter,
        Task,
    },
};

struct Node {
    fact: (Fact, bool),
    weight: usize,
    children: Vec<Node>,
}

pub struct GoalGraph {
    goals: Vec<Node>,
}

impl GoalGraph {
    pub fn new(task: &Task, weight: usize) -> Self {
        let goals = task.goal.iter().map(|g| generate(task, g.clone(), weight)).collect();
        let graph = Self { goals };
        println!("Graph size: {}", graph.size());
        graph
    }

    fn size(&self) -> usize {
        let mut size = 0;
        let mut queue: Vec<&Node> = self.goals.iter().collect();
        while let Some(node) = queue.pop() {
            size += 1;
            queue.extend(node.children.iter());
        }
        size
    }
}

impl Heuristic for GoalGraph {
    fn estimate(&self, task: &Task, state: &State) -> usize {
        let mut estimate = 0;
        let mut queue: Vec<&Node> = self.goals.iter().collect();
        while let Some(node) = queue.pop() {
            let (fact, value) = &node.fact;
            if state.has_fact(task, fact) == *value {
                continue;
            }
            if node.children.is_empty() {
                estimate += node.weight;
            } else {
                queue.extend(node.children.iter());
            }
        }
        estimate
    }
}

fn generate(task: &Task, fact: (Fact, bool), weight: usize) -> Node {
    let facts: Vec<_> = task
        .actions
        .iter()
        .filter_map(|a| {
            match a
                .effect
                .iter()
                .find(|atom| atom.predicate == fact.0.predicate() && atom.value == fact.1)
            {
                Some(atom) => Some((a, atom)),
                None => None,
            }
        })
        .flat_map(|(action, atom)| extract_facts(task, &fact.0, action, atom))
        .collect();
    let children;
    // Cannot have a decimal weight
    if weight >= facts.len() && !facts.is_empty() {
        let weight = weight / facts.len();
        children = facts.into_iter().map(|fact| generate(task, fact, weight)).collect();
    } else {
        children = vec![];
    }
    Node { fact, weight, children }
}

fn extract_facts(task: &Task, fact: &Fact, action: &Action, atom: &Atom) -> Vec<(Fact, bool)> {
    let fixed = atom
        .args
        .iter()
        .enumerate()
        .filter_map(|(i, arg)| match arg {
            Argument::Index(p_i) => Some((*p_i, fact.args()[i])),
            Argument::Const(_) => None,
        })
        .collect();
    action
        .precondition
        .iter()
        .filter(|atom| !task.static_predicates.contains(&atom.predicate))
        .flat_map(|atom| {
            populate_atom(task, &action.parameters, atom, &fixed)
                .into_iter()
                .map(|objects| (Fact::new(atom.predicate, objects), atom.value))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn populate_atom(
    task: &Task,
    parameters: &Vec<Parameter>,
    atom: &Atom,
    fixed: &HashMap<usize, usize>,
) -> Vec<Vec<usize>> {
    atom.args
        .iter()
        .map(|arg| match arg {
            Argument::Index(i) => match fixed.get(i) {
                Some(i) => vec![*i],
                None => task.objects_typed[parameters[*i].type_index].to_owned(),
            },
            Argument::Const(i) => vec![*i],
        })
        .multi_cartesian_product()
        .collect()
}
