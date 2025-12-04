use anyhow::Result;
use anyhow::bail;

pub fn solve_one(input: &str) -> Result<String> {
    let rolls = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Ok(false),
                    '@' => Ok(true),
                    c => bail!("unexpected char {c}"),
                })
                .collect::<Result<Vec<bool>>>()
        })
        .collect::<Result<Vec<Vec<bool>>>>()?;

    let mut count = 0;

    let max_i = rolls.len() - 1;
    let max_j = rolls[0].len() - 1;
    for i in 0..=max_i {
        for j in 0..=max_j {
            if !rolls[i][j] {
                continue;
            }

            if count_adjacent(&rolls, i, j) < 4 {
                count += 1;
            }
        }
    }

    Ok(count.to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut rolls = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Ok(false),
                    '@' => Ok(true),
                    c => bail!("unexpected char {c}"),
                })
                .collect::<Result<Vec<bool>>>()
        })
        .collect::<Result<Vec<Vec<bool>>>>()?;

    let mut count = 0;
    let mut removed = true;

    while removed {
        removed = false;
        for i in 0..rolls.len() {
            for j in 0..rolls[0].len() {
                if !rolls[i][j] {
                    continue;
                }

                if count_adjacent(&rolls, i, j) < 4 {
                    count += 1;
                    rolls[i][j] = false;
                    removed = true;
                }
            }
        }
    }

    Ok(count.to_string())
}

fn count_adjacent(rolls: &[Vec<bool>], i: usize, j: usize) -> usize {
    let mut adj = 0;

    i.checked_sub(1).inspect(|&i| {
        j.checked_sub(1).inspect(|&j| {
            if has_roll(rolls, i, j) {
                adj += 1;
            }
        });
        if has_roll(rolls, i, j) {
            adj += 1;
        }
        if has_roll(rolls, i, j + 1) {
            adj += 1;
        }
    });
    j.checked_sub(1).inspect(|&j| {
        if has_roll(rolls, i, j) {
            adj += 1;
        }
    });
    if has_roll(rolls, i, j + 1) {
        adj += 1;
    }
    j.checked_sub(1).inspect(|&j| {
        if has_roll(rolls, i + 1, j) {
            adj += 1;
        }
    });
    if has_roll(rolls, i + 1, j) {
        adj += 1;
    }
    if has_roll(rolls, i + 1, j + 1) {
        adj += 1;
    }

    adj
}

fn has_roll(rolls: &[Vec<bool>], i: usize, j: usize) -> bool {
    if i >= rolls.len() || j >= rolls[0].len() {
        return false;
    }
    rolls[i][j]
}
