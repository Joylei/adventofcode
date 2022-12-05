use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("./examples/day4_input.txt")?;
    let sol = parse(&input)?;
    let res = sol.resolve_part1()?;
    println!("res={}", res);

    let res = sol.resolve_part2()?;
    println!("res={}", res);
    Ok(())
}

struct Solution(Vec<Round>);

impl Solution {
    fn resolve_part1(&self) -> anyhow::Result<usize> {
        let res = self.0.iter().filter(|x| x.is_full_contain()).count();
        Ok(res)
    }

    fn resolve_part2(&self) -> anyhow::Result<usize> {
        let res = self.0.iter().filter(|x| x.is_overlap()).count();
        Ok(res)
    }
}

struct Round((usize, usize), (usize, usize));

impl Round {
    fn is_full_contain(&self) -> bool {
        if self.0 .0 <= self.1 .0 && self.0 .1 >= self.1 .1 {
            true
        } else if self.0 .0 >= self.1 .0 && self.0 .1 <= self.1 .1 {
            true
        } else {
            false
        }
    }

    fn is_overlap(&self) -> bool {
        is_in_range(self.0 .0, self.1)
            || is_in_range(self.0 .1, self.1)
            || is_in_range(self.1 .0, self.0)
            || is_in_range(self.1 .1, self.0)
    }

    fn parse_range(s: &str) -> (usize, usize) {
        let (a, b) = s.split_once('-').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }
}
fn is_in_range(v: usize, r: (usize, usize)) -> bool {
    v >= r.0 && v <= r.1
}
impl FromStr for Round {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(',').unwrap();
        let res = Self(Self::parse_range(a), Self::parse_range(b));
        Ok(res)
    }
}

fn parse(s: &str) -> anyhow::Result<Solution> {
    let res = s.lines().map(|line| line.parse().unwrap());
    Ok(Solution(res.collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_part1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let sol = parse(input).unwrap();
        let res = sol.resolve_part1().unwrap();
        assert_eq!(res, 2);
    }
}
