use std::collections::HashSet;

use anyhow::Result;
use anyhow::anyhow;

pub fn solve_one(input: &str) -> Result<String> {
    let boxes = input
        .lines()
        .map(|l| {
            let mut parts = l.splitn(3, ',');
            Ok((
                parts
                    .next()
                    .ok_or_else(|| anyhow!("missing first part - {}", l))?
                    .parse::<f64>()?,
                parts
                    .next()
                    .ok_or_else(|| anyhow!("missing second part - {}", l))?
                    .parse::<f64>()?,
                parts
                    .next()
                    .ok_or_else(|| anyhow!("missing third part - {}", l))?
                    .parse::<f64>()?,
            ))
        })
        .collect::<Result<Vec<(f64, f64, f64)>>>()?;
    let boxes = &boxes;

    let mut distances = (0..boxes.len() - 1)
        .flat_map(|i| {
            (i + 1..boxes.len()).map(move |j| {
                let d = ((boxes[i].0 - boxes[j].0).abs().powi(2)
                    + (boxes[i].1 - boxes[j].1).abs().powi(2)
                    + (boxes[i].2 - boxes[j].2).abs().powi(2))
                .sqrt();
                ((i, j), d)
            })
        })
        .collect::<Vec<((usize, usize), f64)>>();
    distances.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let mut circuits: Vec<HashSet<usize>> = vec![];
    for ((i, j), _) in distances.iter().take(1000) {
        let mut i_c = None;
        let mut j_c = None;
        for c in &mut circuits {
            if c.contains(i) {
                i_c = Some(c);
            } else if c.contains(j) {
                j_c = Some(c);
            }
        }

        match (i_c, j_c) {
            (None, Some(c)) | (Some(c), None) => {
                c.insert(*i);
                c.insert(*j);
            }
            (Some(i_c), Some(j_c)) => {
                j_c.iter().for_each(|v| {
                    i_c.insert(*v);
                });
                j_c.clear();
                i_c.insert(*i);
                i_c.insert(*j);
            }
            (None, None) => {
                let mut c = HashSet::new();
                c.insert(*i);
                c.insert(*j);
                circuits.push(c);
            }
        }
    }
    circuits.sort_by_key(|c| c.len());

    Ok(circuits
        .into_iter()
        .rev()
        .take(3)
        .map(|c| c.len())
        .product::<usize>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let boxes = input
        .lines()
        .map(|l| {
            let mut parts = l.splitn(3, ',');
            Ok((
                parts
                    .next()
                    .ok_or_else(|| anyhow!("missing first part - {}", l))?
                    .parse::<f64>()?,
                parts
                    .next()
                    .ok_or_else(|| anyhow!("missing second part - {}", l))?
                    .parse::<f64>()?,
                parts
                    .next()
                    .ok_or_else(|| anyhow!("missing third part - {}", l))?
                    .parse::<f64>()?,
            ))
        })
        .collect::<Result<Vec<(f64, f64, f64)>>>()?;
    let boxes = &boxes;

    let mut distances = (0..boxes.len() - 1)
        .flat_map(|i| {
            (i + 1..boxes.len()).map(move |j| {
                let d = ((boxes[i].0 - boxes[j].0).abs().powi(2)
                    + (boxes[i].1 - boxes[j].1).abs().powi(2)
                    + (boxes[i].2 - boxes[j].2).abs().powi(2))
                .sqrt();
                ((i, j), d)
            })
        })
        .collect::<Vec<((usize, usize), f64)>>();
    distances.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let mut result = 0_f64;
    let mut circuits: Vec<HashSet<usize>> = vec![];
    for ((i, j), _) in distances.iter() {
        let mut i_c = None;
        let mut j_c = None;
        for c in &mut circuits {
            if c.contains(i) {
                i_c = Some(c);
            } else if c.contains(j) {
                j_c = Some(c);
            }
        }

        let c_len = match (i_c, j_c) {
            (None, Some(c)) | (Some(c), None) => {
                c.insert(*i);
                c.insert(*j);
                c.len()
            }
            (Some(i_c), Some(j_c)) => {
                j_c.drain().for_each(|v| {
                    i_c.insert(v);
                });
                i_c.insert(*i);
                i_c.insert(*j);
                i_c.len()
            }
            (None, None) => {
                let mut c = HashSet::new();
                c.insert(*i);
                c.insert(*j);
                let len = c.len();
                circuits.push(c);
                len
            }
        };

        if c_len == boxes.len() {
            result = boxes[*i].0 * boxes[*j].0;
            break;
        }
    }
    Ok(result.to_string())
}
