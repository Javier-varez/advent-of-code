fn count_adjacent(data: &[Vec<char>], x: usize, y: usize, width: usize, height: usize) -> usize {
    let result = (-1..=1).fold(0, |acc, dy| {
        let y = y as isize + dy;
        acc + (if y < 0 || y >= height as isize {
            0
        } else {
            (-1..=1)
                .filter(|dx| *dx != 0 || dy != 0)
                .fold(0, |acc, dx| {
                    let x = x as isize + dx;
                    acc + (if x < 0 || x >= width as isize {
                        0
                    } else if data[y as usize][x as usize] == '@' {
                        1
                    } else {
                        0
                    })
                })
        })
    });
    result as usize
}

fn get_removals(data: &[Vec<char>]) -> Vec<(usize, usize)> {
    let height = data.len();
    let width = data.first().unwrap().len();

    (0..height)
        .flat_map(|y| {
            (0..width)
                .filter(move |x| {
                    let x = *x;
                    if data[y][x] != '@' {
                        false
                    } else {
                        let adj = count_adjacent(data, x, y, width, height);
                        adj < 4
                    }
                })
                .map(move |x| (x, y))
        })
        .collect()
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let mut data: Vec<Vec<char>> = std::fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut total = 0;
    loop {
        let removals = get_removals(&data);
        if removals.is_empty() {
            break;
        }

        total += removals.len();
        removals.into_iter().for_each(|(x, y)| {
            data[y][x] = '.';
        })
    }

    println!("total {total}");
}
