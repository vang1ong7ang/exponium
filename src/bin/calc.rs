use anyhow::Result;
use clap::Parser;
use rug::Float;
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = 256)]
    prec: u32,
    #[arg(long, default_value = "1")]
    x: String,
    #[arg(long, default_value = "1")]
    y: String,
    #[arg(long, default_value_t = 10)]
    base: i32,
    #[arg(long, default_value = "add")]
    code: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let prec = args.prec;
    let x = Float::with_val(prec, Float::parse_radix(&args.x, args.base)?);
    let y = Float::with_val(prec, Float::parse_radix(&args.y, args.base)?);
    match args.code.as_str() {
        "add" => {
            println!("{}", (x + y).to_string_radix(args.base, None));
        }
        "sub" => {
            println!("{}", (x - y).to_string_radix(args.base, None));
        }
        "mul" => {
            println!("{}", (x * y).to_string_radix(args.base, None));
        }
        "div" => {
            println!("{}", (x / y).to_string_radix(args.base, None));
        }
        _ => {
            eprintln!("Unknown operation: {}", args.code);
        }
    }
    Ok(())
}
