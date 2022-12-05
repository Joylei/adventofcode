use std::collections::HashSet;
use std::fs;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("./examples/day3_input.txt")?;
    let sol = parse(&input)?;
    let res = sol.resolve_part2()?;
    println!("res={}", res);
    Ok(())
}

struct Solution<'a>(Vec<Round<'a>>);

impl Solution<'_> {
    fn resolve_part2(&self) -> anyhow::Result<u32> {
        let res = self.0.iter().map(|x| x.priority_of_common_type()).sum();
        Ok(res)
    }
}

#[derive(Default)]
struct Round<'a> {
    a: &'a str,
    b: &'a str,
    c: &'a str,
}

impl Round<'_> {
    fn common_type(&self) -> char {
        //println!("{}   {}", self.0, self.1);
        let a: HashSet<_> = self.a.chars().collect();
        let b: HashSet<_> = self.b.chars().collect();
        let c: HashSet<_> = self.c.chars().collect();
        let t: HashSet<_> = a.intersection(&b).cloned().collect();
        let res = t.intersection(&c).next().unwrap();
        *res
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
    let mut res = vec![];
    let mut iter = s.lines();
    loop {
        let a = match iter.next() {
            None => break,
            Some(v) => v,
        };
        let mut r = Round::default();
        r.a = a;
        r.b = iter.next().unwrap();
        r.c = iter.next().unwrap();
        res.push(r);
    }
    Ok(Solution(res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let sol = parse(input).unwrap();
        let res = sol.resolve_part2().unwrap();
        assert_eq!(res, 70);
    }
}
