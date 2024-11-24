#[derive(Debug)]
struct Number {
    value: usize,
    span: Span,
}

#[derive(Debug)]
struct Symbol {
    loc: usize,
    value: char,
}

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct ParsedLine {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Default for ParsedLine {
    fn default() -> Self {
        Self {
            numbers: vec![],
            symbols: vec![],
        }
    }
}

fn parse_number(n: &str) -> usize {
    n.parse().unwrap()
}

fn parse(lines: &[String]) -> Vec<ParsedLine> {
    let mut parsed_lines = vec![];
    for line in lines {
        let mut parsed_line = ParsedLine {
            numbers: vec![],
            symbols: vec![],
        };

        let mut values = String::new();
        for (line_offset, c) in line.chars().enumerate() {
            match c {
                c if c.is_numeric() => {
                    values.push(c);
                }
                '.' => {
                    if !values.is_empty() {
                        parsed_line.numbers.push(Number {
                            value: parse_number(&values),
                            span: Span {
                                start: line_offset - values.len(),
                                end: line_offset - 1,
                            },
                        });
                        values.clear();
                    }
                }
                c => {
                    parsed_line.symbols.push(Symbol {
                        loc: line_offset,
                        value: c,
                    });
                    if !values.is_empty() {
                        parsed_line.numbers.push(Number {
                            value: parse_number(&values),
                            span: Span {
                                start: line_offset - values.len(),
                                end: line_offset - 1,
                            },
                        });
                        values.clear();
                    }
                }
            }
        }
        if !values.is_empty() {
            parsed_line.numbers.push(Number {
                value: parse_number(&values),
                span: Span {
                    start: line.len() - values.len(),
                    end: line.len() - 1,
                },
            });
            values.clear();
        }
        parsed_lines.push(parsed_line);
    }
    parsed_lines
}

fn reconstruct(parsed: &[ParsedLine]) -> Vec<String> {
    let mut result = vec![];
    const MAX: usize = 140;
    for line in parsed {
        let mut idx = 0;
        let mut reconstructed_line = String::new();
        loop {
            if let Some(s) = line.symbols.iter().find(|s| s.loc == idx) {
                reconstructed_line.push(s.value);
                idx += 1;
            } else if let Some(n) = line.numbers.iter().find(|n| n.span.start == idx) {
                reconstructed_line.push_str(&format!("{}", n.value));
                idx = n.span.end + 1;
            } else {
                reconstructed_line.push('.');
                idx += 1;
            }

            if idx >= MAX {
                break;
            }
        }
        result.push(reconstructed_line);
    }
    result
}

fn are_adjacent(n: &Number, s: &Symbol) -> bool {
    let min = n.span.start.max(1) - 1;
    if s.loc >= min && s.loc <= n.span.end + 1 {
        return true;
    }
    false
}

fn has_adjacent_symbol(
    number: &Number,
    prev_line: &ParsedLine,
    cur_line: &ParsedLine,
    next_line: &ParsedLine,
) -> bool {
    for s in prev_line
        .symbols
        .iter()
        .chain(cur_line.symbols.iter())
        .chain(next_line.symbols.iter())
    {
        if are_adjacent(number, s) {
            return true;
        }
    }
    false
}

fn find_adjacent_numbers(
    s: &Symbol,
    prev_line: &ParsedLine,
    cur_line: &ParsedLine,
    next_line: &ParsedLine,
) -> Option<(usize, usize)> {
    let mut nums = vec![];

    for n in prev_line
        .numbers
        .iter()
        .chain(cur_line.numbers.iter())
        .chain(next_line.numbers.iter())
    {
        if are_adjacent(n, s) {
            nums.push(n);
        }
    }

    assert!(nums.len() <= 2);
    if nums.len() == 2 {
        Some((nums[0].value, nums[1].value))
    } else {
        None
    }
}

fn part1(lines: &Vec<ParsedLine>) {
    let mut sum = 0;

    let empty = ParsedLine::default();
    for (idx, line) in lines.iter().enumerate() {
        let prev_line = if idx > 0 { &lines[idx - 1] } else { &empty };
        let next_line = if idx < lines.len() - 1 {
            &lines[idx + 1]
        } else {
            &empty
        };

        for number in &line.numbers {
            if has_adjacent_symbol(number, prev_line, line, next_line) {
                sum += number.value;
            }
        }
    }

    println!("Sum of adjacent nums is {sum}")
}

fn part2(lines: &Vec<ParsedLine>) {
    let mut sum = 0;

    let empty = ParsedLine::default();
    for (idx, line) in lines.iter().enumerate() {
        let prev_line = if idx > 0 { &lines[idx - 1] } else { &empty };
        let next_line = if idx < lines.len() - 1 {
            &lines[idx + 1]
        } else {
            &empty
        };

        for s in &line.symbols {
            if s.value == '*' {
                if let Some((a, b)) = find_adjacent_numbers(s, prev_line, line, next_line) {
                    sum += a * b;
                }
            }
        }
    }

    println!("Sum of gears is {sum}")
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|l| l.unwrap()).collect();

    let parsed = parse(&lines);

    let reconstructed = reconstruct(&parsed);
    for line in reconstructed {
        println!("{line}");
    }

    part1(&parsed);
    part2(&parsed);
}
