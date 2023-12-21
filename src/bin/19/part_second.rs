#[derive(Clone, Debug)]
pub struct PartRange {
    pub x_min: isize,
    pub x_max: isize,
    pub m_min: isize,
    pub m_max: isize,
    pub a_min: isize,
    pub a_max: isize,
    pub s_min: isize,
    pub s_max: isize,
}

impl PartRange {
    pub fn size(&self) -> usize {
        ((self.x_max - self.x_min + 1)
            * (self.m_max - self.m_min + 1)
            * (self.a_max - self.a_min + 1)
            * (self.s_max - self.s_min + 1)) as usize
    }
}

#[derive(Clone, Debug)]
pub enum WorkflowResult {
    Approved,
    Rejected,
    PassTo(String),
}

enum Parameter {
    X,
    M,
    A,
    S,
}

enum Comparison {
    Greater(isize),
    Less(isize),
}

impl Comparison {
    // successful range first, unsuccessful second
    fn split_range(&self, min: isize, max: isize) -> [Option<(isize, isize)>; 2] {
        match self {
            Comparison::Greater(val) => {
                if min > *val {
                    [Some((min, max)), None]
                } else if max > *val {
                    [Some((val + 1, max)), Some((min, *val))]
                } else {
                    [None, Some((min, max))]
                }
            }
            Comparison::Less(val) => {
                if max < *val {
                    [Some((min, max)), None]
                } else if min < *val {
                    [Some((min, val - 1)), Some((*val, max))]
                } else {
                    [None, Some((min, max))]
                }
            }
        }
    }
}

struct WorkflowCondition {
    pub comparison: Option<Comparison>,
    pub parameter: Option<Parameter>,
    pub result: WorkflowResult,
}

pub struct Workflow {
    conditions: Vec<WorkflowCondition>,
}

impl WorkflowCondition {
    pub fn apply(&self, range: &PartRange) -> [Option<PartRange>; 2] {
        if self.comparison.is_none() {
            return [Some(range.clone()), None];
        }
        match self.parameter.as_ref().unwrap() {
            Parameter::X => {
                let split = self
                    .comparison
                    .as_ref()
                    .unwrap()
                    .split_range(range.x_min, range.x_max);
                let mut results: [Option<PartRange>; 2] = [None, None];
                for i in 0..=1 {
                    if let Some((min, max)) = split[i] {
                        results[i] = Some(PartRange {
                            x_min: min,
                            x_max: max,
                            ..*range
                        })
                    }
                }
                results
            }
            Parameter::M => {
                let split = self
                    .comparison
                    .as_ref()
                    .unwrap()
                    .split_range(range.m_min, range.m_max);
                let mut results: [Option<PartRange>; 2] = [None, None];
                for i in 0..=1 {
                    if let Some((min, max)) = split[i] {
                        results[i] = Some(PartRange {
                            m_min: min,
                            m_max: max,
                            ..*range
                        })
                    }
                }
                results
            }
            Parameter::A => {
                let split = self
                    .comparison
                    .as_ref()
                    .unwrap()
                    .split_range(range.a_min, range.a_max);
                let mut results: [Option<PartRange>; 2] = [None, None];
                for i in 0..=1 {
                    if let Some((min, max)) = split[i] {
                        results[i] = Some(PartRange {
                            a_min: min,
                            a_max: max,
                            ..*range
                        })
                    }
                }
                results
            }
            Parameter::S => {
                let split = self
                    .comparison
                    .as_ref()
                    .unwrap()
                    .split_range(range.s_min, range.s_max);
                let mut results: [Option<PartRange>; 2] = [None, None];
                for i in 0..=1 {
                    if let Some((min, max)) = split[i] {
                        results[i] = Some(PartRange {
                            s_min: min,
                            s_max: max,
                            ..*range
                        })
                    }
                }
                results
            }
        }
    }
}

impl Workflow {
    pub fn from_conditions(input: &str) -> Workflow {
        let mut workflow = Workflow { conditions: vec![] };

        for condition_str in input.split(',') {
            let label = condition_str.split(':').last().unwrap();
            let result = match label {
                "A" => WorkflowResult::Approved,
                "R" => WorkflowResult::Rejected,
                x => WorkflowResult::PassTo(String::from(x)),
            };
            if !condition_str.contains(':') {
                workflow.conditions.push(WorkflowCondition {
                    comparison: None,
                    parameter: None,
                    result,
                });
                continue;
            }
            let value = condition_str
                .split(['>', '<', ':'])
                .nth(1)
                .unwrap()
                .parse::<isize>()
                .unwrap();
            let comparison = if condition_str.contains('>') {
                Comparison::Greater(value)
            } else {
                Comparison::Less(value)
            };
            let parameter = match condition_str.chars().next().unwrap() {
                'x' => Parameter::X,
                'm' => Parameter::M,
                'a' => Parameter::A,
                's' => Parameter::S,
                c => panic!("unexpected char in input: `{}`", c),
            };
            workflow.conditions.push(WorkflowCondition {
                comparison: Some(comparison),
                parameter: Some(parameter),
                result,
            });
        }

        workflow
    }

    pub fn apply(&self, range: &PartRange) -> Vec<(PartRange, WorkflowResult)> {
        let mut results = vec![];
        let mut passing_range = Some(range.clone());

        for condition in &self.conditions {
            if passing_range.is_none() {
                break;
            }
            let [successful, failed] = condition.apply(passing_range.as_ref().unwrap());
            if let Some(good_range) = successful {
                results.push((good_range, condition.result.clone()));
            }
            if let Some(bad_range) = failed {
                passing_range = Some(bad_range);
            }
        }

        results
    }
}
