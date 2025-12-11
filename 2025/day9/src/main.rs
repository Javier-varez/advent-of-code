fn area((p1, p2): (Point, Point)) -> usize {
    let width = ((p1.0 as isize) - (p2.0 as isize)).abs() + 1;
    let height = ((p1.1 as isize) - (p2.1 as isize)).abs() + 1;
    (width * height) as usize
}

#[derive(Clone, Copy, Debug)]
struct Point(usize, usize);

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    pub fn min(&self, other: &Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }

    pub fn max(&self, other: &Self) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }

    pub fn mid(&self, other: &Self) -> Self {
        Self(self.0.midpoint(other.0), self.1.midpoint(other.1))
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data: Vec<Point> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut options: Vec<(Point, Point)> = data
        .iter()
        .enumerate()
        .flat_map(|(i, Point(x1, y1))| {
            data.iter()
                .skip(i + 1)
                .map(|Point(x2, y2)| (Point::new(*x1, *y1), Point::new(*x2, *y2)))
        })
        .collect();

    options.sort_by_key(|v| area(*v));
    options.reverse();

    let lines: Vec<(Point, Point)> = data
        .iter()
        .cloned()
        .zip(data.iter().cloned().cycle().skip(1))
        .collect();

    let real_max = options
        .iter()
        .find(|(p1, p2)| {
            // println!("checking: {p1:?},{p2:?}");
            let pmin = p1.min(p2);
            let pmax = p1.max(p2);

            let internal_crossings = lines.iter().any(|(start, end)| {
                let lmin = start.min(end);
                let lmax = start.max(end);
                // println!("\tlmin {lmin:?}, lmax {lmax:?}");

                let x_in = lmin.0 < pmax.0 && lmax.0 > pmin.0;
                let y_in = lmin.1 < pmax.1 && lmax.1 > pmin.1;
                // println!("\tx_in {x_in:?}, y_in {y_in:?}");
                x_in && y_in
            });

            if internal_crossings {
                return false;
            }

            // Count crossings via ray tracing from middle to the right
            let mid = p1.mid(p2);
            let transitions = lines
                .iter()
                .filter(|(start, end)| {
                    if start.1 == end.1 {
                        // Horizontal line
                        return false;
                    }

                    let lmin = start.min(end);
                    let lmax = start.max(end);

                    start.0 > mid.0 && lmin.1 <= mid.1 && lmax.1 >= mid.1
                })
                .count();

            // println!("transitions: {transitions}");
            (transitions % 2) == 1
        })
        .unwrap();

    println!(
        "real_max ({:?}),({:?}) {}",
        real_max.0,
        real_max.1,
        area(*real_max)
    );
}
