fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(filename).unwrap();

    let matrix: Vec<String> = data.lines().map(|l| l.to_string()).collect();

    let width = matrix[0].len();

    let mut ops = vec![];

    let mut last_non_empty_col = 0;
    (0..width).for_each(|cur_col| {
        let empty_column = matrix
            .iter()
            .all(|row| row.chars().nth(cur_col).unwrap() == ' ');
        if empty_column {
            let cur_op: Vec<Vec<char>> = matrix
                .iter()
                .map(|row| {
                    row.chars()
                        .skip(last_non_empty_col)
                        .take(cur_col - last_non_empty_col)
                        .collect::<Vec<char>>()
                })
                .collect();
            ops.push(cur_op);

            last_non_empty_col = cur_col + 1;
        }
    });

    if last_non_empty_col != width {
        let cur_op: Vec<Vec<char>> = matrix
            .iter()
            .map(|row| {
                row.chars()
                    .skip(last_non_empty_col)
                    .take(width - last_non_empty_col)
                    .collect::<Vec<char>>()
            })
            .collect();
        ops.push(cur_op);
    }

    let result: usize = ops
        .iter()
        .map(|op| {
            let width = op[0].len();
            let height = op.len() - 1;

            let operands: Vec<usize> = (0..width)
                .map(|col| {
                    let number: String = op.iter().take(height).map(|row| row[col]).collect();
                    number.trim().parse().unwrap()
                })
                .collect();

            let op = op[height].first().unwrap();

            let result: usize = if *op == '*' {
                operands.iter().product()
            } else {
                operands.iter().sum()
            };
            result
        })
        .sum();
    println!("Result is {result}");
}
