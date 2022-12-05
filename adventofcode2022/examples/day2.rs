use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("./examples/day2_input.txt")?;
    let sol = parse(&input)?;
    let res = sol.resolve_part1()?;
    println!("res={}", res);
    Ok(())
}

struct Solution(Vec<Round>);

impl Solution {
    fn resolve_part1(&self) -> anyhow::Result<i32> {
        let res = self.0.iter().map(|x| x.score().1).sum();
        Ok(res)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Rock,
    Paper,
    Scissor,
}

impl Element {
    fn score(&self) -> i32 {
        match self {
            Element::Rock => 1,
            Element::Paper => 2,
            Element::Scissor => 3,
        }
    }

    fn cmp(&self, other: &Self) -> i32 {
        if self == other {
            return 0;
        }
        match (self, other) {
            (Element::Rock, Element::Paper) => 1,
            (Element::Paper, Element::Scissor) => 1,
            (Element::Scissor, Element::Rock) => 1,
            _ => -1,
        }
    }
}

impl FromStr for Element {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "A" | "X" => Element::Rock,
            "B" | "Y" => Element::Paper,
            "C" | "Z" => Element::Scissor,
            _ => unreachable!(),
        };
        Ok(res)
    }
}

struct Round(Element, Element);

impl Round {
    fn score(&self) -> (i32, i32) {
        let v = self.0.cmp(&self.1);
        let (a, b) = match v {
            0 => (3, 3),
            -1 => (6, 0),
            1 => (0, 6),
            _ => unreachable!(),
        };
        (self.0.score() + a, self.1.score() + b)
    }
}

fn parse(s: &str) -> anyhow::Result<Solution> {
    let res = s.lines().map(|line| {
        let a = line[0..1].parse().unwrap();
        let b = line[2..3].parse().unwrap();
        Round(a, b)
    });
    Ok(Solution(res.collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_part1() {
        let input = "A Y
B X
C Z";
        let sol = parse(input).unwrap();
        let res = sol.resolve_part1().unwrap();
        assert_eq!(res, 15);
    }
}
