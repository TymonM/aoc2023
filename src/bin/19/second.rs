use crate::part_second::{PartRange, Workflow, WorkflowResult};
use itertools::Itertools;
use std::collections::HashMap;

const RANGE_MIN: isize = 1;
const RANGE_MAX: isize = 4000;

pub fn run(input: &str) {
    let mut workflows = HashMap::new();
    let mut lines = input.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (label, conditions) = line.split(['{', '}']).next_tuple().unwrap();
        workflows.insert(String::from(label), Workflow::from_conditions(conditions));
    }

    let mut sum = 0;
    let mut tasks = vec![(
        PartRange {
            x_min: RANGE_MIN,
            m_min: RANGE_MIN,
            a_min: RANGE_MIN,
            s_min: RANGE_MIN,
            x_max: RANGE_MAX,
            m_max: RANGE_MAX,
            a_max: RANGE_MAX,
            s_max: RANGE_MAX,
        },
        WorkflowResult::PassTo(String::from("in")),
    )];
    while let Some(current_task) = tasks.pop() {
        match current_task.1 {
            WorkflowResult::Approved => sum += current_task.0.size(),
            WorkflowResult::Rejected => (),
            WorkflowResult::PassTo(next) => {
                // dbg!(&next);
                // tasks.append(dbg!(&mut workflows[&next].apply(&current_task.0)));
                tasks.append(&mut workflows[&next].apply(&current_task.0));
            }
        }
    }

    println!("{}", sum);
}
