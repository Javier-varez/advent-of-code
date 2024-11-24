use anyhow::{anyhow, bail};
use std::collections::HashMap;

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

fn main() -> anyhow::Result<()> {
    let mut line_iter = std::io::stdin().lines();

    let instructions = line_iter
        .next()
        .ok_or(anyhow!("Expected instructions line"))
        .and_then(|l| Ok(l?))?;

    let _empty_line = line_iter
        .next()
        .ok_or(anyhow!("Expected at least two lines"))
        .and_then(|l| Ok(l?))
        .and_then(|l| {
            if l.is_empty() {
                Ok(l)
            } else {
                bail!("Line is not empty");
            }
        })?;

    let map: HashMap<String, (String, String)> = line_iter
        .map(|l| l.map_err(|e| anyhow::Error::from(e)))
        .fold(Ok(HashMap::new()), |m, l| match m {
            Ok(mut m) => l.and_then(|l| {
                let (parent, leafs) = {
                    let mut iter = l.split("=");
                    (
                        iter.next()
                            .ok_or(anyhow!("Expected parent node separated by ="))?
                            .trim(),
                        iter.next()
                            .ok_or(anyhow!("Expected leaf nodes separated by ="))?
                            .trim(),
                    )
                };

                let (left, right) = {
                    let mut iter = leafs
                        .trim_start_matches("(")
                        .trim_end_matches(")")
                        .split(",");
                    (
                        iter.next().ok_or(anyhow!("Expected left leaf"))?.trim(),
                        iter.next().ok_or(anyhow!("Expected right leaf"))?.trim(),
                    )
                };
                m.insert(parent.to_string(), (left.to_string(), right.to_string()));
                Ok(m)
            }),
            e => e,
        })?;

    let current_nodes: Vec<_> = map
        .iter()
        .filter_map(|(k, _v)| {
            if k.ends_with("A") {
                Some(k.as_str())
            } else {
                None
            }
        })
        .collect();
    println!("Initial nodes: {current_nodes:?}");

    let cycles: Vec<_> = current_nodes
        .iter()
        .map(|n| {
            let mut cur = *n;
            let mut instruction_iter = instructions.chars().cycle();
            let mut count = 0;
            while !cur.ends_with("Z") {
                let inst = instruction_iter.next().expect("Should never end");
                let next = match inst {
                    'L' => &map[cur].0,
                    'R' => &map[cur].1,
                    inst => {
                        panic!("Unexpected instruction: {inst:?}");
                    }
                };
                cur = next;
                count += 1;
            }
            let first = count;
            count = 0;

            while count == 0 || !cur.ends_with("Z") {
                let inst = instruction_iter.next().expect("Should never end");
                let next = match inst {
                    'L' => &map[cur].0,
                    'R' => &map[cur].1,
                    inst => {
                        panic!("Unexpected instruction: {inst:?}");
                    }
                };
                cur = next;
                count += 1;
            }
            (first, count)
        })
        .collect();

    assert!(cycles.iter().all(|(a, b)| a == b));
    assert!(cycles.iter().all(|(a, _)| a % instructions.len() == 0));

    let steps = cycles.iter().fold(1, |v, (a, _)| lcm(v, *a));

    println!("Took {steps} steps");

    Ok(())
}
