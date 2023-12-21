use std::collections::{HashMap, VecDeque};

pub fn run(input: &str) {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut conjunctions: HashMap<&str, (Vec<String>, Vec<String>)> = HashMap::new();
    for declaration in input.lines() {
        let name = &declaration.split_whitespace().next().unwrap()[1..];
        let outputs = declaration
            .split(" -> ")
            .last()
            .unwrap()
            .split(", ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        match declaration.chars().next().unwrap() {
            '%' => {
                for output in &outputs {
                    if conjunctions.contains_key(output.as_str()) {
                        conjunctions
                            .get_mut(output.as_str())
                            .unwrap()
                            .1
                            .push(String::from(name));
                    }
                }
                modules.insert(
                    String::from(name),
                    Box::new(FlipFlop::with_details(name, outputs.clone())),
                );
            }
            '&' => {
                conjunctions.insert(name, (outputs, vec![]));
            }
            'b' => {
                for output in &outputs {
                    if conjunctions.contains_key(output.as_str()) {
                        conjunctions
                            .get_mut(output.as_str())
                            .unwrap()
                            .1
                            .push(String::from("broadcaster"));
                    }
                }
                modules.insert(
                    String::from("broadcaster"),
                    Box::new(Broadcaster::with_outputs(outputs.clone())),
                );
            }
            c => panic!("unexpected char in input `{}`", c),
        };
    }
    for (name, (outputs, inputs)) in conjunctions {
        modules.insert(
            String::from(name),
            Box::new(Conjunction::with_details(name, outputs, inputs)),
        );
    }

    let mut pulses = VecDeque::from([Pulse {
        is_high: false,
        from: String::from("button"),
        to: String::from("broadcaster"),
    }]);
    let mut count = 0;
    while let Some(current_pulse) = pulses.pop_front() {
        count += 1;
        for output in modules
            .get_mut(&current_pulse.to)
            .unwrap()
            .process_pulse(current_pulse)
        {
            pulses.push_back(output);
        }
    }
    println!("{}", count);
}

struct Pulse {
    pub is_high: bool,
    pub from: String,
    pub to: String,
}

trait Module {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse>;
}

struct FlipFlop {
    name: String,
    is_on: bool,
    outputs: Vec<String>,
}

struct Conjunction {
    name: String,
    input_states: HashMap<String, bool>,
    outputs: Vec<String>,
}

struct Broadcaster {
    outputs: Vec<String>,
}

impl FlipFlop {
    fn with_details(name: &str, outputs: Vec<String>) -> Self {
        FlipFlop {
            name: String::from(name),
            is_on: false,
            outputs,
        }
    }
}

impl Module for FlipFlop {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.is_high {
            return vec![];
        }
        self.is_on = !self.is_on;
        self.outputs
            .iter()
            .map(|output| Pulse {
                is_high: self.is_on,
                from: self.name.clone(),
                to: output.clone(),
            })
            .collect()
    }
}

impl Conjunction {
    fn with_details(name: &str, outputs: Vec<String>, inputs: Vec<String>) -> Self {
        Conjunction {
            name: String::from(name),
            input_states: inputs.iter().map(|input| (input.clone(), false)).collect(),
            outputs,
        }
    }
}

impl Module for Conjunction {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.input_states.insert(pulse.from, pulse.is_high);
        let strength = !self.input_states.iter().all(|(_, high)| *high);
        self.outputs
            .iter()
            .map(|output| Pulse {
                is_high: strength,
                from: self.name.clone(),
                to: output.clone(),
            })
            .collect()
    }
}

impl Broadcaster {
    fn with_outputs(outputs: Vec<String>) -> Self {
        Broadcaster { outputs }
    }
}

impl Module for Broadcaster {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.outputs
            .iter()
            .map(|output| Pulse {
                is_high: pulse.is_high,
                from: String::from("broadcaster"),
                to: output.clone(),
            })
            .collect()
    }
}
