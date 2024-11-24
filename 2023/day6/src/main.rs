fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap()).collect();

    let mut iter = lines.iter();

    let time = iter
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let distance = iter
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let t = time as f64;
    let d = (distance + 1) as f64;
    let min_time = (t - f64::sqrt(t * t - 4.0 * d)) / 2.0;
    let max_time = (t + f64::sqrt(t * t - 4.0 * d)) / 2.0;
    let min_time = min_time.ceil() as usize;
    let max_time = max_time.floor() as usize;
    println!("T {time}, D {distance}, ({min_time} - {max_time})");

    let num_sols = max_time - min_time + 1;
    println!("num sols {num_sols}");
}
