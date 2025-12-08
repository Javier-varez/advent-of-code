fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(filename).unwrap();

    let mut matrix: Vec<Vec<String>> = data
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    let operations = matrix.last().unwrap().clone();
    matrix.pop(); //remove operations line

    let len = matrix[0].len();
    let mut iters: Vec<_> = matrix.into_iter().map(|r| r.into_iter()).collect();

    let matrix: Vec<Vec<usize>> = (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|iter| iter.next().unwrap().parse().unwrap())
                .collect()
        })
        .collect();

    let result = operations
        .iter()
        .zip(matrix.iter())
        .fold(0, |acc, (op, data)| {
            let result: usize = if op == "+" {
                data.iter().sum()
            } else {
                data.iter().product()
            };
            acc + result
        });

    println!("result {result}");
}
