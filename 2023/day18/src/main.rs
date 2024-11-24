use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Instruction {
    direction: Direction,
    amount: usize,
}

fn parse_direction(dir: char) -> anyhow::Result<Direction> {
    match dir {
        '3' => Ok(Direction::Up),
        '1' => Ok(Direction::Down),
        '2' => Ok(Direction::Left),
        '0' => Ok(Direction::Right),
        d => Err(anyhow::anyhow!("Unknown direction {d}")),
    }
}

fn parse_line(l: &str) -> anyhow::Result<Instruction> {
    let mut iter = l.split_ascii_whitespace();
    let _ = iter.next().ok_or(anyhow::anyhow!("Expected first char"))?;
    let _ = iter.next().ok_or(anyhow::anyhow!("Expected second char"))?;
    let mixed = iter
        .next()
        .ok_or(anyhow::anyhow!("Invalid direction"))?
        .trim_start_matches("(#")
        .trim_end_matches(")");
    let amount = usize::from_str_radix(&mixed.chars().take(5).collect::<String>(), 16)?;
    let direction = parse_direction(
        mixed
            .chars()
            .last()
            .ok_or(anyhow::anyhow!("Unkown direction"))?,
    )?;

    Ok(Instruction { direction, amount })
}

type Coord = (isize, isize);

fn walk((row, col): Coord, dir: Direction) -> Coord {
    match dir {
        Direction::Up => (row - 1, col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col - 1),
        Direction::Right => (row, col + 1),
    }
}

struct Map(HashMap<isize, Vec<(isize, Direction, Direction)>>);

impl Map {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, (row, col): Coord, prev_dir: Direction, next_dir: Direction) {
        if let Some(row) = self.0.get_mut(&row) {
            row.push((col, prev_dir, next_dir));
        } else {
            self.0.insert(row, vec![(col, prev_dir, next_dir)]);
        }
    }
}

fn follow_instructions(insns: &[Instruction]) -> Map {
    let mut result = Map::new();

    let mut current = (0, 0);

    let cur = insns.iter();
    let next = insns.iter().skip(1).chain(std::iter::once(&insns[0]));

    for (cur_insn, next_insn) in cur.zip(next) {
        assert!(cur_insn.amount >= 1);

        for _i in 0..cur_insn.amount - 1 {
            let next = walk(current, cur_insn.direction);
            result.insert(next, cur_insn.direction, cur_insn.direction);
            current = next;
        }

        let next = walk(current, cur_insn.direction);
        result.insert(next, cur_insn.direction, next_insn.direction);
        current = next;
    }
    assert_eq!(current, (0, 0));

    result
}

fn area(map: &Map) -> usize {
    let mut count = 0usize;
    for (_row, cols) in map.0.iter() {
        let mut cols = cols.clone();
        cols.sort_by(|(col_a, _, _), (col_b, _, _)| col_a.cmp(col_b));

        let mut inner = false;
        let mut last = None;
        let mut first_edge = None;

        for (col, prev_dir, next_dir) in cols {
            count += 1;

            if let Some(last) = last {
                assert!(col > last);
                if last + 1 < col && inner {
                    count += (col - last - 1) as usize;
                }
            }

            if prev_dir != next_dir {
                if next_dir == Direction::Right {
                    assert!(first_edge.is_none());
                    first_edge = Some(prev_dir);
                } else if prev_dir == Direction::Left {
                    assert!(first_edge.is_none());
                    first_edge = Some(next_dir);
                } else if prev_dir == Direction::Right {
                    assert!(first_edge.is_some());
                    if next_dir == first_edge.unwrap() {
                        inner = !inner;
                    }
                    first_edge = None;
                } else if next_dir == Direction::Left {
                    assert!(first_edge.is_some());
                    if prev_dir == first_edge.unwrap() {
                        inner = !inner;
                    }
                    first_edge = None;
                }
            } else if prev_dir == Direction::Up || prev_dir == Direction::Down {
                assert!(first_edge.is_none());
                inner = !inner;
            }

            last = Some(col);
        }
    }

    count
}

fn main() -> anyhow::Result<()> {
    let instructions = std::io::stdin()
        .lines()
        .map(|l| Ok(parse_line(&l?)?))
        .collect::<anyhow::Result<Vec<Instruction>>>()?;

    let mut last = instructions.first().unwrap();
    for i in instructions.iter().skip(1) {
        // This solution only works if there are no sudden changes in opposite directions
        assert!(i.direction != last.direction.opposite());
        last = i;
    }

    let map = follow_instructions(&instructions);
    let count = area(&map);

    println!("count {count}");

    Ok(())
}
