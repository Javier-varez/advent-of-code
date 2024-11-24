use anyhow::Result;

type Pattern = Vec<Vec<char>>;

fn parse_patterns(lines: &[String]) -> Vec<Pattern> {
    let mut patterns = vec![];
    let mut current_pattern = vec![];
    for line in lines {
        if line.trim() == "" {
            patterns.push(current_pattern);
            current_pattern = vec![];
            continue;
        }

        current_pattern.push(line.chars().collect());
    }

    if !current_pattern.is_empty() {
        patterns.push(current_pattern)
    }

    patterns
}

fn print_matrix<T, U, V>(matrix: V)
where
    V: AsRef<[U]>,
    U: AsRef<[T]>,
    T: std::fmt::Display,
{
    println!("[");
    for line in matrix.as_ref().iter() {
        print!("\t");
        for c in line.as_ref().iter() {
            print!("{c}, ");
        }
        println!("")
    }
    println!("]");
}

fn print_patterns<U, T, V, Y>(patterns: Y)
where
    Y: AsRef<[V]>,
    V: AsRef<[U]>,
    U: AsRef<[T]>,
    T: std::fmt::Display,
{
    for (i, pattern) in patterns.as_ref().iter().enumerate() {
        println!("Pattern {i}");
        print_matrix(pattern);
    }
}

fn check_diffs(l: &[char], r: &[char]) -> usize {
    assert_eq!(l.len(), r.len());
    l.iter()
        .zip(r.iter())
        .fold(0, |acc, (l, r)| if l == r { acc } else { acc + 1 })
}

fn find_horizontal_reflection(pattern: &Pattern) -> Option<usize> {
    for pivot in 0..pattern.len() - 1 {
        let mut i = pivot;
        let mut j = pivot + 1;
        let mut diffs = 0;

        // We consider the pivot and walk i and j outwards until they don't match or
        // one of the indexes is already going to fall out of the range

        loop {
            diffs += check_diffs(&pattern[i], &pattern[j]);
            if diffs > 1 {
                break;
            }

            if (i == 0) || (j == pattern.len() - 1) {
                if diffs == 1 {
                    return Some(pivot);
                } else {
                    break;
                }
            }
            i -= 1;
            j += 1;
        }
    }
    None
}

fn transpose(pattern: &Pattern) -> Pattern {
    let rows = pattern.len();
    let cols = pattern[0].len();
    let mut res = vec![vec![' '; rows]; cols];

    for (i, row) in pattern.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            res[j][i] = *elem;
        }
    }

    res
}

fn find_vertical_reflection(pattern: &Pattern) -> Option<usize> {
    let pattern = transpose(pattern);
    find_horizontal_reflection(&pattern)
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lines()
        .map(|c| Ok(c?))
        .collect::<Result<Vec<String>>>()?;

    let patterns = parse_patterns(&lines);
    print_patterns(&patterns);

    let mut count = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        if let Some(pivot) = find_vertical_reflection(&pattern) {
            println!("Found vertical reflection for pattern {i} at pivot {pivot}");
            count += pivot + 1;
        } else {
            println!("No vertical reflection for pattern {i}");
        }

        if let Some(pivot) = find_horizontal_reflection(&pattern) {
            println!("Found horizontal reflection for pattern {i} at pivot {pivot}");
            count += 100 * (pivot + 1);
        } else {
            println!("No horizontal reflection for pattern {i}");
        }
    }
    println!("count {count}");

    Ok(())
}
