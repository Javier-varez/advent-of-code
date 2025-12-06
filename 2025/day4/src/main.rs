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

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data: Vec<Vec<char>> = std::fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = data.len();
    let width = data.first().unwrap().len();

    let total = (0..height).fold(0, |acc, y| {
        acc + (0..width).fold(0, |acc, x| {
            if data[y][x] != '@' {
                acc
            } else {
                let adj = count_adjacent(&data, x, y, width, height);
                let res = if adj < 4 { 1 } else { 0 };
                res + acc
            }
        })
    });
    println!("total {total}");
}
