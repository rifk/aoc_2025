use std::ops::Range;

use anyhow::Result;
use anyhow::anyhow;

pub fn solve_one(input: &str) -> Result<String> {
    let (ranges, ids) = {
        let (r, ids) = input
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("missing empty line"))?;
        let r = r
            .lines()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (s, e) = l
                    .split_once('-')
                    .ok_or_else(|| anyhow!("missing '-': {l}"))?;
                Ok(s.parse::<u64>()?..e.parse::<u64>()? + 1)
            })
            .collect::<Result<Vec<Range<u64>>>>()?;
        let ids = ids
            .lines()
            .map(|l| Ok(l.parse::<u64>()?))
            .collect::<Result<Vec<u64>>>()?;
        (r, ids)
    };
    Ok(ids
        .into_iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut ranges = {
        let (r, _) = input
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("missing empty line"))?;
        r.lines()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (s, e) = l
                    .split_once('-')
                    .ok_or_else(|| anyhow!("missing '-': {l}"))?;
                Ok(s.parse::<u64>()?..e.parse::<u64>()? + 1)
            })
            .collect::<Result<Vec<Range<u64>>>>()?
    };
    ranges.sort_by_key(|r| r.start);
    let mut merged_ranges = vec![ranges[0].clone()];
    for range in ranges.into_iter().skip(1) {
        let i = merged_ranges.len() - 1;
        let last = &mut merged_ranges[i];
        if last.contains(&range.start) {
            if !last.contains(&(range.end - 1)) {
                last.end = range.end;
            }
        } else {
            merged_ranges.push(range);
        }
    }
    Ok(merged_ranges
        .into_iter()
        .map(|range| range.end - range.start)
        .sum::<u64>()
        .to_string())
}
