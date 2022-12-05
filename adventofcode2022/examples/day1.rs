use std::fs;
use std::mem;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("./examples/day1_input.txt")?;
    let sol = parse(&input)?;
    let res = sol.resolve_part1()?;
    println!("res={}", res);
    let res = sol.resolve_part2()?;
    println!("res={}", res);
    Ok(())
}

struct Solution(Vec<Elf>);

impl Solution {
    fn resolve_part1(&self) -> anyhow::Result<usize> {
        let res = self.0.iter().map(|x| x.sum()).max().unwrap();
        Ok(res)
    }

    fn resolve_part2(&self) -> anyhow::Result<usize> {
        let mut v: Vec<_> = self.0.iter().map(|x| x.sum()).collect();
        v.sort();
        v.reverse();
        let res = v.iter().take(3).sum();
        Ok(res)
    }
}

struct Elf(Vec<usize>);
impl Elf {
    fn sum(&self) -> usize {
        self.0.iter().sum()
    }
}

fn parse(s: &str) -> anyhow::Result<Solution> {
    let mut res = vec![];
    let mut cur = vec![];
    for line in s.lines() {
        if line.is_empty() {
            let t = mem::take(&mut cur);
            res.push(Elf(t));
        } else {
            cur.push(line.parse()?);
        }
    }
    if !cur.is_empty() {
        res.push(Elf(cur));
    }
    Ok(Solution(res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_part1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let sol = parse(input).unwrap();
        let res = sol.resolve_part1().unwrap();
        assert_eq!(res, 24000);
    }
}
