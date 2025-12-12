fn parse_shape<'a, T: Iterator<Item = &'a str>>(a: &mut T) -> anyhow::Result<usize> {
    let i: usize = a.next().unwrap().trim_matches(':').parse()?;
    println!("{i}");

    Ok(a.take_while(|a| !a.trim().is_empty())
        .map(|c| c.chars().filter(|c| *c == '#').count())
        .sum())
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(filename).unwrap();
    let mut lines = data.lines();

    let mut shapes = vec![];
    loop {
        let mut iter = lines.clone();
        let Ok(area) = parse_shape(&mut iter) else {
            break;
        };
        lines = iter;
        shapes.push(area);
    }

    let count = lines
        .filter(|l| {
            let (area, counts) = l.split_once(":").unwrap();
            let (l, r) = area.split_once("x").unwrap();
            let l: usize = l.parse().unwrap();
            let r: usize = r.parse().unwrap();
            let region_area = l * r;

            let req_area: usize = counts
                .split_whitespace()
                .zip(&shapes)
                .map(|(c, s)| {
                    let c: usize = c.parse().unwrap();
                    c * s
                })
                .sum();
            req_area < region_area
        })
        .count();

    println!("count: {count}");
}
