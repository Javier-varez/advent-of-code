fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data: Vec<(usize, usize)> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let max = data
        .iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            data.iter().skip(i + 1).map(|(x2, y2)| {
                let width = ((*x1 as isize) - (*x2 as isize)).abs() + 1;
                let height = ((*y1 as isize) - (*y2 as isize)).abs() + 1;
                width * height
            })
        })
        .max()
        .unwrap();
    println!("max {max}");
}
