use std::io::Read;

type Label = String;
type FocalLength = usize;
type Box = Vec<(Label, FocalLength)>;

struct Boxes(Vec<Box>);

impl Boxes {
    fn new() -> Self {
        Self(vec![vec![]; 256])
    }

    fn get_mut(&mut self, idx: usize) -> &mut Box {
        &mut self.0[idx]
    }
}

fn hash(step: &str) -> usize {
    step.bytes()
        .fold(0usize, |acc, value| ((acc + value as usize) * 17) % 256)
}

fn handle_instruction(inst: &str, boxes: &mut Boxes) {
    if inst.contains("-") {
        let label = inst.split("-").next().unwrap();
        let box_idx = hash(&label);
        let list = boxes.get_mut(box_idx);

        if let Some(i) = list
            .iter()
            .enumerate()
            .find_map(|(i, (l, _fl))| if l == label { Some(i) } else { None })
        {
            list.remove(i);
        }
    } else {
        let label = inst.split("=").next().unwrap();
        let new_fl = inst.split("=").skip(1).next().unwrap().parse().unwrap();
        let box_idx = hash(&label);
        let list = boxes.get_mut(box_idx);

        if let Some((_l, fl)) = list.iter_mut().find(|(l, _fl)| l == label) {
            *fl = new_fl;
        } else {
            list.push((label.to_string(), new_fl));
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let boxes = input
        .trim()
        .split(",")
        .fold(Boxes::new(), |mut boxes, inst| {
            handle_instruction(inst, &mut boxes);
            boxes
        });

    let count = boxes.0.iter().enumerate().fold(0, |acc, (box_idx, b)| {
        b.iter().enumerate().fold(acc, |acc, (slot_idx, (_l, fl))| {
            acc + (box_idx + 1) * (slot_idx + 1) * fl
        })
    });

    println!("Hi! {count}");
}
