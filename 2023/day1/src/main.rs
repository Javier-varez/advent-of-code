fn check_seq<T: std::iter::Iterator<Item = char> + Clone>(iter: &T, chars: &str) -> bool {
    let new_iter = (*iter).clone();
    new_iter.take(chars.len()).eq(chars.chars())
}

fn find_first(line: &str) -> usize {
    let mut iter = line.chars();
    loop {
        match iter.next().unwrap() {
            c if c.is_numeric() => {
                return (c as u8 - b'0') as usize;
            }
            'o' if check_seq(&iter, "ne") => {
                return 1;
            }
            't' if check_seq(&iter, "wo") => {
                return 2;
            }
            't' if check_seq(&iter, "hree") => {
                return 3;
            }
            'f' if check_seq(&iter, "our") => {
                return 4;
            }
            'f' if check_seq(&iter, "ive") => {
                return 5;
            }
            's' if check_seq(&iter, "ix") => {
                return 6;
            }
            's' if check_seq(&iter, "even") => {
                return 7;
            }
            'e' if check_seq(&iter, "ight") => {
                return 8;
            }
            'n' if check_seq(&iter, "ine") => {
                return 9;
            }
            _ => {}
        }
    }
}

fn find_last(line: &str) -> usize {
    let mut iter = line.chars().rev();
    loop {
        match iter.next().unwrap() {
            c if c.is_numeric() => {
                return (c as u8 - b'0') as usize;
            }
            'e' if check_seq(&iter, "no") => {
                return 1;
            }
            'o' if check_seq(&iter, "wt") => {
                return 2;
            }
            'e' if check_seq(&iter, "erht") => {
                return 3;
            }
            'r' if check_seq(&iter, "uof") => {
                return 4;
            }
            'e' if check_seq(&iter, "vif") => {
                return 5;
            }
            'x' if check_seq(&iter, "is") => {
                return 6;
            }
            'n' if check_seq(&iter, "eves") => {
                return 7;
            }
            't' if check_seq(&iter, "hgie") => {
                return 8;
            }
            'e' if check_seq(&iter, "nin") => {
                return 9;
            }
            _ => {}
        }
    }
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|l| l.unwrap()).collect();

    let mut acc = 0;
    for line in lines.iter() {
        let first = find_first(line);
        let last = find_last(line);
        let num = first * 10 + last;

        acc += num;
    }
    println!("result is {}", acc);
}
