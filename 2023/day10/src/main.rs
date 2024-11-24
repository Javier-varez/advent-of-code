use std::collections::HashMap;

#[derive(Debug)]
struct Map(Vec<Vec<char>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord((usize, usize));

struct Contour(HashMap<Coord, char>);

impl Coord {
    fn up(&self) -> Option<Coord> {
        let (r, c) = self.0;
        if r == 0 {
            None
        } else {
            Some(Self((r - 1, c)))
        }
    }

    fn down(&self, map: &Map) -> Option<Coord> {
        let (r, c) = self.0;
        if r == map.rows() - 1 {
            None
        } else {
            Some(Self((r + 1, c)))
        }
    }

    fn left(&self) -> Option<Coord> {
        let (r, c) = self.0;
        if c == 0 {
            None
        } else {
            Some(Self((r, c - 1)))
        }
    }

    fn right(&self, map: &Map) -> Option<Coord> {
        let (r, c) = self.0;
        if c == map.cols() - 1 {
            None
        } else {
            Some(Self((r, c + 1)))
        }
    }
}

impl Map {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.0[0].len()
    }

    fn at(&self, c: Coord) -> char {
        let (r, c) = c.0;
        self.0[r][c]
    }

    fn find_start(&self) -> Coord {
        Coord(
            self.0
                .iter()
                .enumerate()
                .find_map(|(r, m)| {
                    if let Some(c) =
                        m.iter()
                            .enumerate()
                            .find_map(|(c, m)| if *m == 'S' { Some(c) } else { None })
                    {
                        Some((r, c))
                    } else {
                        None
                    }
                })
                .unwrap(),
        )
    }

    fn first_step(&self, start: Coord) -> (Coord, char) {
        let up = start.up().is_some_and(|c| {
            let up = self.at(c);
            up == '|' || up == 'F' || up == '7'
        });
        let down = start.down(self).is_some_and(|c| {
            let down = self.at(c);
            down == '|' || down == 'L' || down == 'J'
        });
        let left = start.left().is_some_and(|c| {
            let left = self.at(c);
            left == '-' || left == 'L' || left == 'F'
        });
        let right = start.right(self).is_some_and(|c| {
            let right = self.at(c);
            right == '-' || right == '7' || right == 'J'
        });

        match (up, down, left, right) {
            (true, true, false, false) => (start.up().unwrap(), '|'),
            (true, false, false, true) => (start.up().unwrap(), 'L'),
            (true, false, true, false) => (start.up().unwrap(), 'J'),
            (false, true, true, false) => (start.left().unwrap(), '7'),
            (false, true, false, true) => (start.right(self).unwrap(), 'F'),
            (false, false, true, true) => (start.left().unwrap(), '-'),
            c => {
                panic!("Invalid surroundings {c:?}");
            }
        }
    }

    fn next(&self, c: Coord) -> (Coord, Coord) {
        let ch = self.at(c);
        match ch {
            '-' => (c.left().unwrap(), c.right(self).unwrap()),
            '|' => (c.up().unwrap(), c.down(self).unwrap()),
            'F' => (c.right(self).unwrap(), c.down(self).unwrap()),
            'L' => (c.right(self).unwrap(), c.up().unwrap()),
            '7' => (c.left().unwrap(), c.down(self).unwrap()),
            'J' => (c.left().unwrap(), c.up().unwrap()),
            _ => unimplemented!(),
        }
    }
}

fn build_contour(map: &Map) -> Contour {
    let mut contour = HashMap::new();
    let s = map.find_start();

    let mut steps = 1;
    let mut prev = s;

    // Derive first step
    let (mut position, subs) = map.first_step(s);
    println!("start {s:?}, subs: {subs}");
    contour.insert(position, map.at(position));
    contour.insert(s, subs);

    while position != s || steps == 0 {
        let next = match map.next(position) {
            (a, b) if a == prev => b,
            (a, b) if b == prev => a,
            _ => unimplemented!(),
        };

        if map.at(next) != 'S' {
            contour.insert(next, map.at(next));
        }

        prev = position;
        position = next;

        steps += 1;
    }

    Contour(contour)
}

// |iL7o
// |iLJi
// oLJo
// |iL---JiiiL--7oo

fn main() {
    let map = Map(std::io::stdin()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            l.chars().collect()
        })
        .collect());

    let contour = build_contour(&map);

    // |iL---JiiiL--7oo
    // count inner tiles
    let mut count = 0;
    for row in 0..map.rows() {
        let mut inner = false;
        let mut last_bound = None;
        for col in 0..map.cols() {
            let c = contour.0.get(&Coord((row, col))).unwrap_or(&'.');
            match c {
                '-' => {
                    // No changes
                }
                '|' => {
                    inner = !inner;
                    last_bound = None;
                }
                'F' | 'L' => {
                    last_bound = Some(c);
                }
                '7' | 'J' => {
                    match (last_bound.unwrap(), c) {
                        ('F', 'J') | ('L', '7') => {
                            inner = !inner;
                        }
                        ('F', '7') | ('L', 'J') => {}
                        c => {
                            panic!("What?: {c:?}");
                        }
                    };
                    last_bound = None;
                }
                '.' => {
                    if inner {
                        count += 1;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
    println!("count: {count}");
}
