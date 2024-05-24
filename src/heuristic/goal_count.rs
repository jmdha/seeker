use pddllib::{state::State, task::Task};

pub fn estimate(task: &Task, state: &State) -> usize {
    task.goal
        .iter()
        .filter(|(fact, value)| state.has_fact(task, fact) != *value)
        .count()
}
