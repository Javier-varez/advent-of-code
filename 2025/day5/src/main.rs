fn is_fresh(id: usize, ranges: &[(usize, usize)]) -> bool {
    ranges.iter().any(|(begin, end)| id >= *begin && id <= *end)
}

fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap();

    let data = std::fs::read_to_string(filename).unwrap();

    let ranges: Vec<_> = data
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (l, r) = l.split_once("-").unwrap();
            let l: usize = l.parse().unwrap();
            let r: usize = r.parse().unwrap();
            (l, r)
        })
        .collect();

    let mut found_new_line = false;
    let available: Vec<usize> = data
        .lines()
        .skip_while(|l| {
            let r = !found_new_line;
            found_new_line |= l.is_empty();
            r
        })
        .map(|l| l.parse().unwrap())
        .collect();

    let val = available.iter().fold(0, |acc, id| {
        acc + if is_fresh(*id, &ranges) { 1 } else { 0 }
    });
    println!("ranges: {ranges:?}, ingredient_ids: {available:?}");
    println!("count: {val}");
}
