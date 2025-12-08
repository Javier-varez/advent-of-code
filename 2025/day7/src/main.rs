use std::collections::HashMap;

fn simulate_and_get_splits(
    map: &[Vec<char>],
    (row, col): (isize, isize),
    solutions: &mut HashMap<(isize, isize), usize>,
) -> usize {
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    if row >= height {
        return 1;
    }

    if col < 0 || col >= width {
        return 0;
    }

    if let Some(sol) = solutions.get(&(row, col)) {
        return *sol;
    }

    let result = if map[row as usize][col as usize] == '^' {
        let left = simulate_and_get_splits(map, (row, col - 1), solutions);
        let right = simulate_and_get_splits(map, (row, col + 1), solutions);
        left + right
    } else {
        simulate_and_get_splits(map, (row + 1, col), solutions)
    };

    solutions.insert((row, col), result);
    result
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

    let mut solutions = HashMap::new();
    let timelines = simulate_and_get_splits(&data, start, &mut solutions);

    println!("timelines = {timelines}");
}
