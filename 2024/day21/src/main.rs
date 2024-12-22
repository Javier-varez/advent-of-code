#![allow(dead_code)]

use std::collections::BinaryHeap;
use std::collections::HashMap;

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

#[derive(Clone, Copy, Eq, PartialEq)]
enum Keypad {
    Numeric(Location),
    Directional(Location),
}

const NUMERIC_A: Location = Location(3, 2);
const DIRECTIONAL_A: Location = Location(0, 2);

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

fn compute_path(map: &'static HashMap<char, Location>, mut cur_loc: Location, seq: &str) -> String {
    let mut moves = String::new();
    for c in seq.chars() {
        let next_loc = map.get(&c).unwrap();
        let dr = next_loc.0 - cur_loc.0;
        let dc = next_loc.1 - cur_loc.1;
        let abs_dr = if dr < 0 { -dr } else { dr };
        let abs_dc = if dc < 0 { -dc } else { dc };
        for _ in 0..abs_dr {
            moves.push(if dr < 0 { '^' } else { 'v' });
        }
        for _ in 0..abs_dc {
            moves.push(if dc < 0 { '<' } else { '>' });
        }
        moves.push('A');
        cur_loc = *next_loc;
    }
    moves
}

fn move_keypads(keypads: &mut [Keypad], seq: &str) -> usize {
    let keypad = keypads.first().unwrap();
    let sub_seq = match keypad {
        Keypad::Numeric(cur_loc) => compute_path(&NUMERIC_KEYS, *cur_loc, seq),
        Keypad::Directional(cur_loc) => compute_path(&DIR_KEYS, *cur_loc, seq),
    };

    println!("sub seq {sub_seq}");

    if keypads.len() > 1 {
        move_keypads(&mut keypads[1..], &sub_seq)
    } else {
        sub_seq.len()
    }
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

#[derive(Clone)]
struct KeypadState {
    cost: usize,
    cur_loc: Location,
    next_loc: Option<Location>,
    seq: String,
    ty: KeypadType,
    next: Option<(String, BinaryHeap<KeypadState>)>,
}

impl PartialEq for KeypadState {
    fn eq(&self, other: &Self) -> bool {
        return self.cost == other.cost
            && self.cur_loc == other.cur_loc
            && self.next_loc == other.next_loc
            && self.ty == other.ty;
    }
}

impl Eq for KeypadState {}

impl PartialOrd for KeypadState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(<Self as Ord>::cmp(&self, other))
    }
}

impl Ord for KeypadState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.seq.cmp(&other.seq) {
            std::cmp::Ordering::Equal => other.cost.cmp(&self.cost),
            o => o,
        }
    }
}

impl KeypadState {
    fn new(keypads: &[KeypadType]) -> Self {
        let first_ty = *keypads.first().unwrap();

        let next = if keypads.len() > 1 {
            let mut heap = BinaryHeap::new();
            heap.push(Self::new(&keypads[1..]));
            Some((String::new(), heap))
        } else {
            None
        };

        Self {
            cost: 0,
            cur_loc: if first_ty == KeypadType::Numeric {
                NUMERIC_A
            } else {
                DIRECTIONAL_A
            },
            next_loc: None,
            seq: String::new(),
            ty: first_ty,
            next,
        }
    }
}

fn is_valid_loc(loc: Location, ty: KeypadType) -> bool {
    match ty {
        KeypadType::Numeric => loc.0 <= 3 && loc.1 <= 2 && loc != Location(3, 0),
        KeypadType::Directional => loc.0 <= 1 && loc.1 <= 2 && loc != Location(0, 0),
    }
}

fn move_keypads_inner(idx: usize, pq: &mut BinaryHeap<KeypadState>, seq: &str) -> usize {
    while let Some(mut state) = pq.pop() {
        // for _ in 0..idx {
        //     print!("  ");
        // }
        // println!(
        //     "{idx} Controlling keypad at {:?}, next {:?}",
        //     state.cur_loc, state.next_loc
        // );

        if let Some(next_loc) = state.next_loc {
            let delta = next_loc - state.cur_loc;
            if delta == Location(0, 0) {
                state.next_loc = None;
                if let Some((next_seq, next_keyp)) = &mut state.next {
                    let c = 'A';
                    // for _ in 0..idx {
                    //     print!("  ");
                    // }
                    // println!("{idx} keypad wants press '{c}'",);
                    next_seq.push(c);
                    let inner_cost = move_keypads_inner(idx + 1, next_keyp, next_seq);
                    state.cost = inner_cost;
                } else {
                    state.cost = state.seq.len();
                };
                pq.push(state);
                continue;
            }

            // Try moving on the row axis first
            if delta.0 != 0 {
                let next_loc = state.cur_loc + Location(if delta.0 > 0 { 1 } else { -1 }, 0);
                if is_valid_loc(next_loc, state.ty) {
                    let mut state = state.clone();
                    state.cur_loc = next_loc;
                    // Update next keypads and cost
                    if let Some((next_seq, next_keyp)) = &mut state.next {
                        let c = if delta.0 > 0 { 'v' } else { '^' };
                        // for _ in 0..idx {
                        //     print!("  ");
                        // }
                        // println!("{idx} keypad wants to move on rows: Push '{c}'",);
                        next_seq.push(c);
                        let inner_cost = move_keypads_inner(idx + 1, next_keyp, next_seq);
                        state.cost = inner_cost;
                    } else {
                        state.cost = state.seq.len();
                    };
                    pq.push(state);
                }
            }

            // Try moving on the column axis now
            if delta.1 != 0 {
                let next_loc = state.cur_loc + Location(0, if delta.1 > 0 { 1 } else { -1 });
                if is_valid_loc(next_loc, state.ty) {
                    let mut state = state.clone();
                    state.cur_loc = next_loc;
                    // Update next keypads and cost
                    if let Some((next_seq, next_keyp)) = &mut state.next {
                        let c = if delta.1 > 0 { '>' } else { '<' };
                        // for _ in 0..idx {
                        //     print!("  ");
                        // }
                        // println!("{idx} keypad wants to move on columns: Push '{c}'",);
                        next_seq.push(c);
                        let inner_cost = move_keypads_inner(idx + 1, next_keyp, next_seq);
                        state.cost = inner_cost;
                    } else {
                        state.cost = state.seq.len();
                    };
                    pq.push(state);
                }
            }
        } else {
            if state.seq.len() >= seq.len() {
                let cost = state.cost;
                if idx == 0 {
                    println!("0: {}", &state.seq);
                    let mut nq = &state.next;
                    let mut idx = 1;
                    while let Some((s, ip)) = &nq {
                        println!("{idx}: {}", s);
                        idx += 1;
                        nq = &ip.peek().unwrap().next;
                    }
                }
                pq.push(state);
                return cost;
            }

            let cur_char = seq.chars().skip(state.seq.len()).next().unwrap();
            let next_loc = location(cur_char, state.ty);
            state.next_loc = Some(next_loc);
            state.seq.push(cur_char);

            // for _ in 0..idx {
            //     print!("  ");
            // }
            // println!("{idx} next char '{cur_char}', next_loc '{next_loc:?}'",);
            pq.push(state);
        }
    }

    unreachable!()
}
fn move_keypads2(keypads: &[KeypadType], seq: &str) -> usize {
    let mut pq: BinaryHeap<KeypadState> = BinaryHeap::new();

    pq.push(KeypadState::new(keypads));
    move_keypads_inner(0, &mut pq, seq)
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    _ = args.next();
    let input_file = args
        .next()
        .context("Provide an input file as an argument")?;
    let data = std::fs::read(&input_file)?;
    let data = std::str::from_utf8(&data)?;

    // let mut keypads = vec![
    //     Keypad::Numeric(NUMERIC_A),
    //     Keypad::Directional(DIRECTIONAL_A),
    //     Keypad::Directional(DIRECTIONAL_A),
    // ];

    let mut keypads = vec![
        KeypadType::Numeric,
        KeypadType::Directional,
        KeypadType::Directional,
        KeypadType::Directional,
    ];

    for seq in data.lines() {
        println!("handling seq {seq}");
        let moves = move_keypads2(&mut keypads, seq);
        println!("Optimum move for {seq} is: {moves}");
    }

    Ok(())
}
