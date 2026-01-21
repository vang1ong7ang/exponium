use anyhow::Result;
use clap::Parser;
use rug::Float;
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = 256)]
    prec: u32,
    #[arg(long, default_value = "1")]
    rate: String,
    #[arg(long, default_value = "1")]
    cost: String,
    #[arg(long, default_value = "1")]
    prin: String,
    #[arg(long, default_value_t = 1<<16)]
    step: usize,
    #[arg(long, default_value_t = 10)]
    base: i32,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let prin = Float::with_val(args.prec, Float::parse_radix(args.prin, args.base)?);
    let rate = Float::with_val(args.prec, Float::parse_radix(args.rate, args.base)?);
    let cost = Float::with_val(args.prec, Float::parse_radix(args.cost, args.base)?);
    let done = cost / prin;
    let mut copr = Float::new(args.prec);
    let mut time = Float::new(args.prec);
    let mut dist = done.clone();
    let mut test = done.clone();
    while test > copr {
        let mut temp = (test.clone() * 2u32).sqrt();
        for _ in 0..args.step {
            test *= temp.clone() + 1;
            temp += &test;
        }
        if test < done {
            copr += &dist;
            time = temp;
        }
        dist /= 2;
        test = copr.clone() + &dist;
    }
    Ok(println!("{}", (time / rate).to_string_radix(args.base, None)))
}
