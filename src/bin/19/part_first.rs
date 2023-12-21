pub struct Part {
    pub x: isize,
    pub m: isize,
    pub a: isize,
    pub s: isize,
}

#[derive(Clone)]
pub enum WorkflowResult {
    Approved,
    Rejected,
    PassTo(String),
}

pub struct Workflow {
    // TODO: learn more about closures and do/understand this properly
    conditions: Vec<(Box<dyn Fn(&Part) -> bool>, WorkflowResult)>,
}

impl Part {
    pub fn component_sum(&self) -> isize {
        self.x + self.m + self.a + self.s
    }
}

impl Workflow {
    pub fn from_conditions(input: &str) -> Workflow {
        let mut workflow = Workflow { conditions: vec![] };

        for condition_str in input.split(',') {
            let greater_than = condition_str.contains('>');
            let label = condition_str.split(':').last().unwrap();
            let result = match label {
                "A" => WorkflowResult::Approved,
                "R" => WorkflowResult::Rejected,
                x => WorkflowResult::PassTo(String::from(x)),
            };
            if !condition_str.contains(':') {
                workflow.conditions.push((Box::new(|_| true), result));
                continue;
            }
            let value = condition_str
                .split(['>', '<', ':'])
                .nth(1)
                .unwrap()
                .parse::<isize>()
                .unwrap();
            match condition_str.chars().next().unwrap() {
                'x' => {
                    if greater_than {
                        workflow
                            .conditions
                            .push((Box::new(move |part| part.x > value), result));
                    } else {
                        workflow
                            .conditions
                            .push((Box::new(move |part| part.x < value), result));
                    }
                }
                'm' => {
                    if greater_than {
                        workflow
                            .conditions
                            .push((Box::new(move |part| part.m > value), result));
                    } else {
                        workflow
                            .conditions
                            .push((Box::new(move |part| part.m < value), result));
                    }
                }
                'a' => {
                    if greater_than {
                        workflow
                            .conditions
                            .push((Box::new(move |part| part.a > value), result));
                    } else {
                        workflow
                            .conditions
                            .push((Box::new(move |part| part.a < value), result));
                    }
                }
                's' => {
                    if greater_than {
                        workflow
                            .conditions
                            .push((Box::new(move |part| part.s > value), result));
                    } else {
                        workflow
                            .conditions
                            .push((Box::new(move |part| part.s < value), result));
                    }
                }
                c => panic!("unexpected char in input `{}`", c),
            }
        }

        workflow
    }

    pub fn apply(&self, part: &Part) -> WorkflowResult {
        for condition in &self.conditions {
            if condition.0(part) {
                return condition.1.clone();
            }
        }

        unreachable!()
    }
}
