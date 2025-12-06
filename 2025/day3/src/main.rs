fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap();

    let data = std::fs::read_to_string(filename).unwrap();

    let total: usize = data
        .lines()
        .map(|line| {
            const NUM_DIGITS: usize = 12;

            let mut cur_skip_cnt = 0;
            let number = (0..NUM_DIGITS)
                .map(|idx| {
                    let (skip_cnt, cur_digit) = line
                        .chars()
                        .take(line.len() - (NUM_DIGITS - idx - 1))
                        .skip(cur_skip_cnt)
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

                    cur_skip_cnt += skip_cnt + 1;
                    cur_digit.to_digit(10).unwrap() as usize
                })
                .reduce(|acc, val| acc * 10 + val)
                .unwrap();
            number
        })
        .sum();

    println!("total: {total}");
}
