use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::str;
use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 20;
    type ParsedInput = HashMap<ID, Module>;
    type Output = u64;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        let mut network = HashMap::new();
        for line in Self::read_file(dir) {
            let (identifier, targets) = line.split_once(" -> ").unwrap();
            let bytes_id = identifier.as_bytes();
            let prefix = bytes_id[0];
            let module_type = ModuleType::from_prefix(prefix);
            let id_bytes = match module_type {
                ModuleType::Broadcaster => bytes_id,
                _ => &bytes_id[1..],
            };
            let id = gen_id(id_bytes);
            let connections = targets.split(", ").map(|x| gen_id(x.as_bytes())).collect();
            network.insert(id, Module{id, module_type, connections});

            for (id, module) in network.clone().iter() {
                // Initial memories for conjunctions need to know their input modules too
                for connection in module.connections.iter() {
                    if let Some(m) = &mut network.get_mut(connection) {
                        match m.module_type {
                            ModuleType::Conjunction(ref mut memory) => { memory.insert(*id, false); },
                            _ => (),
                        }
                    }
                }
            }

        }
        network
    }

    fn part1(dir: &str) -> Self::Output {
        let mut network = Self::parse_input(dir);
        let (low, high) = push_button_n(&mut network, 1000);
        low * high
    }

    fn part2(dir: &str) -> Self::Output {
        0
    }
}

const FLIP_FLOP: u8 = b'%';
const CONJUNCTION: u8 = b'&';
const BROADCASTER: &[u8] = b"broadcaster";


type ID = [u8;2];

fn gen_id(str_id: &[u8]) -> ID {
    if str_id == BROADCASTER {
        return [0, 0];
    }
    let mut id = [0, 0];
    id[0] = str_id[0];
    id[1] = *str_id.get(1).unwrap_or(&0);
    id
}

fn un_id(id: &ID) -> &str {
    if *id == [0, 0] {return str::from_utf8(BROADCASTER).unwrap();}
    str::from_utf8(&id[0..]).unwrap()
}

#[derive(Clone)]
pub enum ModuleType{
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<ID, bool>),
}

impl ModuleType {
    fn from_prefix(prefix: u8) -> Self {
        match prefix {
            FLIP_FLOP => Self::FlipFlop(false),
            CONJUNCTION => Self::Conjunction(HashMap::new()),
            _ => Self::Broadcaster,
        }
    }
}

#[derive(Clone)]
pub struct Module {
    id: ID,
    module_type: ModuleType,
    connections: Vec<ID>,
}

impl Module {
    fn receive_signal(&mut self, signal: Signal) -> Vec<Signal> {
        let Signal{from, to: _, signal_type} = signal;
        match &mut self.module_type {
            ModuleType::Broadcaster => self.send_to_connections(signal_type),
            ModuleType::FlipFlop(state) => {
                if signal_type {Vec::new()}
                else {
                    let new_state = !*state;
                    self.module_type = ModuleType::FlipFlop(new_state);
                    self.send_to_connections(new_state)
                }
            },
            ModuleType::Conjunction(ref mut memory) => {
                memory.insert(from, signal_type);
                let signal_type = !memory.values().all(|x| *x);
                self.send_to_connections(signal_type)
            },

        }
    }

    fn send_to_connections(&self, signal_type: bool) -> Vec<Signal> {
        self.connections.iter().map(|x| Signal{from: self.id, to: *x, signal_type}).collect()
    }
}

struct Signal {
    from: ID,
    to: ID,
    signal_type: bool,
}

impl Debug for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -{}-> {}", un_id(&self.from), self.signal_type, un_id(&self.to)))
    }
}

fn push_button(network: &mut HashMap<ID, Module>) -> (u64, u64) {
    let mut total_low = 0;
    let mut total_high = 0;
    let mut queue = VecDeque::new();
    let broadcaster_id = gen_id(BROADCASTER);
    queue.push_back(Signal{from: broadcaster_id, to: broadcaster_id, signal_type: false});
    while !queue.is_empty() {
        let signal = queue.pop_front().unwrap();
        dbg!(&signal);
        if signal.signal_type {total_high += 1;} else {total_low += 1;}
        if let Some(target_module) = network.get_mut(&signal.to) {
            let new_signals = target_module.receive_signal(signal);
            for s in new_signals {
                queue.push_back(s);
            }
        }
    }
    (total_low, total_high)
}

fn push_button_n(network: &mut HashMap<ID, Module>, number_of_times: u32) -> (u64, u64) {
    // should probably add a cache here
    let mut total_low = 0;
    let mut total_high = 0;
    for _ in 0..number_of_times {
        let(low, high) = push_button(network);
        total_low += low;
        total_high += high;
    }
    (total_low, total_high)
}
