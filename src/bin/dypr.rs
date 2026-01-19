use anyhow::Result;
use clap::Parser;
use rug::Float;
use rug::float::Round;
use std::collections::VecDeque;
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = 256)]
    prec: u32,
    #[arg(long, default_value = "1")]
    cost: String,
    #[arg(long, default_value = "0.001")]
    step: String,
    #[arg(long, default_value_t = 10)]
    base: i32,
    #[arg(long, default_value_t = false)]
    show: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let prec = args.prec;
    let cost = Float::with_val(prec, Float::parse_radix(&args.cost, args.base)?);
    let step = Float::with_val(prec, Float::parse_radix(&args.step, args.base)?);
    let prin = Float::with_val(prec, 1.0);
    let sqrt = cost.clone().sqrt();
    let mins = ((cost.clone() + &sqrt) / &step).to_u32_saturating_round(Round::Down).unwrap() as usize;
    let mut nmap = vec![0; (sqrt / &step).to_u32_saturating_round(Round::Up).unwrap() as usize + 1];
    let mut sats = VecDeque::new();
    let mut data = VecDeque::from([(prin, 0, 0, usize::MAX, 0)]);
    let mut skip = 0;
    for size in 1usize.. {
        let (last_prin, last_prev, last_root, last_beat, _) = data.back().unwrap();
        if *last_root > mins {
            nmap[last_root - mins] += 1;
            if nmap[last_root - mins] == data.len() {
                break;
            }
        }
        while let Some(&front) = sats.front() {
            if front <= *last_prev {
                sats.pop_front();
            } else {
                break;
            }
        }
        let (prin, prev, root) = (0..sats.len() + 1).map(|i| (if i == 0 { skip } else { sats[i - 1] }, if i == sats.len() { size } else { sats[i] })).map(|(x, y)| data.partition_point(|(_, _, _, beat, time)| *time < x || (*time < y && *beat <= size))).map(|v| &data[if v > 0 { v - 1 } else { 0 }]).map(|(prin, prev, root, _, time)| (prin.clone() * &step * (size - *time) + prin - &cost, *time, if *root == 0 { *prev } else { *root })).max_by(|x, y| x.0.partial_cmp(&y.0).unwrap()).unwrap();
        let dist = prin.clone() - last_prin;
        let beat = ((last_prin - dist.clone() / &step) / &dist).to_u32_saturating_round(Round::Up).unwrap() as usize + size;
        if beat < *last_beat {
            sats.push_back(size);
        }
        for _ in skip..prev {
            let (_, _, root, _, _) = data.pop_front().unwrap();
            if root > mins {
                nmap[root - mins] -= 1;
            }
        }
        skip = prev;
        data.push_back((prin, prev, root, beat, size));
    }
    if args.show {
        println!("{}", (step * data.back().unwrap().2).to_string_radix(args.base, None));
    } else {
        println!("{}", data.back().unwrap().2);
    }
    Ok(())
}
