use crate::part_first::{Part, Workflow, WorkflowResult};
use itertools::Itertools;
use std::collections::HashMap;

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
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let (x, m, a, s) = line[1..line.len() - 1]
            .split(',')
            .map(|x| x[2..].parse::<isize>().unwrap())
            .next_tuple()
            .unwrap();
        let part = Part { x, m, a, s };
        let mut current_workflow = String::from("in");
        loop {
            let result = workflows[&current_workflow].apply(&part);
            if let WorkflowResult::Approved = result {
                sum += part.component_sum();
                break;
            } else if let WorkflowResult::Rejected = result {
                break;
            } else if let WorkflowResult::PassTo(next_workflow) = result {
                current_workflow = next_workflow;
            }
        }
    }

    println!("{}", sum);
}
