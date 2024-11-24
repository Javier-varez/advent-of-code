#[derive(Clone)]
struct Range {
    src_start: usize,
    dst_start: usize,
    len: usize,
}

#[derive(Clone)]
struct Map {
    from: String,
    to: String,
    t: Vec<Range>,
}

impl Map {
    fn translate(&self, src: usize) -> usize {
        for range in &self.t {
            if src >= range.src_start && src < (range.src_start + range.len) {
                return src - range.src_start + range.dst_start;
            }
        }
        src
    }
}

struct Maps(Vec<Map>);

fn parse<T: Iterator<Item = String>>(mut iter: T) -> (Vec<usize>, Maps) {
    let seeds_txt = iter.next().unwrap();
    let seeds: Vec<usize> = seeds_txt
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let seeds: Vec<usize> = {
        seeds
            .iter()
            .step_by(2)
            .cloned()
            .zip(seeds.iter().skip(1).step_by(2).cloned())
            .map(|(start, len)| {
                println!("start {start}, len {len}");
                assert!(len > 0);
                std::iter::successors(Some(start), move |p| {
                    let n = *p + 1;
                    if n < start + len {
                        Some(n)
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    };

    let mut maps = Maps(vec![]);

    while let Some(line) = &iter.next() {
        if line.is_empty() {
            continue;
        }

        let (from, to) = {
            let mut components = line
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .split("-")
                .map(|s| s.to_owned());
            let from = components.next().unwrap();
            let _ = components.next().unwrap();
            let to = components.next().unwrap();
            (from, to)
        };

        let mut map = Map {
            from,
            to,
            t: vec![],
        };

        loop {
            let line = iter.next();
            if line.is_none() || line.as_ref().unwrap().is_empty() {
                break;
            }

            let r = {
                let mut components = line
                    .as_ref()
                    .unwrap()
                    .split_whitespace()
                    .map(|v| v.parse().unwrap());
                let dst = components.next().unwrap();
                let src = components.next().unwrap();
                let len = components.next().unwrap();
                Range {
                    src_start: src,
                    dst_start: dst,
                    len,
                }
            };

            map.t.push(r);
        }
        maps.0.push(map);
    }
    (seeds, maps)
}

fn main() {
    let (seeds, maps) = parse(std::io::stdin().lines().map(|l| l.unwrap()));

    let transforms = [
        ("seed", "soil"),
        ("soil", "fertilizer"),
        ("fertilizer", "water"),
        ("water", "light"),
        ("light", "temperature"),
        ("temperature", "humidity"),
        ("humidity", "location"),
    ];

    let transforms = transforms.map(|(from, to)| {
        maps.0
            .iter()
            .find(|m| m.to == to && m.from == from)
            .cloned()
            .unwrap()
    });

    let min = seeds
        .iter()
        .map(|seed| transforms.iter().fold(*seed, |i, m| m.translate(i)))
        .min()
        .unwrap();
    println!("min is {min}");
}
