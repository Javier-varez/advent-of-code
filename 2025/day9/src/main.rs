fn area(((x1, y1), (x2, y2)): ((usize, usize), (usize, usize))) -> usize {
    let width = ((x1 as isize) - (x2 as isize)).abs() + 1;
    let height = ((y1 as isize) - (y2 as isize)).abs() + 1;
    (width * height) as usize
}

fn inside_perimeter(
    lines: &[((usize, usize), (usize, usize))],
    range_x: (usize, usize),
    range_y: (usize, usize),
    rect_x: (usize, usize),
    rect_y: (usize, usize),
    point: (usize, usize),
    direction: (isize, isize),
) -> bool {
    if point.0 < range_x.0 || point.0 > range_x.1 {
        return false;
    }
    if point.1 < range_y.0 || point.1 > range_y.1 {
        return false;
    }

    let in_perim = lines.iter().any(|(start, end)| {
        (point.0 >= (start.0.min(end.0)))
            && (point.0 <= (start.0.max(end.0)))
            && (point.1 >= (start.1.min(end.1)))
            && (point.1 <= (start.1.max(end.1)))
    });

    if in_perim {
        let inside_rect =
            point.0 > rect_x.0 && point.0 < rect_x.1 && point.1 > rect_y.0 && point.1 < rect_y.1;
        return !inside_rect;
    }

    // Move in the direction
    let point = (
        (point.0 as isize + direction.0) as usize,
        (point.1 as isize + direction.1) as usize,
    );
    inside_perimeter(lines, range_x, range_y, rect_x, rect_y, point, direction)
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data: Vec<(usize, usize)> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let max_x = data.iter().map(|(x, _)| *x).max().unwrap();
    let min_x = data.iter().map(|(x, _)| *x).min().unwrap();
    let max_y = data.iter().map(|(_, y)| *y).max().unwrap();
    let min_y = data.iter().map(|(_, y)| *y).min().unwrap();

    let range_x = (min_x, max_x);
    let range_y = (min_y, max_y);

    let mut options: Vec<((usize, usize), (usize, usize))> = data
        .iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            data.iter()
                .skip(i + 1)
                .map(|(x2, y2)| ((*x1, *y1), (*x2, *y2)))
        })
        .collect();

    options.sort_by_key(|v| area(*v));
    options.reverse();

    let lines: Vec<((usize, usize), (usize, usize))> = data
        .iter()
        .cloned()
        .zip(data.iter().cloned().cycle().skip(1))
        .collect();

    let real_max = options
        .iter()
        .find(|((x1, y1), (x2, y2))| {
            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);
            let any_points_inside = data.iter().any(|(x3, y3)| {
                let x_in = x3 > xmin && x3 < xmax;
                let y_in = y3 > ymin && y3 < ymax;
                x_in && y_in
            });

            if any_points_inside {
                return false;
            }

            // Check that the middle point is enclosed by the perimeter
            let x_mid = xmin.midpoint(*xmax);
            let y_mid = ymin.midpoint(*ymax);

            let rect_x = (*xmin, *xmax);
            let rect_y = (*ymin, *ymax);
            let res_x_neg = inside_perimeter(
                &lines,
                range_x,
                range_y,
                rect_x,
                rect_y,
                (x_mid, y_mid),
                (-1, 0),
            );
            let res_x_pos = inside_perimeter(
                &lines,
                range_x,
                range_y,
                rect_x,
                rect_y,
                (x_mid, y_mid),
                (1, 0),
            );
            let res_y_neg = inside_perimeter(
                &lines,
                range_x,
                range_y,
                rect_x,
                rect_y,
                (x_mid, y_mid),
                (0, -1),
            );
            let res_y_pos = inside_perimeter(
                &lines,
                range_x,
                range_y,
                rect_x,
                rect_y,
                (x_mid, y_mid),
                (0, 1),
            );
            res_x_neg && res_x_pos && res_y_neg && res_y_pos
        })
        .unwrap();

    println!(
        "real_max ({:?}),({:?}) {}",
        real_max.0,
        real_max.1,
        area(*real_max)
    );
}
