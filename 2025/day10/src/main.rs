#[derive(Debug)]
struct Button(u64);

impl Button {
    fn parse(data: &str) -> Self {
        Self(
            data.trim_matches(['(', ')'])
                .split(',')
                .fold(0, |result, idx| {
                    let idx: usize = idx.parse().unwrap();
                    let mask = 1 << idx;
                    mask | result
                }),
        )
    }
}

#[derive(Debug)]
struct Leds(u64);

impl Leds {
    fn parse(data: &str) -> Self {
        let data = data.trim_matches(['[', ']']);
        Self(data.chars().enumerate().fold(0, |result, (idx, v)| {
            if v == '#' {
                result | (1 << idx)
            } else {
                result
            }
        }))
    }
}

#[derive(Debug)]
struct Machine {
    leds: Leds,
    buttons: Vec<Button>,
}

fn min_presses_impl(m: &Machine, cur: u64, btns: &[Button]) -> Option<usize> {
    btns.iter()
        .enumerate()
        .filter_map(|(i, btn)| {
            let new = cur ^ btn.0;
            if new == m.leds.0 {
                return Some(1);
            }
            let rem = &btns[i + 1..];
            if rem.is_empty() {
                return None;
            }

            min_presses_impl(m, new, rem).map(|v| v + 1)
        })
        .min()
}

fn min_presses(m: &Machine) -> usize {
    min_presses_impl(m, 0, &m.buttons).unwrap()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data: Vec<Machine> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let leds = Leds::parse(parts.next().unwrap());
            let mut buttons = vec![];

            for part in parts {
                if part.starts_with("{") {
                    break;
                }

                let btn = Button::parse(part);
                buttons.push(btn);
            }
            Machine { leds, buttons }
        })
        .collect();

    let result: usize = data
        .iter()
        .map(|m| {
            let min = min_presses(m);
            println!("Machine: {m:?}");
            println!("min presses: {min}",);
            min
        })
        .sum();
    println!("result: {result}");
}
