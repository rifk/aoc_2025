use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

use anyhow::Result;
use anyhow::anyhow;

pub fn solve_one(input: &str) -> Result<String> {
    let red = input
        .lines()
        .map(|l| {
            let (x, y) = l
                .split_once(',')
                .ok_or_else(|| anyhow!("missing ',' - {}", l))?;
            Ok((x.parse::<u64>()?, y.parse::<u64>()?))
        })
        .collect::<Result<Vec<(u64, u64)>>>()?;
    let red = &red;

    let areas = (0..red.len() - 1)
        .flat_map(|i| {
            (i + 1..red.len())
                .map(move |j| (red[i].0.abs_diff(red[j].0) + 1) * (red[i].1.abs_diff(red[j].1) + 1))
        })
        .collect::<Vec<u64>>();

    Ok(areas
        .into_iter()
        .max()
        .ok_or_else(|| anyhow!("no max"))?
        .to_string())
}

#[derive(Debug, Clone)]
enum Dir {
    U,
    D,
    L,
    R,
}
impl Dir {
    fn to_enter_rotate(&self) -> Rotate {
        match self {
            Dir::R | Dir::U => Rotate::Cw,
            Dir::D | Dir::L => Rotate::Ccw,
        }
    }
}
#[derive(Debug, PartialEq)]
enum Rotate {
    Cw,
    Ccw,
}

pub fn solve_two(input: &str) -> Result<String> {
    let red = input
        .lines()
        .map(|l| {
            let (x, y) = l
                .split_once(',')
                .ok_or_else(|| anyhow!("missing ',' - {}", l))?;
            Ok((x.parse::<u64>()?, y.parse::<u64>()?))
        })
        .collect::<Result<Vec<(u64, u64)>>>()?;
    let red = &red;

    let path = (0..red.len() + 1)
        .map(|i| i % red.len())
        .collect::<Vec<usize>>()
        .windows(2)
        .flat_map(|w| {
            let a = red[w[0]];
            let b = red[w[1]];
            if a.0 == b.0 {
                if a.1 > b.1 {
                    (b.1..=a.1)
                        .map(|y| ((a.0, y), Dir::U))
                        .collect::<Vec<((u64, u64), Dir)>>()
                } else {
                    (a.1..=b.1)
                        .map(|y| ((a.0, y), Dir::D))
                        .collect::<Vec<((u64, u64), Dir)>>()
                }
            } else if a.0 > b.0 {
                (b.0..=a.0)
                    .map(|x| ((x, a.1), Dir::L))
                    .collect::<Vec<((u64, u64), Dir)>>()
            } else {
                (a.0..=b.0)
                    .map(|x| ((x, a.1), Dir::R))
                    .collect::<Vec<((u64, u64), Dir)>>()
            }
        })
        .fold(HashMap::new(), |mut map, (k, v)| {
            map.entry(k)
                .and_modify(|vec: &mut Vec<Dir>| vec.push(v.clone()))
                .or_insert_with(|| vec![v]);
            map
        });

    let mut areas = (0..red.len() - 1)
        .flat_map(|i| {
            (i + 1..red.len()).map(move |j| {
                (
                    (red[i].0.abs_diff(red[j].0) + 1) * (red[i].1.abs_diff(red[j].1) + 1),
                    (i, j),
                )
            })
        })
        .collect::<Vec<(u64, (usize, usize))>>();
    areas.sort_by_key(|(a, _)| *a);
    areas.reverse();

    areas
        .into_iter()
        .find_map(|(a, (i, j))| {
            let start_x = min(red[i].0, red[j].0);
            let end_x = max(red[i].0, red[j].0);
            let start_y = min(red[i].1, red[j].1);
            let end_y = max(red[i].1, red[j].1);

            let mut ok = true;

            let mut inside = (None, None);
            let mut x = 0;
            while ok && x < end_x {
                x += 1;
                inside.0 = next_inside(&path, inside.0, (x - 1, start_y), (x, start_y));
                inside.1 = next_inside(&path, inside.1, (x - 1, end_y), (x, end_y));
                if x >= start_x && (inside.0.is_none() || inside.1.is_none()) {
                    ok = false;
                }
            }

            let mut inside = (None, None);
            let mut y = 0;
            while ok && y < end_y {
                y += 1;
                inside.0 = next_inside(&path, inside.0, (start_x, y - 1), (start_x, y));
                inside.1 = next_inside(&path, inside.1, (end_x, y - 1), (end_x, y));
                if y >= start_y && (inside.0.is_none() || inside.1.is_none()) {
                    ok = false;
                }
            }

            if ok { Some(a.to_string()) } else { None }
        })
        .ok_or_else(|| anyhow!("no areas found"))
}

fn next_inside(
    path: &HashMap<(u64, u64), Vec<Dir>>,
    inside: Option<Rotate>,
    from: (u64, u64),
    to: (u64, u64),
) -> Option<Rotate> {
    match path.get(&to) {
        Some(dirs) => match inside {
            Some(_) => inside,
            None => Some(dirs[dirs.len() - 1].to_enter_rotate()),
        },
        None => match path.get(&from) {
            Some(dirs) => {
                if Some(dirs[0].to_enter_rotate()) == inside {
                    inside
                } else {
                    None
                }
            }
            None => inside,
        },
    }
}
