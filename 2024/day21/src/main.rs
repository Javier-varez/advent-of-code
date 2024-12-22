use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::Context;
use lazy_static::lazy_static;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
struct Location(isize, isize);

impl std::ops::Sub for Location {
    type Output = Location;

    fn sub(self, rhs: Self) -> Self::Output {
        Location(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add for Location {
    type Output = Location;

    fn add(self, rhs: Self) -> Self::Output {
        Location(self.0 + rhs.0, self.1 + rhs.1)
    }
}

lazy_static! {
    static ref NUMERIC_KEYS: HashMap<char, Location> = {
        let mut map = HashMap::new();
        map.insert('7', Location(0, 0));
        map.insert('8', Location(0, 1));
        map.insert('9', Location(0, 2));
        map.insert('4', Location(1, 0));
        map.insert('5', Location(1, 1));
        map.insert('6', Location(1, 2));
        map.insert('1', Location(2, 0));
        map.insert('2', Location(2, 1));
        map.insert('3', Location(2, 2));
        map.insert('0', Location(3, 1));
        map.insert('A', Location(3, 2));
        map
    };
    static ref DIR_KEYS: HashMap<char, Location> = {
        let mut map = HashMap::new();
        map.insert('^', Location(0, 1));
        map.insert('A', Location(0, 2));
        map.insert('<', Location(1, 0));
        map.insert('v', Location(1, 1));
        map.insert('>', Location(1, 2));
        map
    };
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum KeypadType {
    Numeric,
    Directional,
}

fn location(c: char, t: KeypadType) -> Location {
    match t {
        KeypadType::Numeric => *NUMERIC_KEYS.get(&c).expect("Unknown key"),
        KeypadType::Directional => *DIR_KEYS.get(&c).expect("Unknown key"),
    }
}

fn is_valid_loc(loc: Location, ty: KeypadType) -> bool {
    match ty {
        KeypadType::Numeric => loc.0 <= 3 && loc.1 <= 2 && loc != Location(3, 0),
        KeypadType::Directional => loc.0 <= 1 && loc.1 <= 2 && loc != Location(0, 0),
    }
}

type Seq = String;

lazy_static! {
// Map from (sequence, num_keypads) => cost
static ref MEMORY: Mutex<HashMap<(Seq, usize), usize>> = Mutex::new(HashMap::new());
}

fn compute_sequences(src_loc: Location, dst_loc: Location, ty: KeypadType) -> Vec<Seq> {
    let mut sequences = vec![];

    fn inner(
        sequences: &mut Vec<Seq>,
        seq: &mut String,
        cur_loc: Location,
        dst_loc: Location,
        ty: KeypadType,
    ) {
        if cur_loc == dst_loc {
            let mut new_str = seq.clone();
            new_str.push('A');
            sequences.push(new_str);
            return;
        }

        let delta = dst_loc - cur_loc;

        if delta.0 != 0 {
            let next_loc = cur_loc + Location(if delta.0 > 0 { 1 } else { -1 }, 0);
            if is_valid_loc(next_loc, ty) {
                seq.push(if delta.0 > 0 { 'v' } else { '^' });
                inner(sequences, seq, next_loc, dst_loc, ty);
                seq.pop();
            }
        }

        if delta.1 != 0 {
            let next_loc = cur_loc + Location(0, if delta.1 > 0 { 1 } else { -1 });
            if is_valid_loc(next_loc, ty) {
                seq.push(if delta.1 > 0 { '>' } else { '<' });
                inner(sequences, seq, next_loc, dst_loc, ty);
                seq.pop();
            }
        }
    }

    let mut s = String::new();
    inner(&mut sequences, &mut s, src_loc, dst_loc, ty);
    sequences
}

fn solve_optimal(seq: &str, keypads: &[KeypadType]) -> usize {
    if keypads.len() == 0 {
        return seq.len();
    }

    {
        let memory = MEMORY.lock().unwrap();
        if let Some(sol) = memory.get(&(seq.to_owned(), keypads.len())) {
            return *sol;
        }
    }

    let cur_keypad = *keypads.first().unwrap();
    let mut total_cost = 0;

    let mut cur_loc = location('A', cur_keypad);
    for c in seq.chars() {
        // Compute inner sequences for each character
        let next_loc = location(c, cur_keypad);
        let seqs = compute_sequences(cur_loc, next_loc, cur_keypad);
        let cost = seqs
            .iter()
            .map(|seq| solve_optimal(seq, &keypads[1..]))
            .min()
            .expect("No paths found!");
        total_cost += cost;
        cur_loc = next_loc;
    }

    {
        let mut memory = MEMORY.lock().unwrap();
        memory.insert((seq.to_owned(), keypads.len()), total_cost);
    }

    total_cost
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    _ = args.next();
    let input_file = args
        .next()
        .context("Provide an input file as an argument")?;
    let data = std::fs::read(&input_file)?;
    let data = std::str::from_utf8(&data)?;

    let mut keypads = vec![
        KeypadType::Numeric,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
    ];

    let mut total = 0;
    for seq in data.lines() {
        println!("handling seq {seq}");
        let moves = solve_optimal(seq, &mut keypads);
        println!("Optimum move for {seq} is: {moves}");
        let seq: usize = seq[..3].parse()?;
        total += moves * seq;
    }
    println!("total = {total}");

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_compute_sequences() {
        let seqs = compute_sequences(Location(3, 2), Location(1, 1), KeypadType::Numeric);
        assert_eq!(seqs, ["^^<A", "^<^A", "<^^A"]);

        let seqs = compute_sequences(Location(2, 0), Location(3, 1), KeypadType::Numeric);
        assert_eq!(seqs, [">vA"]);

        let seqs = compute_sequences(Location(0, 2), Location(1, 0), KeypadType::Directional);
        assert_eq!(seqs, ["v<<A", "<v<A"]);
    }
}
