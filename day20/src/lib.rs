use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace1},
    combinator::opt,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use num::integer::lcm;
use petgraph::graphmap::DiGraphMap;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, Hash)]
enum Module {
    Button,
    Broadcaster,
    FlipFlop(bool),
    Conjunction,
    Rx,
}

struct Config<'a> {
    graph: DiGraphMap<&'a str, bool>,
    module_type_map: HashMap<&'a str, Module>,
}

fn parse_module(text: &str) -> IResult<&str, (&str, Module, Vec<&str>)> {
    let (text, (module_type, name)) =
        tuple((alt((tag("broadcaster"), tag("%"), tag("&"))), opt(alpha1)))(text)?;
    let (text, _) = tag(" -> ")(text)?;
    let (text, outputs) = separated_list1(tag(", "), alpha1)(text)?;
    let module = match module_type {
        "broadcaster" => Module::Broadcaster,
        "%" => Module::FlipFlop(false),
        "&" => Module::Conjunction,
        _ => panic!("Failed to parse module type"),
    };
    Ok((
        text,
        (
            match name {
                Some(name) => name,
                None => "broadcaster",
            },
            module,
            outputs,
        ),
    ))
}

fn parse_input(text: &str) -> IResult<&str, Config> {
    let (text, modules) = separated_list1(multispace1, parse_module)(text)?;
    let mut config = DiGraphMap::new();
    let mut module_type_map = HashMap::new();

    for (node, module_type, outputs) in modules {
        config.add_node(node);
        module_type_map.insert(node, module_type);
        for out_node in outputs {
            if !config.contains_node(out_node) {
                config.add_node(out_node);
            }
            config.add_edge(node, out_node, false);
        }
    }

    // Add the implicit button
    config.add_node("button");
    config.add_edge("button", "broadcaster", false);
    module_type_map.insert("button", Module::Button);

    // Add new rx module
    module_type_map.insert("rx", Module::Rx);

    Ok((
        text,
        Config {
            graph: config,
            module_type_map,
        },
    ))
}

pub fn part1(text: String) -> u64 {
    let (_, parsed_config) = parse_input(text.as_str()).unwrap();
    let mut config = parsed_config.graph;
    let mut module_type_map = parsed_config.module_type_map;
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(("button", "broadcaster", false));
        while let Some((from_node, to_node, pulse)) = pulses.pop_front() {
            // Count the pulse
            if pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }

            // Update last pulse
            let edge_weight_ref = config
                .edge_weight_mut(from_node, to_node)
                .unwrap_or_else(|| {
                    panic!("Could not find edge between {} and {}", from_node, to_node,)
                });
            *edge_weight_ref = pulse;

            // Figure out the outgoing pulse
            let out_pulse = match module_type_map.get_mut(to_node) {
                Some(Module::Button) => Some(false),
                Some(Module::Broadcaster) => Some(pulse),
                Some(Module::FlipFlop(state)) => {
                    if pulse {
                        None
                    } else {
                        *state = !*state;
                        Some(*state)
                    }
                }
                Some(Module::Conjunction) => Some(
                    !config
                        .edges_directed(to_node, petgraph::Direction::Incoming)
                        .all(|(_, _, in_edge)| *in_edge),
                ),
                Some(Module::Rx) | None => {
                    continue;
                }
            };

            // Send the outgoing pulse
            if let Some(out_pulse) = out_pulse {
                config
                    .neighbors_directed(to_node, petgraph::Direction::Outgoing)
                    .for_each(|out| {
                        pulses.push_back((to_node, out, out_pulse));
                    });
            }
        }
    }
    low_pulses * high_pulses
}

// On inspection, the input graph consists of 4 large cycles.
// The idea is that we can find a cycle length for each of these cycles
// and then take the least common multiple to get the final answer.
//
// The cycle lengths that we need are the number of button presses it takes
// for each input to the conjunction module "ls" to be high.
//
// Interesting aside, each cycle ends in a conjunction module which sends
// low when all of its inputs (every other flip flop in its cycle) are high.
// This low pulse then gets inverted by a 1 input conjunction module so
// a high pulse gets send to "ls".
pub fn part2(text: String) -> u64 {
    let (_, parsed_config) = parse_input(text.as_str()).unwrap();
    let mut config = parsed_config.graph;
    let mut module_type_map = parsed_config.module_type_map;
    let mut num_presses = 0;
    let rx_inputs: Vec<&str> = config
        .neighbors_directed("rx", petgraph::Direction::Incoming)
        .collect();
    assert!(rx_inputs.len() == 1);
    let last_conjunction = rx_inputs[0];
    assert!(matches!(
        module_type_map.get(last_conjunction).unwrap(),
        Module::Conjunction
    ));
    let num_last_conj_inputs = config
        .edges_directed(last_conjunction, petgraph::Direction::Incoming)
        .count();
    let mut last_conj_inputs = HashMap::new();
    let mut last_conj_cycle_lengths = HashMap::new();

    while last_conj_cycle_lengths.len() < num_last_conj_inputs {
        let mut pulses = VecDeque::new();
        pulses.push_back(("button", "broadcaster", false));
        num_presses += 1;
        while let Some((from_node, to_node, pulse)) = pulses.pop_front() {
            // Update last pulse
            let edge_weight_ref = config
                .edge_weight_mut(from_node, to_node)
                .unwrap_or_else(|| {
                    panic!("Could not find edge between {} and {}", from_node, to_node,)
                });
            *edge_weight_ref = pulse;

            // Figure out the outgoing pulse
            let out_pulse = match module_type_map.get_mut(to_node) {
                Some(Module::Button) => Some(false),
                Some(Module::Broadcaster) => Some(pulse),
                Some(Module::FlipFlop(state)) => {
                    if pulse {
                        None
                    } else {
                        *state = !*state;
                        Some(*state)
                    }
                }
                Some(Module::Conjunction) => {
                    if to_node == last_conjunction && pulse {
                        let last_conj_input = last_conj_inputs.get(from_node);
                        if let Some(previous_num_presses) = last_conj_input {
                            last_conj_cycle_lengths
                                .insert(from_node, num_presses - previous_num_presses);
                        } else {
                            last_conj_inputs.insert(from_node, num_presses);
                        }
                    }
                    Some(
                        !config
                            .edges_directed(to_node, petgraph::Direction::Incoming)
                            .all(|(_, _, in_edge)| *in_edge),
                    )
                }
                Some(Module::Rx) => {
                    if pulse {
                        continue;
                    } else {
                        return num_presses;
                    }
                }
                None => {
                    continue;
                }
            };

            // Send the outgoing pulse
            if let Some(out_pulse) = out_pulse {
                config
                    .neighbors_directed(to_node, petgraph::Direction::Outgoing)
                    .for_each(|out| {
                        pulses.push_back((to_node, out, out_pulse));
                    });
            }
        }
    }
    last_conj_cycle_lengths
        .into_iter()
        .fold(1, |acc, (_, cycle_len)| lcm(acc, cycle_len))
}
