use std::collections::HashSet;

fn simulate_and_get_splits(
    map: &[Vec<char>],
    (row, col): (isize, isize),
    splits: &mut HashSet<(isize, isize)>,
) {
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    if row < 0 || row >= height {
        return;
    }

    if col < 0 || col >= width {
        return;
    }

    if splits.contains(&(row, col)) {
        return;
    }

    if map[row as usize][col as usize] == '^' {
        splits.insert((row, col));

        simulate_and_get_splits(map, (row, col - 1), splits);
        simulate_and_get_splits(map, (row, col + 1), splits);
    } else {
        simulate_and_get_splits(map, (row + 1, col), splits)
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data: Vec<Vec<char>> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let start = data
        .iter()
        .enumerate()
        .find_map(|(row, data)| {
            data.iter()
                .enumerate()
                .find_map(|(col, elem)| (*elem == 'S').then_some((row as isize, col as isize)))
        })
        .unwrap();

    let mut splits = HashSet::new();
    simulate_and_get_splits(&data, start, &mut splits);

    println!("splits = {}", splits.len());
}
