use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum State {
    NoGroup,
    Group,
}

#[derive(Hash, Eq, PartialEq)]
struct Key {
    state: State,
    damaged: Vec<usize>,
    s: String,
}

fn count_arrangements(
    s: &str,
    state: State,
    damaged: &mut [usize],
    cache: &mut HashMap<Key, usize>,
) -> usize {
    if (damaged.len() == 1 && damaged[0] == 0) || damaged.len() == 0 {
        if s.chars().any(|c| c == '#') {
            // no failures left, but there are failures in the remainder of the str
            return 0;
        } else {
            return 1;
        }
    }

    let k = Key {
        s: s.to_owned(),
        state,
        damaged: damaged.to_owned(),
    };

    if let Some(r) = cache.get(&k) {
        return *r;
    }

    let solution = match (state, s.split_at(1)) {
        (_, ("?", _)) => {
            let with_fail: String = s
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { '#' } else { c })
                .collect();
            let without_fail: String = s
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { '.' } else { c })
                .collect();
            let fail = count_arrangements(&with_fail, state, damaged, cache);
            let no_fail = count_arrangements(&without_fail, state, damaged, cache);
            fail + no_fail
        }
        (_, ("#", "")) => {
            if damaged.len() == 1 && damaged[0] == 1 {
                1
            } else {
                0
            }
        }
        (State::Group, (".", "")) => {
            if damaged.len() == 1 && damaged[0] == 0 {
                1
            } else {
                0
            }
        }
        (State::NoGroup, (".", "")) => {
            if damaged.len() == 0 {
                1
            } else {
                0
            }
        }
        (_, ("#", rest)) => {
            if damaged.len() == 0 || damaged[0] == 0 {
                // Not possible
                0
            } else {
                damaged[0] -= 1;
                let res = count_arrangements(rest, State::Group, damaged, cache);
                damaged[0] += 1;
                res
            }
        }
        (State::NoGroup, (".", rest)) => count_arrangements(rest, State::NoGroup, damaged, cache),
        (State::Group, (".", rest)) => {
            assert!(damaged.len() > 0);
            if damaged[0] != 0 {
                // Not possible, failure
                0
            } else {
                count_arrangements(rest, State::NoGroup, &mut damaged[1..], cache)
            }
        }
        p => {
            panic!("Unexpected pattern {p:?}");
        }
    };
    cache.insert(k, solution);
    solution
}

fn main() -> Result<()> {
    let mut count = 0;
    let mut cache = HashMap::new();
    for line in std::io::stdin().lines() {
        let line = line?;
        let mut iter = line.split_whitespace();
        let pattern: String =
            std::iter::repeat(iter.next().unwrap().chars().chain(std::iter::once('?')))
                .take(5)
                .flatten()
                .collect();
        let pattern = &pattern[..pattern.len() - 1];
        let seq: Vec<usize> =
            std::iter::repeat(iter.next().unwrap().split(",").map(|v| v.parse().unwrap()))
                .take(5)
                .flatten()
                .collect();

        let mut cloned_seq = seq.clone();
        let arr = count_arrangements(&pattern, State::NoGroup, &mut cloned_seq, &mut cache);
        assert_eq!(seq, cloned_seq);
        println!("Pattern {pattern:?}, errs {seq:?}, arrangements {arr:?}");
        count += arr;
    }
    println!("Cound of possibilities {count}");
    Ok(())
}
