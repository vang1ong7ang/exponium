use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use exponium::Expr;
use rug::Float;
use std::io::BufRead;
use std::io::stdin;
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = 256)]
    prec: u32,
    #[arg(long, default_value = "1.0")]
    rate: String,
    #[arg(long, default_value = "1.0")]
    cost: String,
    #[arg(long, default_value = "10000000000.0")]
    term: String,
    #[arg(long, default_value_t = 10)]
    base: i32,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let rate = Float::with_val(args.prec, Float::parse_radix(args.rate, args.base)?);
    let cost = Float::with_val(args.prec, Float::parse_radix(args.cost, args.base)?);
    let term = Float::with_val(args.prec, Float::parse_radix(args.term, args.base)?);
    let expr = stdin().lock().lines().map(|v| read(v?)).collect::<Result<Vec<(char, char, char, char)>>>()?;
    if let Some(result) = Expr::from(expr).term::<1048576>(args.prec, rate, cost, term) { Ok(println!("{}", result.to_string_radix(args.base, None))) } else { Ok(eprintln!("Error evaluating expression")) }
}

fn read(expr: String) -> Result<(char, char, char, char)> {
    let mut expr = expr.chars();
    let z = expr.next().context("invalid expr")?;
    let o = expr.next().context("invalid expr")?;
    let x = expr.next().context("invalid expr")?;
    let y = expr.next().context("invalid expr")?;
    Ok((z, o, x, y))
}
