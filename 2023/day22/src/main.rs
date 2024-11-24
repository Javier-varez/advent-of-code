use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

type Coord = [usize; 3];

pub trait IterToNumbers {
    fn to_numbers(self) -> Numbers<Self>
    where
        Self: Sized;
}

impl<'a, U, T> IterToNumbers for U
where
    U: std::iter::Iterator<Item = T>,
    T: AsRef<str>,
{
    fn to_numbers(self) -> Numbers<Self> {
        Numbers(self)
    }
}

pub struct Numbers<U>(U);

impl<'a, U> std::iter::Iterator for Numbers<U>
where
    U: std::iter::Iterator<Item = &'a str>,
{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| i.parse().unwrap())
    }
}

type Brick = (Coord, Coord);

fn overlaps(
    ([xs1, ys1, _], [xe1, ye1, _]): &Brick,
    ([xs2, ys2, _], [xe2, ye2, _]): &Brick,
) -> bool {
    let x_overlap = xe1 >= xs2 && xe2 >= xs1;
    let y_overlap = ye1 >= ys2 && ye2 >= ys1;
    x_overlap && y_overlap
}

fn collapse(mut bricks: Vec<(Coord, Coord)>) -> Vec<(Coord, Coord)> {
    bricks.sort_by(|([_, _, z1], [_, _, _]), ([_, _, z2], [_, _, _])| z1.cmp(z2));
    // println!("bricks: {:?}", bricks);

    for i in 0..bricks.len() {
        let mut min_z = 1;
        let (already_collapsed, current) = {
            let (already_collapsed, not_collapsed) = bricks.split_at_mut(i);
            (&*already_collapsed, &mut not_collapsed[0])
        };

        for (_j, cur_collapsed) in already_collapsed.iter().enumerate() {
            if overlaps(cur_collapsed, current) {
                min_z = min_z.max(cur_collapsed.1[2] + 1);
            }
        }
        // Brick coordinates must be in increasing order
        assert!(current.0[2] <= current.1[2]);
        assert!(current.0[2] >= min_z);
        let drop = current.0[2] - min_z;
        current.0[2] -= drop;
        current.1[2] -= drop;
    }

    bricks
}

fn support_map(bricks: &[Brick]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut supports_map = vec![vec![]; bricks.len()];
    let mut is_supported_by_map = vec![vec![]; bricks.len()];
    for (i, brick) in bricks.iter().enumerate() {
        let brick_z = brick.0[2];
        for (j, lower) in bricks.iter().take(i).enumerate() {
            let lower_z = lower.1[2];
            if overlaps(brick, lower) && lower_z + 1 == brick_z {
                is_supported_by_map[i].push(j);
                supports_map[j].push(i);
            }
        }
    }
    (supports_map, is_supported_by_map)
}

fn main() {
    let bricks = std::io::stdin()
        .lines()
        .map(|l| -> (Coord, Coord) {
            l.unwrap()
                .split("~")
                .map(|coord| {
                    coord
                        .split(",")
                        .to_numbers()
                        .collect_vec()
                        .try_into()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    let bricks = collapse(bricks);

    let (support_map, is_supported_by_map) = support_map(&bricks);
    let mut can_be_removed = vec![true; bricks.len()];
    for supporting_bricks in &is_supported_by_map {
        if supporting_bricks.len() == 1 {
            can_be_removed[supporting_bricks[0]] = false;
        }
    }

    let mut total_collapsed = vec![0usize; bricks.len()];
    for i in 0..bricks.len() {
        let mut checked = HashSet::new();
        let mut stack = VecDeque::new();
        stack.push_back(i);
        checked.insert(i);

        while let Some(cur) = stack.pop_front() {
            for supported_by_i in &support_map[cur] {
                if is_supported_by_map[*supported_by_i]
                    .iter()
                    .all(|brick_idx| checked.contains(brick_idx))
                {
                    if !checked.contains(supported_by_i) {
                        stack.push_back(*supported_by_i);
                        checked.insert(*supported_by_i);
                    }
                }
            }
        }
        total_collapsed[i] = checked.iter().count() - 1;
    }

    println!("total collapsed {total_collapsed:?}");
    println!("max collapsed {}", total_collapsed.iter().sum::<usize>());
    println!("total bricks {}", bricks.len());
}
