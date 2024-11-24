use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref RANKS: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('A', 12);
        m.insert('K', 11);
        m.insert('Q', 10);
        m.insert('T', 9);
        m.insert('9', 8);
        m.insert('8', 7);
        m.insert('7', 6);
        m.insert('6', 5);
        m.insert('5', 4);
        m.insert('4', 3);
        m.insert('3', 2);
        m.insert('2', 1);
        m.insert('J', 0);
        m
    };
}

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

impl<'a> TryFrom<&'a str> for Hand {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            anyhow::bail!("Expected a hand to have 5 characters: {value}");
        }

        let map = value.chars().fold(HashMap::new(), |mut m, c| {
            let n = m.get_key_value(&c).map(|(_, n)| *n).unwrap_or(0);
            m.insert(c, n + 1);
            m
        });

        // if only jokers are present, return five of kind
        if value == "JJJJJ" {
            return Ok(Self::FiveOfAKind(value.to_string()));
        }

        // Max non-joker count
        let max_count = map
            .iter()
            .map(|(k, v)| if *k == 'J' { 0 } else { *v })
            .max()
            .unwrap();

        assert_ne!(max_count, 0);

        let num_jokers = map
            .iter()
            .find(|(k, _)| **k == 'J')
            .map(|(_, v)| *v)
            .unwrap_or(0);

        let s = value.to_string();

        let compensated_len = if num_jokers != 0 {
            map.len() - 1
        } else {
            map.len()
        };

        Ok(match max_count + num_jokers {
            5 => Self::FiveOfAKind(s),
            4 => Self::FourOfAKind(s),
            3 if (compensated_len == 2) => Self::FullHouse(s),
            3 => Self::ThreeOfAKind(s),
            2 if (compensated_len == 3) => Self::TwoPair(s),
            2 => Self::OnePair(s),
            1 => Self::HighCard(s),
            _ => {
                anyhow::bail!("Unexpected number of repeated cards: {max_count}");
            }
        })
    }
}

pub fn compare_hands_of_same_type(left: &str, right: &str) -> std::cmp::Ordering {
    left.chars()
        .zip(right.chars())
        .fold(std::cmp::Ordering::Equal, |o, (l, r)| match o {
            std::cmp::Ordering::Equal if RANKS[&l] > RANKS[&r] => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal if RANKS[&l] < RANKS[&r] => std::cmp::Ordering::Less,
            o => o,
        })
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (&self, other) {
            (Self::FiveOfAKind(s1), Self::FiveOfAKind(s2)) => compare_hands_of_same_type(s1, s2),
            (Self::FiveOfAKind(_), _) => std::cmp::Ordering::Greater,
            (_, Self::FiveOfAKind(_)) => std::cmp::Ordering::Less,
            (Self::FourOfAKind(s1), Self::FourOfAKind(s2)) => compare_hands_of_same_type(s1, s2),
            (Self::FourOfAKind(_), _) => std::cmp::Ordering::Greater,
            (_, Self::FourOfAKind(_)) => std::cmp::Ordering::Less,
            (Self::FullHouse(s1), Self::FullHouse(s2)) => compare_hands_of_same_type(s1, s2),
            (Self::FullHouse(_), _) => std::cmp::Ordering::Greater,
            (_, Self::FullHouse(_)) => std::cmp::Ordering::Less,
            (Self::ThreeOfAKind(s1), Self::ThreeOfAKind(s2)) => compare_hands_of_same_type(s1, s2),
            (Self::ThreeOfAKind(_), _) => std::cmp::Ordering::Greater,
            (_, Self::ThreeOfAKind(_)) => std::cmp::Ordering::Less,
            (Self::TwoPair(s1), Self::TwoPair(s2)) => compare_hands_of_same_type(s1, s2),
            (Self::TwoPair(_), _) => std::cmp::Ordering::Greater,
            (_, Self::TwoPair(_)) => std::cmp::Ordering::Less,
            (Self::OnePair(s1), Self::OnePair(s2)) => compare_hands_of_same_type(s1, s2),
            (Self::OnePair(_), _) => std::cmp::Ordering::Greater,
            (_, Self::OnePair(_)) => std::cmp::Ordering::Less,
            (Self::HighCard(s1), Self::HighCard(s2)) => compare_hands_of_same_type(s1, s2),
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct Bet {
    hand: Hand,
    bet: usize,
}

fn main() -> anyhow::Result<()> {
    let mut bets: Vec<Bet> = std::io::stdin()
        .lines()
        .map(|l| {
            l.map_err(|e| anyhow::Error::from(e)).and_then(|l| {
                let mut iter = l.split_whitespace();
                let hand = iter
                    .next()
                    .ok_or(anyhow::anyhow!("hand not found!"))?
                    .try_into()?;
                let bet = iter
                    .next()
                    .ok_or(anyhow::anyhow!("bet not found!"))?
                    .parse()?;
                Ok(Bet { hand, bet })
            })
        })
        .collect::<anyhow::Result<Vec<Bet>>>()?;

    bets.sort_by(|b1, b2| b1.hand.cmp(&b2.hand));

    let sum = bets
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, bet)| acc + (idx + 1) * bet.bet);
    println!("sum: {sum}");

    Ok(())
}
