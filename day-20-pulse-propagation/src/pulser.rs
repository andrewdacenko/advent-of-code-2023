use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Signal {
    from: String,
    to: String,
    pulse: Pulse,
}

impl Signal {
    fn new(from: &str, to: &str, pulse: &Pulse) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            pulse: pulse.to_owned(),
        }
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    is_on: bool,
    module_type: ModuleType,
    next: Vec<String>,
    from: Vec<String>,
    previous_pulses: HashMap<String, Pulse>,
}

impl Module {
    fn parse(line: &str) -> (String, Self) {
        let (name_str, next_str) = line.split_once(" -> ").unwrap();
        let next = next_str.split(", ").map(|s| s.to_string()).collect();
        let (name, module_type) = match name_str {
            "broadcaster" => ("broadcaster".to_string(), ModuleType::Broadcaster),
            name => match &name[0..1] {
                "%" => (name[1..].to_string(), ModuleType::FlipFlop),
                "&" => (name[1..].to_string(), ModuleType::Conjunction),
                x => panic!("Unknown prefix {}", x),
            },
        };
        (
            name.clone(),
            Module {
                name,
                is_on: false,
                module_type,
                next,
                from: vec![],
                previous_pulses: HashMap::new(),
            },
        )
    }

    fn initiate_previous_pulses(&mut self, mapping: &HashMap<String, Vec<String>>) {
        let previous_pulses: HashMap<String, Pulse> = mapping
            .iter()
            .filter(|(_name, next)| next.contains(&self.name))
            .map(|(name, _next)| (name.to_owned(), Pulse::Low))
            .collect();
        self.from = previous_pulses.clone().into_keys().collect();
        if self.module_type.ne(&ModuleType::Conjunction) {
            return;
        }
        self.previous_pulses = previous_pulses;
    }

    fn propagate(&mut self, signal: &Signal) -> Option<Pulse> {
        match self.module_type {
            ModuleType::Broadcaster => Some(Pulse::Low),
            ModuleType::FlipFlop => {
                if signal.pulse.eq(&Pulse::High) {
                    return None;
                }

                self.is_on = !self.is_on;

                if self.is_on {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
            ModuleType::Conjunction => {
                self.previous_pulses
                    .insert(signal.from.to_owned(), signal.pulse.to_owned());
                if self
                    .previous_pulses
                    .iter()
                    .all(|(_name, pulse)| pulse.eq(&Pulse::High))
                {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }

    fn is_default(&self) -> bool {
        match self.module_type {
            ModuleType::Broadcaster => true,
            ModuleType::FlipFlop => self.is_on.eq(&false),
            ModuleType::Conjunction => self
                .previous_pulses
                .iter()
                .all(|(_name, pulse)| pulse.eq(&Pulse::Low)),
        }
    }
}

const BUTTON: &str = "button";
const BROADCASTER: &str = "broadcaster";

type Modules = HashMap<String, Module>;
type Broadcasts = Vec<(u128, u128)>;

pub fn count_pulses(input: &str, cycles: u128) -> u128 {
    let mut modules = parse_modules(input);
    let mut broadcasts: Broadcasts = vec![];
    for _i in 0..cycles {
        broadcasts.push(broadcast(&mut modules));
        if modules.iter().all(|(_, module)| module.is_default()) {
            break;
        }
    }

    let loop_size = broadcasts.len() as u128;
    let loops = cycles / loop_size;
    let rest = (cycles % loop_size) as usize;

    let (low, high): (Vec<_>, Vec<_>) = broadcasts.iter().cloned().unzip();
    let (rest_low, rest_high): (Vec<_>, Vec<_>) = broadcasts.iter().take(rest).cloned().unzip();

    return low.iter().sum::<u128>() * high.iter().sum::<u128>() * loops.pow(2)
        + rest_low.iter().sum::<u128>() * rest_high.iter().sum::<u128>();
}

pub fn count_pulses_till_machine_starts(input: &str) -> usize {
    let mut modules = parse_modules(input);
    let mut targets: HashMap<String, usize> = modules
        .get("vf")
        .unwrap()
        .from
        .iter()
        .map(|name| (name.to_owned(), 0))
        .collect();
    let mut cycles: usize = 0;
    loop {
        cycles += 1;

        let mut queue = VecDeque::from([Signal {
            from: BUTTON.to_string(),
            to: BROADCASTER.to_string(),
            pulse: Pulse::Low,
        }]);
        while let Some(signal) = queue.pop_front() {
            if targets.contains_key(&signal.to) && signal.pulse.eq(&Pulse::Low) {
                if let Some(target) = targets.get_mut(&signal.to) {
                    *target = cycles;
                }
                match targets.iter().fold(1, |acc, (_name, cycle)| acc * cycle) {
                    0 => {}
                    total => return total,
                }
            }

            let Some(module) = modules.get_mut(&signal.to) else {continue;};
            let Some(next_pulse) = module.propagate(&signal) else { continue };
            let mut next_queue: VecDeque<Signal> = module
                .next
                .iter()
                .map(|next| Signal::new(&module.name, &next, &next_pulse))
                .collect();
            queue.append(&mut next_queue);
        }
    }
}

fn parse_modules(input: &str) -> Modules {
    let mut modules: Modules = input.split("\n").map(Module::parse).collect();
    let modules_mapping = modules
        .iter()
        .map(|(name, module)| (name.to_owned(), module.next.clone()))
        .collect();
    for (_name, module) in modules.iter_mut() {
        module.initiate_previous_pulses(&modules_mapping);
    }
    modules
}

fn broadcast(modules: &mut Modules) -> (u128, u128) {
    let mut queue = VecDeque::from([Signal {
        from: BUTTON.to_string(),
        to: BROADCASTER.to_string(),
        pulse: Pulse::Low,
    }]);
    let (mut low, mut high) = (0, 0);
    while let Some(signal) = queue.pop_front() {
        if signal.pulse.eq(&Pulse::Low) {
            low += 1;
        } else {
            high += 1;
        }
        let Some(module) = modules.get_mut(&signal.to) else {continue;};
        let Some(next_pulse) = module.propagate(&signal) else { continue };
        let mut next_queue: VecDeque<Signal> = module
            .next
            .iter()
            .map(|next| Signal::new(&module.name, &next, &next_pulse))
            .collect();
        queue.append(&mut next_queue);
    }

    return (low, high);
}
