use std::collections::HashMap;

fn find_paths(node: &str, map: &HashMap<String, Vec<String>>) -> usize {
    if node == "out" {
        return 1;
    }

    map[node]
        .iter()
        .map(|new_node| find_paths(new_node, map))
        .sum()
}

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let filename = std::env::args().nth(1).unwrap();
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .for_each(|l| {
            let (src, d) = l.split_once(":").unwrap();
            let dest = d.split_whitespace().map(|v| v.to_string()).collect();
            map.insert(src.to_string(), dest);
        });

    let paths = find_paths("you", &map);

    println!("p1 {paths}");
}
