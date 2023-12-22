use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Pulse {
    pub is_high: bool,
    pub from: String,
    pub to: String,
}

// TODO: learn more about traits and do this properly
pub trait Module {
    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse>;
}

pub struct FlipFlop {
    name: String,
    is_on: bool,
    outputs: Vec<String>,
}

pub struct Conjunction {
    name: String,
    input_states: HashMap<String, bool>,
    outputs: Vec<String>,
}

pub struct Broadcaster {
    outputs: Vec<String>,
}

impl FlipFlop {
    pub fn with_details(name: &str, outputs: Vec<String>) -> Self {
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
    pub fn with_details(name: &str, outputs: Vec<String>, inputs: Vec<String>) -> Self {
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
    pub fn with_outputs(outputs: Vec<String>) -> Self {
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
