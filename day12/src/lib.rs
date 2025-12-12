use anyhow::Result;
use anyhow::anyhow;

pub fn solve_one(input: &str) -> Result<String> {
    let areas = input
        .rsplit_once("\n\n")
        .ok_or_else(|| anyhow!("missing empty line"))?
        .1
        .lines()
        .map(|l| {
            let (a, c) = l
                .split_once(": ")
                .ok_or_else(|| anyhow!("missing ': ' - {}", l))?;
            let a = a
                .split_once('x')
                .ok_or_else(|| anyhow!("missing 'x' - {}", a))?;
            Ok((
                (a.0.parse::<u64>()?, a.1.parse::<u64>()?),
                c.split(' ')
                    .map(|v| Ok(v.parse::<u64>()?))
                    .collect::<Result<Vec<u64>>>()?,
            ))
        })
        .collect::<Result<Vec<((u64, u64), Vec<u64>)>>>()?;

    Ok(areas
        .into_iter()
        .filter(|(a, counts)| (a.0 * a.1) >= (9 * counts.iter().sum::<u64>()))
        .count()
        .to_string())
}

pub fn solve_two(_: &str) -> Result<String> {
    todo!()
}
