use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            &Direction::Down => Direction::Up,
            &Direction::Up => Direction::Down,
            &Direction::Left => Direction::Right,
            &Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Trace {
    direction: Option<Direction>,
    consecutive: usize,
}

impl Trace {
    fn new() -> Self {
        Self {
            direction: None,
            consecutive: 0,
        }
    }

    fn with_movement(&self, dir: Direction) -> Self {
        if dir == self.direction.unwrap_or(Direction::Down) {
            Self {
                consecutive: self.consecutive + 1,
                direction: self.direction,
            }
        } else {
            Self {
                consecutive: 1,
                direction: Some(dir),
            }
        }
    }
}

struct HeapEntry {
    cost: usize,
    // Where we are
    row_col: (usize, usize),
    // How we got there
    trace: Trace,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for HeapEntry {}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reverse is used to convert max-queue (BinaryHeap) into a min-queue
        std::cmp::Reverse(self.cost).partial_cmp(&std::cmp::Reverse(other.cost))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse is used to convert max-queue (BinaryHeap) into a min-queue
        std::cmp::Reverse(self.cost).cmp(&std::cmp::Reverse(other.cost))
    }
}

fn dir_to_incr(dir: Direction) -> (isize, isize) {
    match dir {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn compute_directions(
    (max_row, max_col): (usize, usize),
    (cur_row, cur_col): (usize, usize),
    trace: &Trace,
) -> Vec<(usize, usize, Direction)> {
    const DIRS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    let cur_row = cur_row as isize;
    let cur_col = cur_col as isize;
    let max_row = max_row as isize;
    let max_col = max_col as isize;

    fn can_move_in_this_dir(trace: &Trace, cur: Direction) -> bool {
        const MAX_CONSECUTIVE: usize = 10;
        const MIN_CONSECUTIVE: usize = 4;

        let backwards = trace.direction.is_some_and(|d| d.opposite() == cur);
        let too_many_consecutive = trace.with_movement(cur).consecutive > MAX_CONSECUTIVE;
        let too_little_consecutive =
            trace.consecutive < MIN_CONSECUTIVE && trace.direction.is_some_and(|d| d != cur);

        !backwards && !too_many_consecutive && !too_little_consecutive
    }

    DIRS.iter()
        .filter_map(|dir| {
            let (incr_row, incr_col) = dir_to_incr(*dir);
            let (next_row, next_col) = (cur_row + incr_row, cur_col + incr_col);
            if next_row < 0 || next_row >= max_row || next_col < 0 || next_col >= max_col {
                return None;
            }

            if !can_move_in_this_dir(trace, *dir) {
                return None;
            }

            Some((next_row as usize, next_col as usize, *dir))
        })
        .collect()
}

fn shortest_path(entry: (usize, usize), map: &[Vec<usize>]) -> usize {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    let mut current = entry;
    let mut cur_cost = 0;
    let mut trace = Trace::new();

    loop {
        let max = (map.len(), map[0].len());
        let end = (max.0 - 1, max.1 - 1);

        if current == end && trace.consecutive == 4 {
            return cur_cost;
        }

        let dirs = compute_directions(max, current, &trace);

        for (next_row, next_col, dir) in dirs {
            let cost = cur_cost + map[next_row][next_col];
            let trace = trace.with_movement(dir);

            queue.push(HeapEntry {
                cost,
                row_col: (next_row, next_col),
                trace,
            });
        }

        let (cost, row_col, new_trace) = loop {
            // Visit next node and return that
            let HeapEntry {
                cost,
                row_col,
                trace: new_trace,
            } = queue.pop().unwrap();

            let visited_entry = (row_col, new_trace);
            if !visited.contains(&visited_entry) {
                visited.insert(visited_entry);
                break (cost, row_col, new_trace);
            }
        };

        current = row_col;
        cur_cost = cost;
        trace = new_trace;
    }
}

fn main() {
    let map: Vec<Vec<usize>> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().bytes().map(|c| (c - b'0').into()).collect())
        .collect();

    let min_cost = shortest_path((0, 0), &map);

    println!("Hello, world! {min_cost}");
}
