use std::fs;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("./examples/day5_input.txt")?;
    let mut sol = parse(&input)?;
    let res = sol.resolve_part1()?;
    println!("res={}", res);

    let res = sol.resolve_part2()?;
    println!("res={}", res);
    Ok(())
}

#[derive(Debug, Default)]
struct Solution {
    stacks: Vec<Stack>,
    moves: Vec<(usize, usize, usize)>,
}

impl Solution {
    fn resolve_part1(&mut self) -> anyhow::Result<String> {
        let mut stacks = self.stacks.clone();
        let moves = self.moves.clone();
        for (n, src, dst) in moves {
            //bypass borrow checker
            let dst = unsafe { &mut *(&mut stacks[dst - 1] as *mut _) };
            stacks[src - 1].move_one(dst, n);
        }
        let res: String = stacks.iter().map(|s| s.top()).collect();
        Ok(res)
    }

    fn resolve_part2(&mut self) -> anyhow::Result<String> {
        let mut stacks = self.stacks.clone();
        let moves = self.moves.clone();
        for (n, src, dst) in moves {
            //bypass borrow checker
            let dst = unsafe { &mut *(&mut stacks[dst - 1] as *mut _) };
            stacks[src - 1].move_many(dst, n);
        }
        let res: String = stacks.iter().map(|s| s.top()).collect();
        Ok(res)
    }

    pub fn ensure_stack(&mut self, i: usize) -> &mut Stack {
        while self.stacks.len() < i + 1 {
            self.stacks.push(Stack::default());
        }
        &mut self.stacks[i]
    }

    pub fn init(&mut self) {
        for s in &mut self.stacks {
            s.reverse();
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Stack(Vec<char>);

impl Stack {
    fn put(&mut self, c: char) {
        self.0.push(c);
    }

    fn top(&self) -> char {
        self.0.last().cloned().unwrap()
    }

    fn reverse(&mut self) {
        self.0.reverse();
    }

    fn move_one(&mut self, dst: &mut Self, n: usize) {
        for _ in 0..n {
            let c = self.0.pop().unwrap();
            dst.put(c);
        }
    }

    fn move_many(&mut self, dst: &mut Self, n: usize) {
        let i = dst.0.len();
        for _ in 0..n {
            let c = self.0.pop().unwrap();
            dst.0.insert(i, c);
        }
    }
}

fn parse(s: &str) -> anyhow::Result<Solution> {
    // parse stacks
    let mut res = Solution::default();
    let mut lines = s.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut holder = None;
        for (i, c) in line.chars().enumerate() {
            if c == '[' {
                let idx = i / 4;
                holder = Some(res.ensure_stack(idx));
                continue;
            }
            if let Some(stack) = holder.take() {
                stack.put(c);
            }
        }
    }
    res.init();

    // parse moves
    for line in lines {
        let m = parse_move(line);
        res.moves.push(m);
    }

    Ok(res)
}

fn parse_move(line: &str) -> (usize, usize, usize) {
    let vec: Vec<_> = line.split(' ').filter_map(|v| v.parse().ok()).collect();
    (vec[0], vec[1], vec[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_part1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let mut sol = parse(input).unwrap();
        let res = sol.resolve_part1().unwrap();
        assert_eq!(res, "CMZ");
    }

    #[test]
    fn test_resolve_part2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let mut sol = parse(input).unwrap();
        let res = sol.resolve_part2().unwrap();
        assert_eq!(res, "MCD");
    }
}
