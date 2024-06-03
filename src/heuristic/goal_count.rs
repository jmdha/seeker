use super::Heuristic;
use pddllib::{state::State, task::Task};

#[derive(Default, Debug)]
pub struct GoalCount {}

impl Heuristic for GoalCount {
    fn estimate(&self, task: &Task, state: &State) -> usize {
        task.goal
            .iter()
            .filter(|(fact, value)| state.has_fact(task, fact) != *value)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use pddllib::translation::translate;

    use super::*;
    const DOMAIN: &'static str = "(
    (:predicates
        (pred1 ?p)
        (pred2 ?p1 ?p2)
    )
    )";

    #[test]
    fn none() {
        const PROBLEM: &'static str = "(
            (:objects obj1 obj2 obj3 obj4)
            (:init)
            (:goal
                (and
                    (pred1 obj1)
                    (pred1 obj2)
                    (pred2 obj3 obj4)
                    (pred2 obj4 obj3)
                )
            )
        )";
        let task = translate(&DOMAIN, &PROBLEM).unwrap();
        let heuristic = GoalCount::default();
        assert_eq!(4, heuristic.estimate(&task, &task.init));
    }

    #[test]
    fn half() {
        const PROBLEM: &'static str = "(
            (:objects obj1 obj2 obj3 obj4)
            (:init
                (pred1 obj1)
                (pred2 obj3 obj4)
            )
            (:goal
                (and
                    (pred1 obj1)
                    (pred1 obj2)
                    (pred2 obj3 obj4)
                    (pred2 obj4 obj3)
                )
            )
        )";
        let task = translate(&DOMAIN, &PROBLEM).unwrap();
        let heuristic = GoalCount::default();
        assert_eq!(2, heuristic.estimate(&task, &task.init));
    }

    #[test]
    fn full() {
        const PROBLEM: &'static str = "(
            (:objects obj1 obj2 obj3 obj4)
            (:init
                (pred1 obj1)
                (pred1 obj2)
                (pred2 obj3 obj4)
                (pred2 obj4 obj3)
            )
            (:goal
                (and
                    (pred1 obj1)
                    (pred1 obj2)
                    (pred2 obj3 obj4)
                    (pred2 obj4 obj3)
                )
            )
        )";
        let task = translate(&DOMAIN, &PROBLEM).unwrap();
        let heuristic = GoalCount::default();
        assert_eq!(0, heuristic.estimate(&task, &task.init));
    }
}
