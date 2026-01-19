use anyhow::Result;
use clap::Parser;
use crossterm::cursor::Hide;
use crossterm::cursor::MoveTo;
use crossterm::cursor::Show;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::execute;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use exponium::Game;
use rug::Float;
use std::fs::File;
use std::io::Write;
use std::io::stdout;
use std::time::Duration;
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = 256)]
    prec: u32,
    #[arg(long, default_value = "1.0")]
    rate: String,
    #[arg(long, default_value = "65536")]
    cost: String,
    #[arg(long, default_value_t = 100)]
    poll: u64,
    #[arg(long, default_value_t = 10)]
    base: i32,
    #[arg(long)]
    save: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let prec = args.prec;
    let rate = Float::with_val(prec, Float::parse_radix(args.rate, args.base)?);
    let cost = Float::with_val(prec, Float::parse_radix(args.cost, args.base)?);
    let mut game = Game::new(prec, rate, cost);
    let mut freq = Float::with_val(prec, 1.0);
    let mut step = Float::with_val(prec, 0.0);
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    let mut hist = Vec::new();
    loop {
        if poll(Duration::from_millis(args.poll))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => {
                            freq *= 2.0;
                        }
                        KeyCode::Down => {
                            freq /= 2.0;
                        }
                        KeyCode::Left => {
                            step *= 0;
                        }
                        KeyCode::Enter => {
                            game.harv(step.clone());
                            hist.push(step.clone());
                            step *= 0;
                        }
                        KeyCode::Esc => {
                            if let Some(save) = args.save {
                                let mut file = File::create(save)?;
                                for s in hist {
                                    writeln!(file, "{}", s.to_string_radix(args.base, None))?;
                                }
                            }
                            execute!(stdout(), Show, LeaveAlternateScreen)?;
                            break Ok(disable_raw_mode()?);
                        }
                        _ => {}
                    }
                }
            }
        }
        step += &freq;
        let head = ["TIME", "TOTL", "STEP", "EARN", "PREV", "PRIN", "RATE", "COST", "FREQ"];
        let data = [game.time().clone(), game.gain(step.clone()) + game.prin(), step.clone(), game.gain(step.clone()), game.time().clone(), game.prin().clone(), game.rate().clone(), game.cost().clone(), freq.clone()];
        for i in 0..head.len() {
            execute!(stdout(), MoveTo(0, i as u16))?;
            println!("{}: {}", head[i], data[i].to_string_radix(args.base, None));
        }
        execute!(stdout(), MoveTo(0, head.len() as u16))?;
        println!("\r\n<Enter>: Harvest\r\n<Esc>: Exit\r\n<Up>: Speed +\r\n<Down>: Speed -\r\n<Left>: Reset");
    }
}
