#[derive(Debug)]
struct Map(Vec<Vec<char>>);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Energy {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct EnergyMap(Vec<Vec<Vec<Energy>>>);

impl EnergyMap {
    fn new(rows: usize, cols: usize) -> Self {
        Self(vec![vec![vec![]; cols]; rows])
    }

    fn insert(&mut self, (row, col, dir): (usize, usize, Energy)) -> bool {
        if self.0[row][col].iter().find(|d| **d == dir).is_some() {
            // Already taken, no need to revisit
            return true;
        }
        self.0[row][col].push(dir);
        false
    }
}

fn apply_energy((row, col): (isize, isize), dir: Energy) -> (isize, isize) {
    match dir {
        Energy::Up => (row - 1, col),
        Energy::Down => (row + 1, col),
        Energy::Left => (row, col - 1),
        Energy::Right => (row, col + 1),
    }
}

fn next_energy_options(pipe: char, energy: Energy) -> Vec<Energy> {
    match pipe {
        '.' => vec![energy],
        '/' => vec![match energy {
            Energy::Up => Energy::Right,
            Energy::Down => Energy::Left,
            Energy::Right => Energy::Up,
            Energy::Left => Energy::Down,
        }],
        '\\' => vec![match energy {
            Energy::Up => Energy::Left,
            Energy::Down => Energy::Right,
            Energy::Right => Energy::Down,
            Energy::Left => Energy::Up,
        }],
        '|' => match energy {
            Energy::Up => vec![Energy::Up],
            Energy::Down => vec![Energy::Down],
            Energy::Right | Energy::Left => vec![Energy::Up, Energy::Down],
        },
        '-' => match energy {
            Energy::Left => vec![Energy::Left],
            Energy::Right => vec![Energy::Right],
            Energy::Up | Energy::Down => vec![Energy::Left, Energy::Right],
        },
        _ => unimplemented!(),
    }
}

fn next_states((row, col, dir): (usize, usize, Energy), pipe: char) -> Vec<(isize, isize, Energy)> {
    let row = row as isize;
    let col = col as isize;
    next_energy_options(pipe, dir)
        .iter()
        .map(|energy| {
            let (row, col) = apply_energy((row, col), *energy);
            (row, col, *energy)
        })
        .collect()
}

impl Map {
    fn get_pipe(&self, row: usize, col: usize) -> char {
        self.0[row][col]
    }

    fn validate_state(
        &self,
        (row, col, dir): (isize, isize, Energy),
    ) -> Option<(usize, usize, Energy)> {
        let rows = self.0.len() as isize;
        let cols = self.0[0].len() as isize;
        if row < 0 || row >= rows || col < 0 || col >= cols {
            return None;
        }
        Some((row as usize, col as usize, dir))
    }

    fn traverse_map(&self, initial_state: (usize, usize, Energy)) -> EnergyMap {
        fn inner(map: &Map, energy: &mut EnergyMap, state: (usize, usize, Energy)) {
            if energy.insert(state) {
                // This path and direction has already been explored
                return;
            }

            let (r, c, _dir) = state;
            let states = next_states(state, map.get_pipe(r, c));
            for s in states {
                if let Some(s) = map.validate_state(s) {
                    // State is valid
                    inner(map, energy, s);
                }
            }
        }

        let mut energy = EnergyMap::new(self.0.len(), self.0[0].len());
        inner(&self, &mut energy, initial_state);
        energy
    }
}

fn main() {
    let map = Map(std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect());

    let rows = map.0.len();
    let cols = map.0[0].len();

    let initial_states = (0..rows)
        .map(|r| (r, 0, Energy::Right))
        .chain((0..rows).map(|r| (r, cols - 1, Energy::Left)))
        .chain((0..cols).map(|c| (0, c, Energy::Down)))
        .chain((0..cols).map(|c| (rows - 1, c, Energy::Up)));

    let energy_maps = initial_states.map(|initial_state| map.traverse_map(initial_state));

    let energized = energy_maps
        .map(|energy_map| {
            energy_map
                .0
                .iter()
                .map(|r| r.iter().filter(|v| !v.is_empty()).count())
                .sum::<usize>()
        })
        .max()
        .unwrap();

    println!("Count energized positions: {energized}");
}
