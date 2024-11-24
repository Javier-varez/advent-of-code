use itertools::Itertools;

type Coord = [isize; 3];

fn time_for_point((x, _): (f64, f64), ([bx, _, ..], [ax, _, ..]): &(Coord, Coord)) -> f64 {
    // x = a * t + b
    // (x - b) / a = t
    let t = (x - *bx as f64) / (*ax as f64);
    t
}

const TEST_AREA: (f64, f64) = (200000000000000.0, 400000000000000.0);

fn main() {
    let input: Vec<(Coord, Coord)> = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split("@")
                .map(|s| {
                    s.trim()
                        .split(",")
                        .map(|s| s.trim().parse().unwrap())
                        .collect_vec()
                        .try_into()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    // y = a1 * x + b1
    // y = a2 * x + b2
    //
    // a1 * x + b1 = a2 * x + b2
    // (a1 - a2) * x = (b2 - b1)
    // x = (b2 - b1) / (a1 - a2)

    let trayectories = input
        .iter()
        .map(|([bx, by, ..], [ax, ay, ..])| {
            let slope = *ay as f64 / *ax as f64;
            let b = *by as f64 - slope * *bx as f64;
            (slope, b)
        })
        .collect_vec();

    let mut count = 0;
    for (i, (a1, b1)) in trayectories.iter().enumerate() {
        for (j, (a2, b2)) in trayectories.iter().enumerate().skip(i + 1) {
            println!("intersection between {i} and {j}:");
            if a1 == a2 {
                println!("\t lines are parallel");
            } else {
                let x = (b2 - b1) / (a1 - a2);
                let y = a1 * x + b1;
                println!("\tx: {x}, y: {y}");
                if time_for_point((x, y), &input[i]) < 0.0 {
                    println!("\tbut it is in the past for {i}!");
                    continue;
                }
                if time_for_point((x, y), &input[j]) < 0.0 {
                    println!("\tbut it is in the past for {j}!");
                    continue;
                }
                if x < TEST_AREA.0 || x > TEST_AREA.1 || y < TEST_AREA.0 || y > TEST_AREA.1 {
                    println!("\tbut it is outside test area!");
                    continue;
                }
                count += 1;
            }
        }
    }
    println!("count {count}");
}
