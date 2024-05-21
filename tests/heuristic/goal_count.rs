use rstest::rstest;
use seeker::heuristic::{goal_count::GoalCount, Heuristic};

use crate::gripper;

#[rstest]
#[case(gripper::DOMAIN, gripper::PROBLEM_UNSOLVED, 4)]
#[case(gripper::DOMAIN, gripper::PROBLEM_HALFSOLVED, 2)]
#[case(gripper::DOMAIN, gripper::PROBLEM_SOLVED, 0)]
fn estimate(#[case] domain: &str, #[case] problem: &str, #[case] expected: usize) {
    let task = pddllib::translation::translate(domain, problem).unwrap();
    let actual = GoalCount::new().estimate(&task.init, &task.goal);
    assert_eq!(expected, actual);
}
