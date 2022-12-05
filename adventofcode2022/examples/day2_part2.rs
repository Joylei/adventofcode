use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("./examples/day2_input.txt")?;
    let sol = parse(&input)?;
    let res = sol.resolve_part2()?;
    println!("res={}", res);
    Ok(())
}

struct Solution(Vec<Round>);

impl Solution {
    fn resolve_part2(&self) -> anyhow::Result<i32> {
        let res = self.0.iter().map(|x| x.score().1).sum();
        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn win(&self) -> Element {
        match self {
            Element::Rock => Element::Paper,
            Element::Paper => Element::Scissor,
            Element::Scissor => Element::Rock,
        }
    }

    fn lose(&self) -> Element {
        match self {
            Element::Rock => Element::Scissor,
            Element::Paper => Element::Rock,
            Element::Scissor => Element::Paper,
        }
    }

    fn cmp(&self, other: &Self) -> i32 {
        match other {
            Element::Paper => 0,
            Element::Rock => -1,
            Element::Scissor => 1,
        }
    }

    fn play(&self, cmp: i32) -> Element {
        match cmp {
            0 => *self,
            -1 => self.lose(),
            1 => self.win(),
            _ => unreachable!(),
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
        let other = self.0.play(v);
        //println!("{:?} {}", &other, v);
        (self.0.score() + a, other.score() + b)
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
    fn test_resolve_part2() {
        let input = "A Y
B X
C Z";
        let sol = parse(input).unwrap();
        let res = sol.resolve_part2().unwrap();
        assert_eq!(res, 12);
    }
}
