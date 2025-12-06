use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Debug)]
enum Op {
    Mult(Vec<u64>),
    Add(Vec<u64>),
}

pub fn solve_one(input: &str) -> Result<String> {
    let problems = {
        let lines = input.lines();
        let nums = lines
            .clone()
            .take_while(|l| !l.contains('+') && !l.contains('*'))
            .map(|l| {
                l.split_whitespace()
                    .map(|n| Ok(n.parse::<u64>()?))
                    .collect::<Result<Vec<u64>>>()
            })
            .collect::<Result<Vec<Vec<u64>>>>()?;
        lines
            .last()
            .ok_or_else(|| anyhow!("no lines"))?
            .split_whitespace()
            .enumerate()
            .map(|(i, op)| {
                let v = nums.iter().map(|n| n[i]).collect();
                Ok(match op {
                    "+" => Op::Add(v),
                    "*" => Op::Mult(v),
                    _ => bail!("unexpected char {}", op),
                })
            })
            .collect::<Result<Vec<Op>>>()?
    };
    Ok(problems
        .into_iter()
        .map(|op| match op {
            Op::Mult(items) => items.into_iter().product::<u64>(),
            Op::Add(items) => items.into_iter().sum(),
        })
        .sum::<u64>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let problems = {
        let (nums, ops) = input
            .trim_end()
            .rsplit_once('\n')
            .ok_or_else(|| anyhow!("no new lines"))?;
        let nums = nums
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        ops.char_indices()
            .filter(|(_, c)| *c == '+' || *c == '*')
            .map(|(i, c)| {
                get_nums(&nums, i).and_then(|v| match c {
                    '+' => Ok(Op::Add(v)),
                    '*' => Ok(Op::Mult(v)),
                    _ => bail!("unexpected char {c}"),
                })
            })
            .collect::<Result<Vec<Op>>>()?
    };
    Ok(problems
        .into_iter()
        .map(|op| match op {
            Op::Mult(items) => items.into_iter().product::<u64>(),
            Op::Add(items) => items.into_iter().sum(),
        })
        .sum::<u64>()
        .to_string())
}

fn get_nums(nums: &[Vec<char>], i: usize) -> Result<Vec<u64>> {
    let mut v = vec![];

    let mut stop = false;
    let mut i = i;
    while !stop {
        stop = true;
        let mut n = 0_u64;
        for nums in nums {
            if i >= nums.len() || nums[i] == ' ' {
                continue;
            }

            n *= 10;
            n += nums[i]
                .to_digit(10)
                .ok_or_else(|| anyhow!("unexpected char {}", nums[i]))? as u64;
            stop = false;
        }
        if n != 0 {
            v.push(n);
        }

        i += 1;
    }
    Ok(v)
}
