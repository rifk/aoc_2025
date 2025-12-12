use std::cmp::min;
use std::collections::HashSet;

use anyhow::Result;
use anyhow::anyhow;

pub fn solve_one(input: &str) -> Result<String> {
    let machines = input
        .lines()
        .map(|line| {
            let (l, line) = line
                .split_once(' ')
                .ok_or_else(|| anyhow!("missing space {}", line))?;
            let (b, _) = line
                .rsplit_once(' ')
                .ok_or_else(|| anyhow!("missing space {}", line))?;
            let l = l
                .trim_start_matches('[')
                .trim_end_matches(']')
                .chars()
                .enumerate()
                .fold(0_u16, |a, (i, c)| if c == '#' { a | (1 << i) } else { a });
            let b = b
                .split(' ')
                .map(|b| {
                    Ok(b.trim_start_matches('(')
                        .trim_end_matches(')')
                        .split(',')
                        .map(|i| Ok(i.parse::<usize>()?))
                        .collect::<Result<Vec<usize>>>()?
                        .into_iter()
                        .fold(0_u16, |a, i| a | (1 << i)))
                })
                .collect::<Result<Vec<u16>>>()?;
            Ok((l, b))
        })
        .collect::<Result<Vec<(u16, Vec<u16>)>>>()?;

    Ok(machines
        .iter()
        .map(|(lights, buttons)| {
            let mut states = HashSet::new();
            states.insert(0);
            let mut count = 0;
            while !states.contains(lights) {
                states = states
                    .into_iter()
                    .flat_map(|s| buttons.iter().map(move |b| s ^ b))
                    .collect();
                count += 1;
            }
            count
        })
        .sum::<usize>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let machines = input
        .lines()
        .map(|line| {
            let (line, jolts) = line
                .rsplit_once(' ')
                .ok_or_else(|| anyhow!("missing space {}", line))?;
            let (_, b) = line
                .split_once(' ')
                .ok_or_else(|| anyhow!("missing space {}", line))?;
            let jolts = jolts
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|v| Ok(v.parse::<u32>()?))
                .collect::<Result<Vec<u32>>>()?;
            let mut b = b
                .split(' ')
                .map(|b| {
                    b.trim_start_matches('(')
                        .trim_end_matches(')')
                        .split(',')
                        .map(|i| Ok(i.parse::<usize>()?))
                        .collect::<Result<Vec<usize>>>()
                })
                .collect::<Result<Vec<Vec<usize>>>>()?;
            b.sort_by_key(|b| b.len());
            b.reverse();
            Ok((jolts, b))
        })
        .collect::<Result<Vec<(Vec<u32>, Vec<Vec<usize>>)>>>()?;

    Ok(machines
        .iter()
        .map(|(jolts, buttons)| {
            find_count(buttons, jolts.clone(), vec![true; buttons.len()], u32::MAX)
                .ok_or_else(|| anyhow!("cannot find count"))
        })
        .sum::<Result<u32>>()?
        .to_string())
}

fn find_count(
    buttons: &[Vec<usize>],
    mut remaining_jolts: Vec<u32>,
    remaining_buttons: Vec<bool>,
    max_count: u32,
) -> Option<u32> {
    if max_count <= *remaining_jolts.iter().max().unwrap() {
        return None;
    }
    let i = match remaining_jolts
        .iter()
        .enumerate()
        .filter(|&(_, j)| *j > 0)
        .min_by_key(|&(i, _)| {
            buttons
                .iter()
                .enumerate()
                .filter(|(j, b)| remaining_buttons[*j] && b.contains(&i))
                .count()
        }) {
        Some((i, _)) => i,
        None => return Some(0),
    };
    let include_buttons = buttons
        .iter()
        .enumerate()
        .filter(|(j, b)| remaining_buttons[*j] && b.contains(&i))
        .collect::<Vec<(usize, &Vec<usize>)>>();
    if include_buttons.is_empty() {
        return None;
    }
    let mut count = u32::MAX;
    let mut combination = {
        let mut comb = vec![0; include_buttons.len()];
        comb[0] = remaining_jolts[i];
        Some(comb)
    };
    let c = remaining_jolts[i];
    remaining_jolts[i] = 0;
    let mut next_max_count = max_count - c;
    while let Some(comb) = combination {
        let mut remaining_jolts = remaining_jolts.clone();
        let mut remaining_buttons = remaining_buttons.clone();
        let mut ok = true;
        'ok: for (comb_i, (j, b)) in include_buttons.iter().enumerate() {
            for k in *b {
                if *k != i {
                    remaining_jolts[*k] = match remaining_jolts[*k].checked_sub(comb[comb_i]) {
                        Some(v) => v,
                        None => {
                            ok = false;
                            break 'ok;
                        }
                    }
                }
            }
            remaining_buttons[*j] = false;
        }
        if ok
            && let Some(c2) =
                find_count(buttons, remaining_jolts, remaining_buttons, next_max_count)
        {
            next_max_count = min(next_max_count, c2);
            count = min(count, c + c2);
        }
        combination = next_combination(comb);
    }
    if count == u32::MAX { None } else { Some(count) }
}

fn next_combination(mut combination: Vec<u32>) -> Option<Vec<u32>> {
    let i = combination.iter().position(|&v| v != 0).unwrap();
    if i >= combination.len() - 1 {
        return None;
    }
    let v = combination[i];
    combination[i + 1] += 1;
    combination[i] = 0;
    combination[0] = v - 1;
    Some(combination)
}
