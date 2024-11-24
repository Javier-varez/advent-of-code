#[derive(Debug)]
struct Stage {
    red: usize,
    green: usize,
    blue: usize,
}

impl Stage {
    fn is_possible(&self, c: &Constraints) -> bool{
        self.red <= c.red && self.green <= c.green && self.blue <= c.blue
    }
}

#[derive(Debug)]
struct Game {
    game: usize,
    stages: Vec<Stage>,
}

impl Game {
    fn is_possible(&self, c: &Constraints) -> bool{
        for stage in &self.stages {
            if !stage.is_possible(c) {
                return false;
            }
        }
        true
    }

    fn calculate_power(&self) -> usize{
        let red = self.stages.iter().map(|s| s.red).max().unwrap();
        let green = self.stages.iter().map(|s| s.green).max().unwrap();
        let blue = self.stages.iter().map(|s| s.blue).max().unwrap();

        red * green * blue
    }
}

fn parse_stage(stage: &str) -> Stage {
    let color_iter = stage.split(",");
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for colortxt in color_iter {
        let colortxt = colortxt.trim();
        if colortxt.is_empty() {
            continue;
        }

        let mut split = colortxt.split_whitespace();
        let value: usize = split.next().unwrap().parse().unwrap();
        let color_name = split.next().unwrap();
        match color_name {
            "blue" => {
                blue += value;
            }
            "green" => {
                green += value;
            }
            "red" => {
                red += value;
            }
            c => {
                panic!("Unexpected color! {c}");
            }
        }
    }

    Stage { red, green, blue }
}

fn parse_line(line: &str) -> Game {
    let mut colon_separated = line.split(":");
    let game: usize = colon_separated
        .next()
        .unwrap()
        .strip_prefix("Game ")
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let mut stages = vec![];

    let stages_text = colon_separated.next().unwrap();
    for stage in stages_text.split(";") {
        let stage = stage.trim();
        if !stage.is_empty() {
            stages.push(parse_stage(stage));
        }
    }
    Game { game, stages }
}

struct Constraints {
    red: usize,
    green: usize,
    blue: usize,
}

fn part1(games: &[Game]) {
    const C: Constraints = Constraints {
        red :12,
        green:13,
        blue: 14,
    };

    let mut count = 0;
    for game in games {
        if game.is_possible(&C) {
            count += game.game;
            println!("Game {} is possible", game.game);
        }
    }

    println!("count: {count}");
}

fn part2(games: &[Game]) {
    let mut total_power = 0;
    for game in games {
        let power = game.calculate_power();
        println!("game {}: {}", game.game, power);
        total_power += power;
    }
    println!("total power {total_power}");
}

fn main() {
    // Assume I/O does not fail for the purposes of this exercise
    let lines: Vec<_> = std::io::stdin().lines().map(|res| res.unwrap()).collect();

    let mut games = vec![];
    for line in lines {
        games.push(parse_line(&line));
    }

    part1(&games);
    part2(&games);
}
