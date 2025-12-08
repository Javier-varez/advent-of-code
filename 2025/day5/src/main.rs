fn is_fresh(id: usize, ranges: &[(usize, usize)]) -> bool {
    ranges.iter().any(|(begin, end)| id >= *begin && id <= *end)
}

fn join_ranges(left: (usize, usize), right: (usize, usize)) -> Option<(usize, usize)> {
    if left.1 >= right.0 && right.1 >= left.0 {
        let begin = left.0.min(right.0);
        let end = left.1.max(right.1);
        Some((begin, end))
    } else {
        None
    }
}

fn deduplicate_ranges(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut new_ranges = vec![];

    for range in ranges {
        if let Some((id, new)) = new_ranges
            .iter_mut()
            .enumerate()
            .find_map(|(id, new)| join_ranges(*new, *range).map(|v| (id, v)))
        {
            new_ranges[id] = new;
        } else {
            new_ranges.push(*range);
        }
    }

    new_ranges
}

fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap();

    let data = std::fs::read_to_string(filename).unwrap();

    let mut ranges: Vec<_> = data
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
    println!("count: {val}");

    loop {
        let new_ranges = deduplicate_ranges(&ranges);
        if new_ranges.len() == ranges.len() {
            break;
        }
        ranges = new_ranges;
    }

    let result = ranges
        .iter()
        .fold(0, |acc, (begin, end)| acc + (end - begin + 1));
    println!("fresh_ids: {result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn join_ranges_test() {
        assert_eq!(join_ranges((0, 12), (12, 23)), Some((0, 23)));
        assert_eq!(join_ranges((0, 13), (12, 23)), Some((0, 23)));
        assert_eq!(join_ranges((0, 24), (12, 23)), Some((0, 24)));
        assert_eq!(join_ranges((11, 24), (12, 23)), Some((11, 24)));
        assert_eq!(join_ranges((12, 24), (12, 23)), Some((12, 24)));
        assert_eq!(join_ranges((13, 24), (12, 23)), Some((12, 24)));
        assert_eq!(join_ranges((0, 11), (12, 23)), None);
        assert_eq!(join_ranges((12, 23), (0, 11)), None);
    }
}
