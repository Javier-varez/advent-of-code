use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

trait Module {
    fn update_input(&mut self, source: &str);
    fn execute(&mut self, source: &str, pulse: Pulse) -> Option<Pulse>;
    fn reset(&mut self);
    fn ty(&self) -> ModType;
}

#[derive(Debug)]
struct FlipFlop {
    state: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self { state: false }
    }
}

impl Module for FlipFlop {
    fn update_input(&mut self, _source: &str) {}

    fn execute(&mut self, _source: &str, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::Low => {
                self.state = !self.state;
                Some(if self.state { Pulse::High } else { Pulse::Low })
            }
            Pulse::High => None,
        }
    }

    fn reset(&mut self) {
        self.state = false;
    }

    fn ty(&self) -> ModType {
        ModType::FlipFlop
    }
}

#[derive(Debug)]
struct Conjunction {
    sources: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }
}

impl Module for Conjunction {
    fn update_input(&mut self, source: &str) {
        self.sources.insert(source.to_string(), Pulse::Low);
    }

    fn execute(&mut self, source: &str, pulse: Pulse) -> Option<Pulse> {
        let state = self
            .sources
            .get_mut(source)
            .expect("Input to the conjunction exists!");
        *state = pulse;
        if self.sources.iter().all(|(_s, p)| *p == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn reset(&mut self) {
        for (_, pulse) in self.sources.iter_mut() {
            *pulse = Pulse::Low
        }
    }

    fn ty(&self) -> ModType {
        ModType::Conjunction
    }
}

#[derive(Debug)]
struct Broadcast {}

impl Broadcast {
    fn new() -> Self {
        Self {}
    }
}

impl Module for Broadcast {
    fn update_input(&mut self, _source: &str) {}

    fn execute(&mut self, _source: &str, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }

    fn reset(&mut self) {}

    fn ty(&self) -> ModType {
        ModType::Broadcast
    }
}

struct ModuleRouting {
    // map from module name to actual module and names of connected modules
    modules: HashMap<String, (Box<dyn Module>, Vec<String>)>,
}

impl ModuleRouting {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    fn insert(&mut self, name: String, module: Box<dyn Module>, connections: Vec<String>) {
        self.modules.insert(name, (module, connections));
    }

    fn update_inputs(&mut self) {
        let keys: Vec<String> = self.modules.keys().cloned().collect();
        for source in keys {
            let destinations = self.modules[&source].1.clone();
            for destination in destinations {
                if let Some((module, _outs)) = self.modules.get_mut(&destination) {
                    module.update_input(&source);
                }
            }
        }
    }

    fn reset(&mut self) {
        for (_, (module, _)) in self.modules.iter_mut() {
            module.reset();
        }
    }
}

impl std::fmt::Debug for ModuleRouting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Modules {{\n")?;
        for module in &self.modules {
            write!(f, "\t{} => {:?}\n", module.0, module.1 .1)?;
        }
        write!(f, "}}\n")
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }

    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b != 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap()).collect();

    let mut modules = ModuleRouting::new();
    for line in &lines {
        let mut iter = line.split("->");
        let module = iter.next().unwrap().trim();
        let (name, module): (String, Box<dyn Module>) = if module.starts_with("%") {
            (
                module.trim_start_matches("%").to_string(),
                Box::new(FlipFlop::new()),
            )
        } else if module.starts_with("&") {
            (
                module.trim_start_matches("&").to_string(),
                Box::new(Conjunction::new()),
            )
        } else if module == "broadcaster" {
            (module.to_string(), Box::new(Broadcast::new()))
        } else {
            unimplemented!();
        };
        let connections: Vec<String> = iter
            .next()
            .unwrap()
            .trim()
            .split(",")
            .map(|l| l.trim().to_string())
            .collect();

        modules.insert(name, module, connections);
    }
    modules.update_inputs();
    println!("{modules:?}");

    // Single button press

    let mut deque = VecDeque::new();
    let mut high = 0usize;
    let mut low = 0usize;

    for _i in 0..1000 {
        deque.push_back((Pulse::Low, "button".to_owned(), "broadcaster".to_owned()));

        while let Some((pulse, source, dest)) = deque.pop_front() {
            match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            };

            if let Some((module, outputs)) = modules.modules.get_mut(&dest) {
                if let Some(out_pulse) = module.execute(&source, pulse) {
                    for out in &*outputs {
                        deque.push_back((out_pulse, dest.clone(), out.to_string()));
                    }
                }
            }
        }
    }
    println!("low = {low}, high = {high}, result = {}", low * high);
    modules.reset();

    let sources_to_rx: Vec<String> = modules
        .modules
        .iter()
        .filter_map(|(name, (module, outs))| {
            if outs.iter().find(|n| *n == "rx").is_none() {
                return None;
            }
            assert_eq!(module.ty(), ModType::Conjunction);
            Some(name.clone())
        })
        .collect();

    // A single conjunction module ouptuts rx. It's inputs must all be high such that the signal low is sent to rx
    assert_eq!(sources_to_rx.len(), 1);
    let conj = &sources_to_rx[0];

    let sources_to_conj: Vec<String> = modules
        .modules
        .iter()
        .filter_map(|(name, (_module, outs))| {
            if outs.iter().find(|n| *n == conj).is_none() {
                return None;
            }
            Some(name.clone())
        })
        .collect();

    println!("sources to conjunction : {sources_to_conj:?}");

    let mut high_after = HashMap::new();

    'outer: for i in std::iter::successors(Some(1usize), |n| Some(n + 1)) {
        deque.push_back((Pulse::Low, "button".to_owned(), "broadcaster".to_owned()));

        while let Some((pulse, source, dest)) = deque.pop_front() {
            match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            };

            if dest == *conj && pulse == Pulse::High {
                if !high_after.contains_key(&source) {
                    high_after.insert(source.clone(), i);
                    println!("{source} ouptuts high after {i:?}");
                }
                if high_after.len() == sources_to_conj.len() {
                    println!("Collected all required information");
                    break 'outer;
                }
            }

            if let Some((module, outputs)) = modules.modules.get_mut(&dest) {
                if let Some(out_pulse) = module.execute(&source, pulse) {
                    for out in &*outputs {
                        deque.push_back((out_pulse, dest.clone(), out.to_string()));
                    }
                }
            }
        }
    }

    let mut result = 1;
    for (_k, v) in high_after {
        result = lcm(result, v);
    }
    println!("result {result}");
}
