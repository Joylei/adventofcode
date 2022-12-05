use std::collections::HashSet;
use std::fs;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("./examples/day3_input.txt")?;
    let sol = parse(&input)?;
    let res = sol.resolve_part1()?;
    println!("res={}", res);
    Ok(())
}

struct Solution<'a>(Vec<Round<'a>>);

impl Solution<'_> {
    fn resolve_part1(&self) -> anyhow::Result<u32> {
        let res = self.0.iter().map(|x| x.priority_of_common_type()).sum();
        Ok(res)
    }
}

struct Round<'a>(&'a str, &'a str);

impl Round<'_> {
    fn common_type(&self) -> char {
        //println!("{}   {}", self.0, self.1);
        let a: HashSet<_> = self.0.chars().collect();
        let b: HashSet<_> = self.1.chars().collect();
        let res = a.intersection(&b).cloned().next().unwrap();
        res
    }

    fn priority_of_common_type(&self) -> u32 {
        let c = self.common_type();
        match c {
            'a'..='z' => c as u32 - 'a' as u32 + 1,
            'A'..='Z' => c as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        }
    }
}

fn parse(s: &str) -> anyhow::Result<Solution> {
    let res = s.lines().map(|line| {
        let len = line.len() / 2;
        let a = &line[0..len];
        let b = &line[len..];
        Round(a, b)
    });
    Ok(Solution(res.collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let sol = parse(input).unwrap();
        let res = sol.resolve_part1().unwrap();
        assert_eq!(res, 157);
    }
}
