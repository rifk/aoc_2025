use anyhow::Result;
use anyhow::anyhow;

pub fn solve_one(input: &str) -> Result<String> {
    let (mut beams, splitters) = {
        let (b, s) = input
            .split_once('\n')
            .ok_or_else(|| anyhow!("missing new line"))?;
        (
            vec![b.find('S').ok_or_else(|| anyhow!("missing 'S' - {}", b))?],
            s.lines()
                .map(|l| {
                    l.chars()
                        .enumerate()
                        .filter_map(|(i, c)| match c {
                            '^' => Some(i),
                            _ => None,
                        })
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
        )
    };

    let mut splits = 0;

    splitters.iter().for_each(|s| {
        beams = beams
            .clone()
            .into_iter()
            .flat_map(|i| {
                if s.contains(&i) {
                    splits += 1;
                    vec![i - 1, i + 1]
                } else {
                    vec![i]
                }
                .into_iter()
            })
            .collect();
        beams.dedup();
    });

    Ok(splits.to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let (mut beams, splitters) = {
        let (b, s) = input
            .split_once('\n')
            .ok_or_else(|| anyhow!("missing new line"))?;
        let mut beams = vec![0_u64; b.len()];
        beams[b.find('S').ok_or_else(|| anyhow!("missing 'S' - {}", b))?] = 1;
        (
            beams,
            s.lines()
                .map(|l| {
                    l.chars()
                        .enumerate()
                        .filter_map(|(i, c)| match c {
                            '^' => Some(i),
                            _ => None,
                        })
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
        )
    };

    splitters.iter().for_each(|s| {
        let mut next_beams = vec![0_u64; beams.len()];
        for (i, c) in beams.iter().enumerate() {
            if *c > 0 {
                if s.contains(&i) {
                    next_beams[i - 1] += c;
                    next_beams[i + 1] += c;
                } else {
                    next_beams[i] += c;
                }
            }
        }
        beams = next_beams;
    });

    Ok(beams.into_iter().sum::<u64>().to_string())
}
