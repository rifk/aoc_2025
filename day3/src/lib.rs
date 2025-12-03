use std::ops::Range;

use anyhow::Result;
use anyhow::anyhow;

pub fn solve_one(input: &str) -> Result<String> {
    let banks = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).ok_or_else(|| anyhow!("not digit {}", c)))
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;
    let total = banks
        .into_iter()
        .map(|bank| {
            let mut f = 0;
            for i in 1..bank.len() - 1 {
                if bank[i] > bank[f] {
                    f = i
                }
            }

            let mut l = f + 1;
            let r = l..bank.len();
            for i in r {
                if bank[i] > bank[l] {
                    l = i
                }
            }

            (10 * bank[f]) + bank[l]
        })
        .sum::<u32>();
    Ok(total.to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let banks = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| anyhow!("not digit {}", c))
                        .map(|v| v as u64)
                })
                .collect::<Result<Vec<u64>>>()
        })
        .collect::<Result<Vec<Vec<u64>>>>()?;
    let total = banks
        .into_iter()
        .map(|bank| {
            let mut i = find_largest_i(&bank, 0..bank.len() - 11);
            let mut jolt = bank[i];
            for j in (0..11).rev() {
                i = find_largest_i(&bank, i + 1..bank.len() - j);
                jolt *= 10;
                jolt += bank[i];
            }
            Ok(jolt)
        })
        .sum::<Result<u64>>()?;
    Ok(total.to_string())
}

fn find_largest_i(bank: &[u64], range: Range<usize>) -> usize {
    let mut i = range.start;
    for j in range.skip(1) {
        if bank[j] > bank[i] {
            i = j;
        }
    }
    i
}
