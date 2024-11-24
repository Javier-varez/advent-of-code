// Map is transposed in column major order
fn transpose(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = map.len();
    let cols = map[0].len();
    let mut new = vec![vec![' '; rows]; cols];

    for (r, row) in map.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            new[c][r] = *val;
        }
    }
    new
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

fn tilt(map: &[Vec<char>], dir: Dir) -> Vec<Vec<char>> {
    let tilt_rows = |map: &[Vec<char>], reverse: bool| {
        let mut new = map.to_owned();

        for row in &mut new {
            let mut num_balls = 0;
            let copied_row = row.clone();
            let iter = if reverse {
                Box::new(copied_row.iter().enumerate().rev())
                    as Box<dyn Iterator<Item = (usize, &char)>>
            } else {
                Box::new(copied_row.iter().enumerate()) as Box<dyn Iterator<Item = (usize, &char)>>
            };
            for (idx, c) in iter {
                match c {
                    'O' => {
                        num_balls += 1;
                        row[idx] = '.';
                    }
                    '#' => {
                        row[idx] = '#';
                        for i in 0..num_balls {
                            let ball_idx = if reverse { idx + 1 + i } else { idx - 1 - i };
                            row[ball_idx] = 'O';
                        }
                        num_balls = 0;
                    }
                    '.' => {}
                    c => panic!("Unexpected character {c}"),
                };
            }

            for i in 0..num_balls {
                let ball_idx = if reverse { i } else { row.len() - 1 - i };
                row[ball_idx] = 'O';
            }
        }

        new
    };

    let (t, r) = match dir {
        Dir::North => (true, true),
        Dir::South => (true, false),
        Dir::West => (false, true),
        Dir::East => (false, false),
    };

    if t {
        transpose(&tilt_rows(&transpose(&map), r))
    } else {
        tilt_rows(&map, r)
    }
}

enum Solution {
    Unique(Vec<Vec<char>>),
    Cycle(usize),
}

fn cycle_until_stable(mut map: Vec<Vec<char>>, steps: usize) -> Vec<Vec<char>> {
    let run = |map: &[Vec<char>], memory: &mut Vec<Vec<Vec<char>>>| {
        let mut new = tilt(map, Dir::North);
        new = tilt(&new, Dir::West);
        new = tilt(&new, Dir::South);
        new = tilt(&new, Dir::East);
        if let Some((i, _)) = memory.iter().enumerate().find(|(_, m)| **m == new) {
            return Solution::Cycle(i);
        }

        memory.push(new.clone());
        Solution::Unique(new)
    };

    let mut memory: Vec<Vec<Vec<char>>> = vec![];

    let mut i = 0;
    let (start, cycle) = loop {
        map = match run(&map, &mut memory) {
            Solution::Unique(s) => s,
            Solution::Cycle(old_idx) => {
                break (old_idx, i - old_idx);
            }
        };
        i += 1;
    };

    let remaining = steps - i - 1;
    let idx = remaining % cycle;
    memory[start + idx].clone()
}

fn weight(map: &[Vec<char>]) -> usize {
    let mut value = 0;
    let len = map.len();
    for (i, row) in map.iter().enumerate() {
        let count = row.iter().filter(|c| **c == 'O').count();
        value += (len - i) * count;
    }
    value
}

fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let map = cycle_until_stable(map, 1000_000_000);

    println!("weight: {}", weight(&map));
}
