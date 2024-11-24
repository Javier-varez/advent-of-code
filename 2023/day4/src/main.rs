struct Game {
    my_numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

fn parse(lines: &[String]) -> Vec<Game> {
    lines
        .iter()
        .map(|line| {
            let winning_seq: Vec<usize> = line
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split("|")
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let my_seq: Vec<usize> = line
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split("|")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            eprintln!("Winning: {winning_seq:?}, MySeq: {my_seq:?}");
            Game {
                my_numbers: my_seq,
                winning_numbers: winning_seq,
            }
        })
        .collect()
}

fn part2(games: &[Game]) {
    let mut copies = vec![1; games.len()];
    for (idx, g) in games.iter().enumerate() {
        let count = g
            .my_numbers
            .iter()
            .filter(|n| g.winning_numbers.contains(n))
            .count();

        let copies_of_current_card = copies[idx];
        copies
            .iter_mut()
            .skip(idx + 1)
            .take(count)
            .for_each(|v| *v += copies_of_current_card);
    }
    let sum: usize = copies.iter().sum();
    println!("sum {sum}");
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|l| l.unwrap()).collect();

    let parsed = parse(&lines);

    part2(&parsed);
}
