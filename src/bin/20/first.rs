use crate::pulse::{Broadcaster, Conjunction, FlipFlop, Module, Pulse};
use std::collections::{HashMap, VecDeque};

pub fn run(input: &str) {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut module_inputs: HashMap<String, Vec<String>> = HashMap::new();
    let mut conjunctions: HashMap<&str, Vec<String>> = HashMap::new();
    for declaration in input.lines() {
        let name = &declaration.split_whitespace().next().unwrap()[1..];
        let outputs = declaration
            .split(" -> ")
            .last()
            .unwrap()
            .split(", ")
            .map(String::from)
            .collect::<Vec<String>>();
        match declaration.chars().next().unwrap() {
            '%' => {
                for output in &outputs {
                    if module_inputs.contains_key(output) {
                        module_inputs
                            .get_mut(output)
                            .unwrap()
                            .push(String::from(name));
                    } else {
                        module_inputs.insert(output.to_string(), vec![String::from(name)]);
                    }
                }
                modules.insert(
                    String::from(name),
                    Box::new(FlipFlop::with_details(name, outputs.clone())),
                );
            }
            '&' => {
                conjunctions.insert(name, outputs);
            }
            'b' => {
                for output in &outputs {
                    if module_inputs.contains_key(output) {
                        module_inputs
                            .get_mut(output)
                            .unwrap()
                            .push(String::from("broadcaster"));
                    } else {
                        module_inputs.insert(output.to_string(), vec![String::from("broadcaster")]);
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
    for (name, outputs) in conjunctions {
        modules.insert(
            String::from(name),
            Box::new(Conjunction::with_details(
                name,
                outputs,
                module_inputs.get(name).unwrap_or(&vec![]).clone(),
            )),
        );
    }

    let mut high_count = 0;
    let mut low_count = 0;
    for _button_press in 0..1000 {
        let mut pulses = VecDeque::from([Pulse {
            is_high: false,
            from: String::from("button"),
            to: String::from("broadcaster"),
        }]);
        while let Some(current_pulse) = pulses.pop_front() {
            match current_pulse.is_high {
                true => high_count += 1,
                false => low_count += 1,
            }
            // dbg!(&current_pulse);
            if let Some(receiver) = modules.get_mut(&current_pulse.to) {
                for output in receiver.process_pulse(current_pulse) {
                    pulses.push_back(output);
                }
            }
        }
    }
    let count = high_count * low_count;
    println!("{}", count);
}
