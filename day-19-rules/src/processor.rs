use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Default, Debug)]
struct PartLimitations {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartLimitations {
    fn default() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn is_valid(&self) -> bool {
        self.x.0 < self.x.1 || self.m.0 < self.m.1 || self.a.0 < self.a.1 || self.s.0 < self.s.1
    }

    fn total(&self) -> u128 {
        (self.x.1 - self.x.0 + 1) as u128
            * (self.m.1 - self.m.0 + 1) as u128
            * (self.a.1 - self.a.0 + 1) as u128
            * (self.s.1 - self.s.0 + 1) as u128
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

pub fn sum_ranges(contents: &str) -> u128 {
    let (flows_str, _) = contents.split_once("\n\n").unwrap();
    return all_ranges(&parse_flows(flows_str))
        .iter()
        .map(|item| item.total())
        .sum();
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

fn all_ranges(flows: &HashMap<String, Flow>) -> Vec<PartLimitations> {
    let mut res = vec![];
    let mut stack = vec![("in".to_string(), PartLimitations::default())];
    while let Some((name, limitation)) = stack.pop() {
        if let Some(flow) = flows.get(&name) {
            for item in process_flow_rule(&limitation, flow) {
                stack.push((item.0, item.1));
            }
        }
        if name.eq("A") {
            res.push(limitation);
        }
    }

    return res;
}

fn process_flow_rule(limitation: &PartLimitations, flow: &Flow) -> Vec<(String, PartLimitations)> {
    let mut res = vec![];

    let mut prev_limitations = limitation.clone();
    for rule in flow.rules.iter() {
        match rule.terminal.as_str() {
            "R" => {
                apply_rule(&mut prev_limitations, rule);
                if !prev_limitations.is_valid() {
                    return res;
                }
            }
            next => {
                let mut current_limitation = prev_limitations.clone();
                apply_terminating_rule(&mut current_limitation, rule);
                if current_limitation.is_valid() {
                    res.push((next.to_string(), current_limitation));
                }
                apply_rule(&mut prev_limitations, rule);
                if !prev_limitations.is_valid() {
                    return res;
                }
            }
        }
    }

    match flow.terminal.as_str() {
        "R" => {}
        next => res.push((next.to_string(), prev_limitations)),
    }

    return res;
}

fn apply_rule(limitations: &mut PartLimitations, rule: &Rule) {
    let mut limit = match rule.prop {
        b'x' => &mut limitations.x,
        b'm' => &mut limitations.m,
        b'a' => &mut limitations.a,
        b's' => &mut limitations.s,
        _ => panic!("Unknown prop"),
    };
    match rule.comparator {
        b'<' => limit.0 = std::cmp::max(limit.0, rule.value),
        b'>' => limit.1 = std::cmp::min(limit.1, rule.value),
        _ => panic!("Unknown comparator"),
    }
}

fn apply_terminating_rule(limitations: &mut PartLimitations, rule: &Rule) {
    let mut limit = match rule.prop {
        b'x' => &mut limitations.x,
        b'm' => &mut limitations.m,
        b'a' => &mut limitations.a,
        b's' => &mut limitations.s,
        _ => panic!("Unknown prop"),
    };
    match rule.comparator {
        b'<' => limit.1 = std::cmp::min(limit.1, rule.value - 1),
        b'>' => limit.0 = std::cmp::max(limit.0, rule.value + 1),
        _ => panic!("Unknown comparator"),
    }
}
