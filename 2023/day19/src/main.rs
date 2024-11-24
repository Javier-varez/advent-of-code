use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Var {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Condition {
    Less,
    Greater,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum RuleResult {
    Accepted,
    Rejected,
    Workflow(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule {
    condition: Option<(Var, Condition, usize)>,
    result: RuleResult,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Workflow(Vec<Rule>);

#[derive(Debug, Clone, Eq, PartialEq)]
enum WorkflowResult {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Item {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Item {
    fn get_var(&self, var: Var) -> usize {
        match var {
            Var::X => self.x,
            Var::M => self.m,
            Var::A => self.a,
            Var::S => self.s,
        }
    }
}

fn parse_condition(cond: &str) -> (Var, Condition, usize) {
    let mut iter = cond.chars();
    let var = match iter.next().unwrap() {
        'x' => Var::X,
        'm' => Var::M,
        'a' => Var::A,
        's' => Var::S,
        _ => todo!(),
    };

    let cond = match iter.next().unwrap() {
        '<' => Condition::Less,
        '>' => Condition::Greater,
        _ => todo!(),
    };

    (var, cond, iter.collect::<String>().parse().unwrap())
}

fn parse_result(cond: &str) -> RuleResult {
    match cond {
        "A" => RuleResult::Accepted,
        "R" => RuleResult::Rejected,
        w => RuleResult::Workflow(w.to_owned()),
    }
}

fn parse_workflow(line: &str) -> (String, Workflow) {
    let mut iter = line.split("{");
    let name = iter.next().unwrap();
    let rules = iter.next().unwrap().trim_end_matches("}");
    let w = Workflow(
        rules
            .split(",")
            .map(|rule| {
                if rule.contains(":") {
                    let mut iter = rule.split(":");
                    let condition = Some(parse_condition(iter.next().unwrap()));
                    let result = parse_result(iter.next().unwrap());
                    Rule { condition, result }
                } else {
                    let result = parse_result(rule);
                    Rule {
                        condition: None,
                        result,
                    }
                }
            })
            .collect(),
    );

    (name.to_owned(), w)
}

fn parse_item(line: &str) -> Item {
    let values: Vec<usize> = line
        .trim_start_matches("{")
        .trim_end_matches("}")
        .split(",")
        .map(|e| {
            let mut iter = e.split("=");
            let _ = iter.next();
            iter.next().unwrap().parse().unwrap()
        })
        .collect();

    Item {
        x: values[0],
        m: values[1],
        a: values[2],
        s: values[3],
    }
}

fn apply_workflows(part: &Item, workflows: &HashMap<String, Workflow>) -> WorkflowResult {
    let mut c = "in";

    loop {
        let w = &workflows[c];
        for rule in &w.0 {
            let apply_result = if let Some(cond) = rule.condition {
                match cond.1 {
                    Condition::Less => part.get_var(cond.0) < cond.2,
                    Condition::Greater => part.get_var(cond.0) > cond.2,
                }
            } else {
                true
            };

            if apply_result {
                match &rule.result {
                    RuleResult::Accepted => return WorkflowResult::Accepted,
                    RuleResult::Rejected => return WorkflowResult::Rejected,
                    RuleResult::Workflow(next_w) => c = next_w,
                };
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ItemRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl ItemRange {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn combinations(&self) -> usize {
        let vars = [Var::A, Var::X, Var::M, Var::S];
        vars.iter()
            .map(|var| {
                let (min, max) = self.get_var_range(*var);
                assert!(min <= max);
                max - min + 1
            })
            .product()
    }

    fn get_var_range(&self, var: Var) -> (usize, usize) {
        match var {
            Var::X => self.x,
            Var::M => self.m,
            Var::A => self.a,
            Var::S => self.s,
        }
    }

    fn set_var_range(&mut self, var: Var, range: (usize, usize)) {
        match var {
            Var::X => self.x = range,
            Var::M => self.m = range,
            Var::A => self.a = range,
            Var::S => self.s = range,
        };
    }

    fn with_constraint(&self, var: Var, cond: Condition, threshold: usize) -> Option<ItemRange> {
        let (mut min, mut max) = self.get_var_range(var);
        match cond {
            Condition::Greater => {
                if threshold >= max {
                    return None;
                }
                min = min.max(threshold + 1);
            }
            Condition::Less => {
                if threshold <= min {
                    return None;
                }
                max = max.min(threshold - 1);
            }
        }

        let mut cloned = self.clone();
        cloned.set_var_range(var, (min, max));
        Some(cloned)
    }

    fn with_opposite_constraint(
        &self,
        var: Var,
        cond: Condition,
        threshold: usize,
    ) -> Option<ItemRange> {
        let (mut min, mut max) = self.get_var_range(var);
        match cond {
            Condition::Greater => {
                // less or eq to thres
                if threshold < min {
                    return None;
                }
                max = max.min(threshold);
            }
            Condition::Less => {
                if threshold > max {
                    return None;
                }
                min = min.max(threshold);
            }
        }

        let mut cloned = self.clone();
        cloned.set_var_range(var, (min, max));
        Some(cloned)
    }
}

fn discover_combinations(workflows: &HashMap<String, Workflow>) -> Vec<ItemRange> {
    fn run_wflow_impl(
        mut cur: ItemRange,
        workflow: &str,
        workflows: &HashMap<String, Workflow>,
        ranges: &mut Vec<ItemRange>,
    ) {
        let workflow = &workflows[workflow];
        for rule in &workflow.0 {
            if let Some((var, cond, threshold)) = rule.condition {
                if let Some(new_range) = cur.with_constraint(var, cond, threshold) {
                    match &rule.result {
                        RuleResult::Workflow(other) => {
                            run_wflow_impl(new_range, other, workflows, ranges)
                        }
                        RuleResult::Accepted => ranges.push(new_range),
                        RuleResult::Rejected => {}
                    }
                }
                if let Some(new_range) = cur.with_opposite_constraint(var, cond, threshold) {
                    cur = new_range;
                }
            } else {
                match &rule.result {
                    RuleResult::Workflow(other) => {
                        run_wflow_impl(cur.clone(), other, workflows, ranges)
                    }
                    RuleResult::Accepted => ranges.push(cur.clone()),
                    RuleResult::Rejected => {}
                }
            }
        }
    }

    let mut ranges = vec![];
    run_wflow_impl(ItemRange::new(), "in", workflows, &mut ranges);
    ranges
}

fn main() {
    let mut workflows = HashMap::new();
    let mut items = vec![];
    let mut found_newline = false;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            found_newline = true;
            continue;
        }

        if !found_newline {
            let (name, w) = parse_workflow(&line);
            workflows.insert(name, w);
        } else {
            let item = parse_item(&line);
            items.push(item);
        }
    }

    let mut count = 0;
    for i in &items {
        match apply_workflows(&i, &workflows) {
            WorkflowResult::Accepted => {
                count += i.a + i.m + i.x + i.s;
            }
            WorkflowResult::Rejected => {}
        }
    }
    println!("Count is {count}");

    let ranges = discover_combinations(&workflows);
    let combinations: usize = ranges.iter().map(|r| r.combinations()).sum();
    println!("Combinations: {combinations:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_range_greater() {
        let range = ItemRange::new()
            .with_constraint(Var::A, Condition::Greater, 2000)
            .unwrap();
        assert_eq!(
            range,
            ItemRange {
                x: (1, 4000),
                m: (1, 4000),
                a: (2001, 4000),
                s: (1, 4000),
            }
        );

        let range = range
            .with_constraint(Var::A, Condition::Greater, 100)
            .unwrap();

        assert_eq!(
            range,
            ItemRange {
                x: (1, 4000),
                m: (1, 4000),
                a: (2001, 4000),
                s: (1, 4000),
            }
        );

        let range = range.with_constraint(Var::X, Condition::Less, 100).unwrap();
        assert_eq!(
            range,
            ItemRange {
                x: (1, 99),
                m: (1, 4000),
                a: (2001, 4000),
                s: (1, 4000),
            }
        );

        assert!(range
            .with_constraint(Var::X, Condition::Greater, 99)
            .is_none());

        let range = range
            .with_opposite_constraint(Var::X, Condition::Greater, 100)
            .unwrap();
        assert_eq!(
            range,
            ItemRange {
                x: (1, 99),
                m: (1, 4000),
                a: (2001, 4000),
                s: (1, 4000),
            }
        );

        let range = range
            .with_opposite_constraint(Var::X, Condition::Greater, 99)
            .unwrap();
        assert_eq!(
            range,
            ItemRange {
                x: (1, 99),
                m: (1, 4000),
                a: (2001, 4000),
                s: (1, 4000),
            }
        );

        let range = range
            .with_opposite_constraint(Var::X, Condition::Greater, 98)
            .unwrap();
        assert_eq!(
            range,
            ItemRange {
                x: (1, 98),
                m: (1, 4000),
                a: (2001, 4000),
                s: (1, 4000),
            }
        );

        let range = range
            .with_opposite_constraint(Var::X, Condition::Less, 2)
            .unwrap();
        assert_eq!(
            range,
            ItemRange {
                x: (2, 98),
                m: (1, 4000),
                a: (2001, 4000),
                s: (1, 4000),
            }
        );
    }
}
