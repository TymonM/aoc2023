use crate::pulse::{Broadcaster, Conjunction, FlipFlop, Module, Pulse};
use std::collections::{HashMap, VecDeque};

const OUTPUT_MODULE: &str = "rx";

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

    let mut repeats = vec![];
    for daughter_pulse in modules
        .get_mut("broadcaster")
        .unwrap()
        .process_pulse(Pulse {
            is_high: false,
            from: String::from("button"),
            to: String::from("broadcaster"),
        })
    {
        let mut repeat = 0;
        'repeat: loop {
            repeat += 1;
            let mut pulses = VecDeque::from([daughter_pulse.clone()]);
            while let Some(current_pulse) = pulses.pop_front() {
                // dbg!(&current_pulse);
                if !current_pulse.is_high && current_pulse.to == OUTPUT_MODULE {
                    break 'repeat;
                }
                if let Some(receiver) = modules.get_mut(&current_pulse.to) {
                    for output in receiver.process_pulse(current_pulse) {
                        pulses.push_back(output);
                    }
                }
            }
        }
        repeats.push(repeat);
    }
    let button_presses = lcm(&repeats);
    println!("{}", button_presses);
}

fn lcm(vals: &[usize]) -> usize {
    let mut least = 1;
    for val in vals {
        least = num::integer::lcm(least, *val);
    }

    least
}
