use std::collections::HashSet;

fn distance(left: (usize, usize, usize), right: (usize, usize, usize)) -> f64 {
    let x_distance = (left.0 as f64) - (right.0 as f64);
    let y_distance = (left.1 as f64) - (right.1 as f64);
    let z_distance = (left.2 as f64) - (right.2 as f64);

    (x_distance * x_distance + y_distance * y_distance + z_distance * z_distance).sqrt()
}

fn precompute_distances(boxes: &[(usize, usize, usize)]) -> Vec<((usize, usize), f64)> {
    let mut result = vec![];

    for (from, from_box_coords) in boxes.iter().enumerate() {
        for (to, to_box_coords) in boxes.iter().enumerate().skip(from + 1) {
            result.push(((from, to), distance(*from_box_coords, *to_box_coords)));
        }
    }

    // Sort by distance
    result.sort_by(|(_, left_distance), (_, right_distance)| {
        left_distance.partial_cmp(right_distance).unwrap()
    });

    result
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let boxes: Vec<(usize, usize, usize)> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            let (left, more) = l.split_once(",").unwrap();
            let (middle, right) = more.split_once(",").unwrap();

            (
                left.parse().unwrap(),
                middle.parse().unwrap(),
                right.parse().unwrap(),
            )
        })
        .collect();

    let num_boxes = boxes.len();

    let distances = precompute_distances(&boxes);
    let mut groups: Vec<HashSet<usize>> = vec![];

    for ((from, to), _) in distances.iter().cloned() {
        let from_group = groups
            .iter()
            .enumerate()
            .find_map(|(idx, group)| group.contains(&from).then_some(idx));
        let to_group = groups
            .iter()
            .enumerate()
            .find_map(|(idx, group)| group.contains(&to).then_some(idx));

        match (from_group, to_group) {
            (Some(from), Some(to)) if from == to => {}
            (Some(from), Some(to)) => {
                let to_group = groups[to].clone();
                for idx in to_group {
                    groups[from].insert(idx);
                }
                groups.remove(to);
            }
            (Some(from), None) => {
                groups[from].insert(to);
            }
            (None, Some(to)) => {
                groups[to].insert(from);
            }
            (None, None) => {
                let mut group = HashSet::new();
                group.insert(from);
                group.insert(to);
                groups.push(group);
            }
        }

        if groups.len() == 1 && groups[0].len() == num_boxes {
            println!("found result.");

            println!("From box: {:?}", boxes[from]);
            println!("To box: {:?}", boxes[to]);
            println!("Result: {}", boxes[from].0 * boxes[to].0);
            break;
        }
    }
}
