use std::collections::HashSet;

use anyhow::Result;
use anyhow::anyhow;

pub fn solve_one(input: &str) -> Result<String> {
    let ranges = input
        .trim()
        .split(',')
        .map(|v| {
            let (first, last) = v
                .split_once('-')
                .ok_or_else(|| anyhow!("missing - in part"))?;
            let first = first.parse::<u64>()?;
            let last = last.parse::<u64>()?;
            Ok(first..=last)
        })
        .collect::<Result<Vec<_>>>()?;
    let res = ranges
        .into_iter()
        .flatten()
        .filter(|v| {
            let mut mult = 10;
            while (v / mult) > (v % mult) {
                mult *= 10;
            }
            (v / mult) == (v % mult) && (v % mult) / (mult / 10) != 0
        })
        .sum::<u64>();
    Ok(res.to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let ranges = input
        .trim()
        .split(',')
        .map(|v| {
            let (first, last) = v
                .split_once('-')
                .ok_or_else(|| anyhow!("missing - in part"))?;
            let first = first.parse::<u64>()?;
            let last = last.parse::<u64>()?;
            Ok(first..=last)
        })
        .collect::<Result<Vec<_>>>()?;
    let res = ranges
        .into_iter()
        .flatten()
        .filter(|v| {
            let v = v.to_string().chars().collect::<Vec<_>>();
            (1..=v.len() / 2).any(|l| v.chunks(l).collect::<HashSet<_>>().len() == 1)
        })
        .sum::<u64>();
    Ok(res.to_string())
}
