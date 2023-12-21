use core::panic;
use std::time::Instant;
use std::{cell::RefCell, collections::HashMap};
fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    H,
    L,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NodeT {
    Broadcaster,
    FlipFlop,
    And,
    Unknown,
}

#[derive(Debug)]
struct Node {
    name: String,
    node_type: NodeT,
    outputs: Vec<String>,
    inputs: Vec<String>,
    current_pulse: Vec<(Pulse, String, Instant)>,
    memory: HashMap<String, Pulse>,
}

impl Node {
    fn new(name: String, node_type: NodeT, outputs: Vec<String>) -> Self {
        let mut memory = HashMap::new();
        match node_type {
            NodeT::FlipFlop => {
                memory.insert("prev_pulse".to_string(), Pulse::L);
            }
            NodeT::And => {
                for neighbor in &outputs {
                    memory.insert(neighbor.to_string(), Pulse::L);
                }
            }
            _ => {}
        }
        Self {
            name,
            node_type,
            outputs,
            inputs: Vec::new(),
            memory,
            current_pulse: Vec::new(),
        }
    }
    fn send_pulse(&mut self, pulse: Pulse, from: String) {
        self.current_pulse.push((pulse, from, Instant::now()));
    }
    fn handle_pulse(&mut self, graph: &HashMap<String, RefCell<Node>>) -> (usize, usize) {
        if self.current_pulse.is_empty() {
            return (0, 0);
        }
        let (pulse, from, _time) = self.current_pulse.remove(0);
        let (mut t_h, mut t_l) = (0, 0);
        match self.node_type {
            NodeT::Broadcaster => {
                for output in &self.outputs {
                    graph
                        .get(output)
                        .unwrap()
                        .borrow_mut()
                        .send_pulse(pulse.clone(), self.name.clone());

                    if pulse == Pulse::H {
                        t_h += 1;
                    } else {
                        t_l += 1;
                    }
                }
            }
            NodeT::FlipFlop => {
                let prev_pulse = self.memory.get("prev_pulse").unwrap();
                let mut to_send = None;
                match pulse {
                    Pulse::H => {}
                    Pulse::L => {
                        if prev_pulse == &Pulse::L {
                            to_send = Some(Pulse::H);
                        } else {
                            to_send = Some(Pulse::L);
                        }
                        //change prev_pulse to opposite
                        self.memory.insert(
                            "prev_pulse".to_string(),
                            if prev_pulse == &Pulse::L {
                                Pulse::H
                            } else {
                                Pulse::L
                            },
                        );
                    }
                }
                if let Some(pulse) = to_send {
                    for output in &self.outputs {
                        graph
                            .get(output)
                            .unwrap()
                            .borrow_mut()
                            .send_pulse(pulse.clone(), self.name.clone());
                        if pulse == Pulse::H {
                            t_h += 1;
                        } else {
                            t_l += 1;
                        }
                    }
                }
            }
            NodeT::And => {
                self.memory.insert(from.to_string(), pulse);
                let mut all_high = true;
                for input in &self.inputs {
                    let entry = self.memory.entry(input.to_string()).or_insert(Pulse::L);
                    all_high = all_high && entry == &Pulse::H;
                }
                let to_send = if all_high { Pulse::L } else { Pulse::H };
                for output in &self.outputs {
                    graph
                        .get(output)
                        .unwrap()
                        .borrow_mut()
                        .send_pulse(to_send.clone(), self.name.clone());
                    if to_send == Pulse::H {
                        t_h += 1;
                    } else {
                        t_l += 1;
                    }
                }
            }
            _ => {
                // clear current_pulse
                self.current_pulse.clear();
            }
        }

        (t_h, t_l)
    }
}

fn solution(input: &str) -> usize {
    let mut graph: HashMap<String, RefCell<Node>> = HashMap::new();
    for line in input.lines() {
        let (node_info, neighbors) = line.split_once(" -> ").unwrap();
        let neighbors: Vec<String> = neighbors.split(", ").map(|s| s.to_string()).collect();
        let (node_type, name) = match node_info.chars().nth(0) {
            Some('&') => (NodeT::And, node_info.get(1..).unwrap()),
            Some('%') => (NodeT::FlipFlop, node_info.get(1..).unwrap()),
            Some('b') => (NodeT::Broadcaster, node_info),
            _ => panic!("Wrong Char"),
        };

        let node = Node::new(name.to_string(), node_type, neighbors);

        graph.insert(name.to_string(), node.into());
    }
    let mut nodes_to_insert:Vec<Node> = Vec::new();
    for node in graph.values() {
        for output in &node.borrow().outputs {
            match graph.get(output) {
                Some(n) => {
                    n.borrow_mut().inputs.push(node.borrow().name.clone());
                }
                None => {
                    //create a node  of type unknown
                    let unknown_node = Node::new(output.to_string(), NodeT::Unknown, Vec::new());
                    nodes_to_insert.push(unknown_node);
                }
            }
        }
    }
    for node in nodes_to_insert {
        graph.insert(node.name.clone(), node.into());
    }

    let (mut t_h, mut t_l) = (0, 0);

    for _i in 0..1000 {
        graph
            .get("broadcaster")
            .unwrap()
            .borrow_mut()
            .send_pulse(Pulse::L, "in".to_string());
        t_l += 1;

        loop {
            let mut all_zero = true;
            // get values ordered by current pulse timestamp
            let mut graph_list = graph.values().collect::<Vec<_>>();
            graph_list.retain(|&node| node.borrow().current_pulse.len() > 0);
            graph_list.sort_by_key(|&node| match node.borrow().current_pulse.get(0) {
                Some((_, _, time)) => time.elapsed(),
                None => std::time::Duration::from_secs(0),
            });
            graph_list.reverse();
            //remove all nodes with current_pulse of length 0

            for node in graph_list {
                let (h, l) = node.borrow_mut().handle_pulse(&graph);
                t_h += h;
                t_l += l;
                all_zero = all_zero && h == 0 && l == 0;
            }
            if all_zero {
                break;
            }
        }
    }

    t_l * t_h
}
