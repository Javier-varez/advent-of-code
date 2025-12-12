use good_lp::{default_solver, variable, Expression, ProblemVariables, Solution, SolverModel};

#[derive(Debug)]
struct Button(u64, Vec<usize>);

impl Button {
    fn parse(data: &str) -> Self {
        let iter = data
            .trim_matches(['(', ')'])
            .split(',')
            .map(|idx| idx.parse::<usize>().unwrap());

        let mask = iter.clone().fold(0, |result, idx| {
            let mask = 1 << idx;
            mask | result
        });
        let indices = iter.collect();
        Self(mask, indices)
    }

    fn mask(&self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
struct Leds(u64);

impl Leds {
    fn parse(data: &str) -> Self {
        let data = data.trim_matches(['[', ']']);
        Self(data.chars().enumerate().fold(0, |result, (idx, v)| {
            if v == '#' {
                result | (1 << idx)
            } else {
                result
            }
        }))
    }

    fn mask(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone)]
struct Joltage(Vec<usize>);

impl Joltage {
    fn parse(data: &str) -> Self {
        let joltage = data
            .trim_matches(['{', '}'])
            .split(',')
            .map(|idx| idx.parse::<usize>().unwrap())
            .collect();

        Self(joltage)
    }
}

#[derive(Debug)]
struct Machine {
    leds: Leds,
    buttons: Vec<Button>,
    req_joltage: Joltage,
}

fn min_presses_impl(m: &Machine, cur: u64, btns: &[Button]) -> Option<usize> {
    btns.iter()
        .enumerate()
        .filter_map(|(i, btn)| {
            let new = cur ^ btn.mask();
            if new == m.leds.mask() {
                return Some(1);
            }
            let rem = &btns[i + 1..];
            if rem.is_empty() {
                return None;
            }

            min_presses_impl(m, new, rem).map(|v| v + 1)
        })
        .min()
}

fn min_presses(m: &Machine) -> usize {
    min_presses_impl(m, 0, &m.buttons).unwrap()
}

fn min_joltage_but_fast(m: &Machine) -> usize {
    let mut problem_vars = ProblemVariables::new();

    let btn_vars: Vec<_> = m
        .buttons
        .iter()
        .map(|_| problem_vars.add(variable().integer().min(0)))
        .collect();

    let objective = btn_vars
        .iter()
        .fold(Expression::with_capacity(0), |mut expr, var| {
            expr.add_mul(1, var);
            expr
        });

    let joltage_constraints: Vec<_> = m
        .req_joltage
        .0
        .iter()
        .enumerate()
        .map(|(joltage_idx, req_joltage)| {
            let contrib_buttons = m
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(btn_idx, b)| {
                    b.1.iter()
                        .find(|e| **e == joltage_idx)
                        .map(|_| btn_vars[btn_idx])
                })
                .fold(Expression::with_capacity(0), |mut expr, var| {
                    expr.add_mul(1, var);
                    expr
                })
                .eq(Expression::from_other_affine(*req_joltage as i32));
            contrib_buttons
        })
        .collect();

    let solution = problem_vars
        .minimise(objective)
        .using(default_solver)
        .with_all(joltage_constraints)
        .solve()
        .unwrap();

    let presses: Vec<usize> = btn_vars
        .iter()
        .map(|v| solution.value(*v) as usize)
        .collect();

    // Validate the result
    let size = m.req_joltage.0.len();
    let result_joltage = presses
        .iter()
        .zip(m.buttons.iter())
        .map(|(num_p, btn)| {
            let mut add_joltage = vec![0; size];
            for j in btn.1.iter() {
                add_joltage[*j] = *num_p;
            }
            add_joltage
        })
        .fold(vec![0; size], |mut res, cur| {
            res.iter_mut().zip(cur.iter()).for_each(|(r, v)| {
                *r += *v;
            });
            res
        });

    assert_eq!(result_joltage, m.req_joltage.0);

    presses.into_iter().sum()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data: Vec<Machine> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let leds = Leds::parse(parts.next().unwrap());
            let mut buttons = vec![];

            let mut req_joltage = None;
            for part in parts {
                if part.starts_with("{") {
                    req_joltage = Some(Joltage::parse(part));
                    break;
                }

                let btn = Button::parse(part);
                buttons.push(btn);
            }
            Machine {
                leds,
                buttons,
                req_joltage: req_joltage.unwrap(),
            }
        })
        .collect();

    let result: usize = data.iter().map(min_presses).sum();
    println!("p1: {result}");

    let result: usize = data.iter().map(min_joltage_but_fast).sum();
    println!("p2: {result}");
}
