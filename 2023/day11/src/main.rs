use anyhow::Result;

fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = v[0].len();
    let mut new = vec![];

    for i in 0..rows {
        let mut row = vec![];
        for item in v.iter() {
            row.push(item[i]);
        }
        new.push(row);
    }

    new
}

struct Map(Vec<Vec<char>>);

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for line in &self.0 {
            writeln!(f, "\t{:?}", line)?;
        }
        writeln!(f, "[")
    }
}

fn collect_empty_rows(v: &Map) -> Vec<usize> {
    v.0.iter()
        .enumerate()
        .filter_map(|(r_idx, l)| {
            if l.iter().all(|c| *c == '.') {
                Some(r_idx)
            } else {
                None
            }
        })
        .collect()
}

fn collect_empty_cols(v: &Map) -> Vec<usize> {
    let transposed = transpose(v.0.clone());
    transposed
        .iter()
        .enumerate()
        .filter_map(|(r_idx, l)| {
            if l.iter().all(|c| *c == '.') {
                Some(r_idx)
            } else {
                None
            }
        })
        .collect()
}

fn collect_stars(v: &Map) -> Vec<(usize, usize)> {
    let coords: Vec<(usize, usize)> =
        v.0.iter()
            .enumerate()
            .flat_map(|(r_idx, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(|(c_idx, c)| {
                        if *c == '#' {
                            Some((r_idx, c_idx))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();
    coords
}

type Coord = (usize, usize);

fn distance(a: Coord, b: Coord, empty_rows: &[usize], empty_cols: &[usize]) -> usize {
    let (first_row, last_row) = if a.0 >= b.0 { (b.0, a.0) } else { (a.0, b.0) };
    let (first_col, last_col) = if a.1 >= b.1 { (b.1, a.1) } else { (a.1, b.1) };

    let num_empty_rows = empty_rows
        .iter()
        .filter(|r_idx| (first_row < **r_idx) && (last_row > **r_idx))
        .count();
    let num_empty_cols = empty_cols
        .iter()
        .filter(|c_idx| (first_col < **c_idx) && (last_col > **c_idx))
        .count();
    const EXPANSION_FACTOR: usize = 1000000;

    let col_dist = last_col - first_col;
    let row_dist = last_row - first_row;
    col_dist + row_dist + (num_empty_cols + num_empty_rows) * (EXPANSION_FACTOR - 1)
}

fn main() -> Result<()> {
    let input = std::io::stdin()
        .lines()
        .map(|l| Ok(l?.chars().collect::<Vec<char>>()))
        .collect::<Result<Vec<Vec<char>>>>()?;

    let map = Map(input);

    let star_coords = collect_stars(&map);
    let empty_rows = collect_empty_rows(&map);
    let empty_cols = collect_empty_cols(&map);

    let mut total = 0;
    for (idx, first) in star_coords.iter().enumerate() {
        for second in star_coords.iter().skip(idx + 1) {
            total += distance(*first, *second, &empty_rows, &empty_cols);
        }
    }
    println!("total {total}");

    Ok(())
}
