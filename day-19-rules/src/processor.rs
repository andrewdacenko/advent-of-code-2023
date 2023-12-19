use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug)]
struct Rule {
    prop: u8,
    comparator: u8,
    value: usize,
    terminal: String,
}

#[derive(Debug)]
struct Flow {
    rules: Vec<Rule>,
    terminal: String,
}

#[derive(Clone, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn total(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

pub fn sum_parts(contents: &str) -> usize {
    let (flows_str, parts_str) = contents.split_once("\n\n").unwrap();
    let flows = parse_flows(flows_str);
    parse_parts(parts_str)
        .iter()
        .map(|part| process_part(part, &flows).and_then(|part| Some(part.total())))
        .flatten()
        .sum()
}

fn parse_flows(list: &str) -> HashMap<String, Flow> {
    list.lines()
        .map(|line| {
            let (name, rest) = line.split_once("{").unwrap();
            let [rules @ .., termial] =
                &rest.split(",").collect::<Vec<&str>>()[..] else { panic!("Can't parse rules") };
            return (
                name.to_string(),
                Flow {
                    rules: rules
                        .iter()
                        .map(|rule| {
                            let (start, terminal) = rule.split_once(":").unwrap();
                            return Rule {
                                prop: start.as_bytes()[0],
                                comparator: start.as_bytes()[1],
                                value: start[2..].parse().expect("Should parse num from rule"),
                                terminal: terminal.to_string(),
                            };
                        })
                        .collect(),
                    terminal: termial[0..termial.len() - 1].to_string(),
                },
            );
        })
        .collect()
}

fn parse_parts(list: &str) -> Vec<Part> {
    list.lines()
        .map(|line| {
            let [x, m, a, s] = NUM_RE
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect::<Vec<usize>>()[..] else {
                    panic!("Can't parse line")
                };
            return Part { x, m, a, s };
        })
        .collect()
}

fn process_part<'a>(part: &'a Part, flows: &'a HashMap<String, Flow>) -> Option<&'a Part> {
    let mut name = "in";
    while let Some(flow) = flows.get(name) {
        name = process_flow(part, flow);
    }
    match name {
        "A" => Some(part),
        _ => None,
    }
}

fn process_flow<'a>(part: &'a Part, flow: &'a Flow) -> &'a String {
    let mut stack = flow.rules.iter();
    while let Some(rule) = stack.next() {
        let value = match rule.prop {
            b'x' => part.x,
            b'm' => part.m,
            b'a' => part.a,
            b's' => part.s,
            _ => panic!("Missing prop {}", rule.prop.to_string()),
        };
        let terminal = match rule.comparator {
            b'<' => {
                if value.lt(&rule.value) {
                    Some(&rule.terminal)
                } else {
                    None
                }
            }
            b'>' => {
                if value.gt(&rule.value) {
                    Some(&rule.terminal)
                } else {
                    None
                }
            }
            _ => panic!("Unknown comparator {}", rule.comparator.to_string()),
        };
        if let Some(next) = terminal {
            return next;
        }
    }

    &flow.terminal
}
