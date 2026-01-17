use anyhow::Result;
use clap::Parser;
use exponium::Game;
use rug::Float;
use std::io::BufRead;
use std::io::stdin;
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = 256)]
    prec: u32,
    #[arg(long, default_value = "1.0")]
    rate: String,
    #[arg(long, default_value = "65536")]
    cost: String,
    #[arg(long, default_value_t = true)]
    repl: bool,
    #[arg(long, default_value_t = true)]
    prpt: bool,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let prec = args.prec;
    let rate = Float::with_val(prec, Float::parse(args.rate)?);
    let cost = Float::with_val(prec, Float::parse(args.cost)?);
    eprintln!("NOMINAL INTERST RATE | HARVEST COST");
    eprintln!("{} | {}", rate, cost);
    let mut game = Game::new(prec, rate, cost);
    if args.repl {
        eprintln!("TIME | PRINCIPAL");
        eprintln!("{} | {}", game.time(), game.prin());
    }
    if args.prpt {
        eprint!(">>> ");
    }
    for line in stdin().lock().lines() {
        match Float::parse(line?) {
            Ok(line) => {
                let wait = Float::with_val(prec, line);
                game.harv(wait);
                if args.repl {
                    eprintln!("TIME | PRINCIPAL");
                    eprintln!("{} | {}", game.time(), game.prin());
                }
                if args.prpt {
                    eprint!(">>> ");
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                if args.prpt {
                    eprint!(">>> ");
                }
            }
        }
    }
    if args.prpt {
        eprintln!("");
    }
    eprintln!("TIME | PRINCIPAL");
    println!("{} {}", game.time(), game.prin());
    Ok(())
}
