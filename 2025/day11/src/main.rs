use std::collections::HashMap;

fn find_paths(
    node: &str,
    dest: &str,
    fft: bool,
    dac: bool,
    map: &HashMap<String, Vec<String>>,
    mem: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if node == dest {
        return if fft && dac { 1 } else { 0 };
    }

    if let Some(res) = mem.get(&(node.to_string(), fft, dac)) {
        return *res;
    }

    let fft = fft | (node == "fft");
    let dac = dac | (node == "dac");

    let result = map[node]
        .iter()
        .map(|new_node| find_paths(new_node, dest, fft, dac, map, mem))
        .sum();
    mem.insert((node.to_string(), fft, dac), result);

    result
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

    let mut mem = HashMap::new();
    let svr_to_out = find_paths("svr", "out", false, false, &map, &mut mem);
    println!("svr_to_out = {svr_to_out}");
}
