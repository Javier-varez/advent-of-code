use std::collections::HashSet;

const NUM_STEPS: usize = 26501365;

struct Map(Vec<Vec<char>>);

impl Map {
    fn get_coord(&self, (r, c): (usize, usize)) -> char {
        if r >= self.0.len() || c >= self.0[0].len() {
            return '#';
        }
        self.0[r][c]
    }

    fn max(&self) -> (usize, usize) {
        (self.0.len() - 1, self.0[0].len() - 1)
    }

    fn start(&self) -> (usize, usize) {
        self.0
            .iter()
            .enumerate()
            .find_map(|(r, content)| {
                content.iter().enumerate().find_map(
                    |(c, v)| {
                        if *v == 'S' {
                            Some((r, c))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap()
    }
}

fn count_n_iters(map: &Map, steps: usize, start: (usize, usize)) -> usize {
    let mut positions = HashSet::new();
    positions.insert(start);

    for _i in 0..steps {
        let mut step_results = HashSet::new();

        for (r, c) in &positions {
            let coord_is_valid = |c| map.get_coord(c) == '#';

            if !coord_is_valid((*r - 1, *c)) {
                step_results.insert((*r - 1, *c));
            }
            if !coord_is_valid((*r + 1, *c)) {
                step_results.insert((*r + 1, *c));
            }
            if !coord_is_valid((*r, *c - 1)) {
                step_results.insert((*r, *c - 1));
            }
            if !coord_is_valid((*r, *c + 1)) {
                step_results.insert((*r, *c + 1));
            }
        }
        positions = step_results;
    }

    return positions.iter().count();
}

fn main() {
    let map = Map(std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect());

    let size = map.0.len();

    let inner_a = NUM_STEPS / size;
    let inner_b = NUM_STEPS / size - 1;
    let odd = if inner_a % 2 == 1 { inner_a } else { inner_b };
    let even = if inner_a % 2 == 0 { inner_a } else { inner_b };

    let odd_blocks = odd * odd;
    let even_blocks = even * even;
    dbg!(odd_blocks);
    dbg!(even_blocks);

    let start = map.start();
    let odd = count_n_iters(&map, size * 2 + 1, start);
    let even = count_n_iters(&map, size * 2, start);
    dbg!(odd);
    dbg!(even);

    let (sr, sc) = start;
    let (mr, mc) = map.max();

    let corners: usize = [(sr, 0), (sr, mc), (0, sc), (mr, sc)]
        .iter()
        .map(|start| count_n_iters(&map, size - 1, *start))
        .sum();
    dbg!(corners);

    let num_large_blocks = NUM_STEPS / size - 1;
    let large: usize = [(0, 0), (0, mc), (mr, 0), (mr, mc)]
        .iter()
        .map(|start| count_n_iters(&map, size * 3 / 2 - 1, *start) * num_large_blocks)
        .sum();
    dbg!(num_large_blocks);

    let num_small_blocks = NUM_STEPS / size;
    let small: usize = [(0, 0), (0, mc), (mr, 0), (mr, mc)]
        .iter()
        .map(|start| count_n_iters(&map, size / 2 - 1, *start) * num_small_blocks)
        .sum();
    dbg!(num_small_blocks);

    let total = odd_blocks * odd + even_blocks * even + corners + large + small;
    dbg!(total);
}
