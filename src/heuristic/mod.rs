mod add;
mod constant;
mod goal_count;
mod goal_graph;

use clap::Subcommand;
use pddllib::{state::State, task::Task};

/// Heuristic
#[derive(Subcommand, Debug, Clone)]
pub enum HeuristicKind {
    /// Simply returns the provided value
    Constant {
        #[arg(default_value_t = 0)]
        value: usize,
    },
    /// Returns the number of goal facts not in the state
    GoalCount,
    GoalGraph {
        #[arg(default_value_t = 100)]
        weight: usize,
    },
    /// Returns the sum of actions required to achieve each goal fact individually in a relaxed
    /// context
    Add,
}

pub trait Heuristic {
    fn estimate(&self, task: &Task, state: &State) -> usize;
}

pub fn generate(task: &Task, kind: HeuristicKind) -> Box<dyn Heuristic> {
    match kind {
        HeuristicKind::Constant { value } => Box::new(constant::Constant::new(value)),
        HeuristicKind::GoalCount => Box::new(goal_count::GoalCount::default()),
        HeuristicKind::GoalGraph { weight } => Box::new(goal_graph::GoalGraph::new(task, weight)),
        HeuristicKind::Add => Box::new(add::Add::new()),
    }
}
