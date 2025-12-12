use std::collections::HashMap;

use anyhow::Result;
use anyhow::anyhow;

pub fn solve_one(input: &str) -> Result<String> {
    let devices = input
        .lines()
        .map(|l| {
            let (d, out) = l
                .split_once(": ")
                .ok_or_else(|| anyhow!("missing ': ' - {}", l))?;
            Ok((
                d.to_string(),
                out.split(' ').map(|o| o.to_string()).collect::<Vec<_>>(),
            ))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    count_outs_one(&devices, "you").map(|c| c.to_string())
}

fn count_outs_one(devices: &HashMap<String, Vec<String>>, in_device: &str) -> Result<usize> {
    let outs = devices
        .get(in_device)
        .ok_or_else(|| anyhow!("cannot find device {}", in_device))?;
    outs.iter()
        .map(|o| {
            if o == "out" {
                Ok(1)
            } else {
                count_outs_one(devices, o)
            }
        })
        .collect::<Result<Vec<usize>>>()
        .map(|v| v.into_iter().sum::<usize>())
}

pub fn solve_two(input: &str) -> Result<String> {
    let devices = input
        .lines()
        .map(|l| {
            let (d, out) = l
                .split_once(": ")
                .ok_or_else(|| anyhow!("missing ': ' - {}", l))?;
            Ok((
                d.to_string(),
                out.split(' ').map(|o| o.to_string()).collect::<Vec<_>>(),
            ))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    let (f, s, count) = {
        let dac_to_fft = count_in_path(&mut HashMap::new(), "fft", &devices, "dac");
        if dac_to_fft > 0 {
            ("dac", "fft", dac_to_fft)
        } else {
            (
                "fft",
                "dac",
                count_in_path(&mut HashMap::new(), "dac", &devices, "fft"),
            )
        }
    };

    let svr_to_f = count_in_path(&mut HashMap::new(), f, &devices, "svr");

    let s_to_out = count_in_path(&mut HashMap::new(), "out", &devices, s);

    Ok((svr_to_f * count * s_to_out).to_string())
}

fn count_in_path(
    mem: &mut HashMap<String, usize>,
    end: &str,
    devices: &HashMap<String, Vec<String>>,
    in_device: &str,
) -> usize {
    if let Some(c) = mem.get(in_device) {
        return *c;
    }
    let c = match devices.get(in_device) {
        Some(outs) => outs
            .iter()
            .map(|o| {
                if o == end {
                    1
                } else {
                    count_in_path(mem, end, devices, o)
                }
            })
            .sum(),
        None => 0,
    };
    mem.insert(in_device.to_string(), c);
    c
}
