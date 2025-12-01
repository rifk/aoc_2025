use anyhow::Result;
use anyhow::bail;

enum Turn {
    Left(usize),
    Right(usize),
}

pub fn solve_one(input: &str) -> Result<String> {
    let turns = input
        .lines()
        .map(|v| {
            let (dir, num) = v.split_at(1);
            let num = num.parse::<usize>()?;
            match dir {
                "L" => Result::Ok(Turn::Left(num)),
                "R" => Result::Ok(Turn::Right(num)),
                _ => bail!("unexpected char {}", dir),
            }
        })
        .collect::<Result<Vec<Turn>>>()?;

    let count = turns
        .into_iter()
        .scan(50, |pos, turn| {
            *pos += match turn {
                Turn::Right(num) => num as i32,
                Turn::Left(num) => -(num as i32),
            };
            Some(pos.rem_euclid(100))
        })
        .filter(|&pos| pos == 0)
        .count();

    Ok(count.to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let turns = input
        .lines()
        .map(|v| {
            let (dir, num) = v.split_at(1);
            let num = num.parse::<usize>()?;
            match dir {
                "L" => Result::Ok(Turn::Left(num)),
                "R" => Result::Ok(Turn::Right(num)),
                _ => bail!("unexpected char {}", dir),
            }
        })
        .collect::<Result<Vec<Turn>>>()?;

    let count = turns
        .into_iter()
        .fold((50_i32, 0_i32), |(mut pos, mut count), turn| {
            let start_zero = pos == 0;
            pos += match turn {
                Turn::Left(num) => -(num as i32),
                Turn::Right(num) => num as i32,
            };

            if pos.is_positive() {
                count += pos.div_euclid(100);
                pos = pos.rem_euclid(100);
            } else if pos.is_negative() {
                count += (pos - 1).div_euclid(100).abs();
                pos = pos.rem_euclid(100);
                if start_zero {
                    count -= 1;
                }
            } else {
                count += 1;
            }
            (pos, count)
        })
        .1;

    Ok(count.to_string())
}
