fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap();

    let data = std::fs::read_to_string(filename).unwrap();

    let total: usize = data
        .lines()
        .map(|line| {
            let (skip_cnt, first_digit) = line
                .chars()
                .take(line.len() - 1)
                .enumerate()
                .max_by(|(_, lhs), (_, rhs)| match lhs.cmp(rhs) {
                    // There's a small gotcha here. The number can appear multiple times in the
                    // same line. When several numbers are equal, we need to take the first one,
                    // which maximizes our chances of finding the largest second number (as we have
                    // more). To do this, we can Force the algorithm to think that the first number
                    // is larger when they are really equal. The default behavior of this function
                    // returns the last largest number.
                    std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
                    a => a,
                })
                .unwrap();
            let first_digit = first_digit.to_digit(10).unwrap();
            let second_digit = line.chars().skip(skip_cnt + 1).max().unwrap();
            let second_digit = second_digit.to_digit(10).unwrap();
            let number = first_digit * 10 + second_digit;
            number as usize
        })
        .sum();

    println!("total: {total}");
}
