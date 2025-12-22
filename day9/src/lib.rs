use std::ops::RangeInclusive;

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

#[derive(Debug, Clone, PartialEq)]
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
    fn get_exit_dir(&self, rotate: &Rotate) -> Self {
        match rotate {
            Rotate::Cw => match self {
                Dir::U => Dir::R,
                Dir::D => Dir::L,
                Dir::L => Dir::U,
                Dir::R => Dir::D,
            },
            Rotate::Ccw => match self {
                Dir::U => Dir::L,
                Dir::D => Dir::R,
                Dir::L => Dir::D,
                Dir::R => Dir::U,
            },
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

    let (min_x, max_x, min_y, max_y) = red.iter().fold(
        (red[0].0, red[0].0, red[0].1, red[0].1),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    );

    let (x_path, y_path) = {
        let mut x: Vec<Vec<(RangeInclusive<u64>, Dir)>> =
            vec![vec![]; (1 + max_y - min_y) as usize];
        let mut y: Vec<Vec<(RangeInclusive<u64>, Dir)>> =
            vec![vec![]; (1 + max_x - min_x) as usize];
        (0..red.len() + 1)
            .map(|i| i % red.len())
            .collect::<Vec<usize>>()
            .windows(2)
            .for_each(|w| {
                let a = red[w[0]];
                let b = red[w[1]];
                if a.0 == b.0 {
                    if a.1 > b.1 {
                        y[(a.0 - min_x) as usize].push(((b.1..=a.1), Dir::U));
                        y[(a.0 - min_x) as usize].sort_by_key(|(r, _)| *r.start());
                    } else {
                        y[(a.0 - min_x) as usize].push(((a.1..=b.1), Dir::D));
                        y[(a.0 - min_x) as usize].sort_by_key(|(r, _)| *r.start());
                    }
                } else if a.0 > b.0 {
                    x[(a.1 - min_y) as usize].push(((b.0..=a.0), Dir::L));
                    x[(a.1 - min_y) as usize].sort_by_key(|(r, _)| *r.start());
                } else {
                    x[(a.1 - min_y) as usize].push(((a.0..=b.0), Dir::R));
                    x[(a.1 - min_y) as usize].sort_by_key(|(r, _)| *r.start());
                }
            });

        (x, y)
    };

    let rotate = {
        let edge = (min_x..=red[0].0)
            .find_map(|x| find_in_paths(min_x, min_y, &x_path, &y_path, (x, red[0].1)))
            .ok_or_else(|| anyhow!("failed to find edge"))?;
        edge[0].to_enter_rotate()
    };
    let (r_exit, l_exit, u_exit, d_exit) = (
        Dir::R.get_exit_dir(&rotate),
        Dir::L.get_exit_dir(&rotate),
        Dir::U.get_exit_dir(&rotate),
        Dir::D.get_exit_dir(&rotate),
    );

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
            let c_i = red[i];
            let c_j = red[j];

            let (start_x, end_x, inc_y, dec_y) = if c_i.0 < c_j.0 {
                (c_i.0, c_j.0, c_i.1, c_j.1)
            } else {
                (c_j.0, c_i.0, c_j.1, c_i.1)
            };

            if (start_x..end_x).any(|x| {
                find_in_paths(min_x, min_y, &x_path, &y_path, (x, inc_y))
                    .map(|dirs| {
                        dirs.contains(&r_exit)
                            && !dirs.contains(&Dir::R)
                            && find_in_paths(min_x, min_y, &x_path, &y_path, (x + 1, inc_y))
                                .is_none()
                    })
                    .unwrap_or(false)
            }) || (start_x + 1..=end_x).any(|x| {
                find_in_paths(min_x, min_y, &x_path, &y_path, (x, dec_y))
                    .map(|dirs| {
                        dirs.contains(&l_exit)
                            && !dirs.contains(&Dir::L)
                            && find_in_paths(min_x, min_y, &x_path, &y_path, (x - 1, dec_y))
                                .is_none()
                    })
                    .unwrap_or(false)
            }) {
                return None;
            }

            let (start_y, end_y, inc_x, dec_x) = if c_i.1 < c_j.1 {
                (c_i.1, c_j.1, c_i.0, c_j.0)
            } else {
                (c_j.1, c_i.1, c_j.0, c_i.0)
            };

            if (start_y..end_y).any(|y| {
                find_in_paths(min_x, min_y, &x_path, &y_path, (inc_x, y))
                    .map(|dirs| {
                        dirs.contains(&d_exit)
                            && !dirs.contains(&Dir::D)
                            && find_in_paths(min_x, min_y, &x_path, &y_path, (inc_x, y + 1))
                                .is_none()
                    })
                    .unwrap_or(false)
            }) || (start_y + 1..=end_y).any(|y| {
                find_in_paths(min_x, min_y, &x_path, &y_path, (dec_x, y))
                    .map(|dirs| {
                        dirs.contains(&u_exit)
                            && !dirs.contains(&Dir::U)
                            && find_in_paths(min_x, min_y, &x_path, &y_path, (dec_x, y - 1))
                                .is_none()
                    })
                    .unwrap_or(false)
            }) {
                return None;
            }

            Some(a.to_string())
        })
        .ok_or_else(|| anyhow!("no areas found"))
}

#[inline]
fn find_in_paths(
    min_x: u64,
    min_y: u64,
    x_path: &[Vec<(RangeInclusive<u64>, Dir)>],
    y_path: &[Vec<(RangeInclusive<u64>, Dir)>],
    k: (u64, u64),
) -> Option<Vec<Dir>> {
    let x_dir = x_path[(k.1 - min_y) as usize]
        .iter()
        .take_while(|(r, _)| *r.start() <= k.0)
        .find(|(r, _)| r.contains(&k.0))
        .map(|(_, dir)| dir);
    let y_dir = y_path[(k.0 - min_x) as usize]
        .iter()
        .take_while(|(r, _)| *r.start() <= k.1)
        .find(|(r, _)| r.contains(&k.1))
        .map(|(_, dir)| dir);
    match (y_dir, x_dir) {
        (None, None) => None,
        (None, Some(d)) | (Some(d), None) => Some(vec![d.clone()]),
        (Some(d1), Some(d2)) => Some(vec![d1.clone(), d2.clone()]),
    }
}
