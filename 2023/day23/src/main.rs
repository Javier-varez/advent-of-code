use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Map(Vec<Vec<char>>);

impl Map {
    fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    fn start(&self) -> (usize, usize) {
        for (i, c) in self.0[0].iter().enumerate() {
            if *c == '.' {
                return (0, i);
            }
        }
        unreachable!();
    }

    fn end(&self) -> (usize, usize) {
        for (i, c) in self.0[self.0.len() - 1].iter().enumerate() {
            if *c == '.' {
                return (self.0.len() - 1, i);
            }
        }
        unreachable!();
    }

    fn branches<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.0
            .iter()
            .enumerate()
            .map(move |(r, vec)| {
                vec.iter().enumerate().filter_map(move |(c, tile)| {
                    let count = self.next_positions((r, c)).count();
                    if *tile != '#' && count > 2 {
                        Some((r, c))
                    } else {
                        None
                    }
                })
            })
            .flatten()
    }

    fn next_positions<'a>(
        &'a self,
        (r, c): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let (nr, nc) = self.size();
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(move |(dr, dc)| {
                if dr < 0 && r == 0 {
                    return None;
                }
                if dc < 0 && c == 0 {
                    return None;
                }
                if dr > 0 && r == nr - 1 {
                    return None;
                }
                if dc > 0 && c == nc - 1 {
                    return None;
                }
                let r = (r as isize + dr) as usize;
                let c = (c as isize + dc) as usize;
                let tile = self.0[r][c];
                match tile {
                    '.' => Some((r, c)),
                    '^' => Some((r, c)),
                    'v' => Some((r, c)),
                    '<' => Some((r, c)),
                    '>' => Some((r, c)),
                    _ => None,
                }
            })
    }
}

fn main() {
    let map = Map(std::io::stdin()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            l.chars().collect_vec()
        })
        .collect_vec());

    println!("branches {:?}", map.branches().collect_vec());
    println!("branches {:?}", map.branches().count());

    let mut cost_map = HashMap::new();
    let start = map.start();
    let end = map.end();

    cost_map.insert(start, HashMap::new());
    cost_map.insert(end, HashMap::new());
    for branch in map.branches() {
        cost_map.insert(branch, HashMap::new());
    }

    let keys = cost_map.iter().map(|(k, _)| *k).collect_vec();
    for node in keys {
        for way in map.next_positions(node) {
            let mut visited = HashSet::new();
            visited.insert(node);
            visited.insert(way);
            let mut cur = way;
            loop {
                let next = map
                    .next_positions(cur)
                    .filter(|n| !visited.contains(n))
                    .collect_vec();
                assert_eq!(next.len(), 1);

                cur = next[0];
                visited.insert(cur);
                if cost_map.contains_key(&cur) {
                    let cost = visited.iter().count() - 1;
                    cost_map.get_mut(&node).unwrap().insert(cur, cost);
                    cost_map.get_mut(&cur).unwrap().insert(node, cost);
                    break;
                }
            }
        }
    }

    let mut solutions = vec![];

    let mut stack = Vec::new();
    stack.push((HashSet::new(), start, 0));

    while let Some((mut visited, node, cost)) = stack.pop() {
        visited.insert(node);

        if node == end {
            solutions.push(cost);
        }

        for (next, added_cost) in cost_map[&node]
            .iter()
            .filter(|(n, _)| !visited.contains(*n))
        {
            stack.push((visited.clone(), *next, cost + added_cost));
        }
    }
    println!("Max distance {}", solutions.iter().max().unwrap());
}
