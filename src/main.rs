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
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_reader;
use serde_json::to_writer_pretty;
use std::env::var;
use std::fs::File;
use std::io::Result;
use std::io::stdout;
use std::time::Duration;
#[derive(Debug, Serialize, Deserialize)]
struct Exponium {
    prec: u32,
    rate: Float,
    cost: Float,
}
impl Default for Exponium {
    fn default() -> Self {
        let prec = 256;
        let rate = Float::with_val(prec, 1.0);
        let cost = Float::with_val(prec, 65536);
        Self { prec, rate, cost }
    }
}
fn main() -> Result<()> {
    let conf = read_exponium_config()?;
    let game = Game::new(conf.prec, &conf.rate, &conf.cost);
    let freq = Float::with_val(conf.prec, 1.0);
    let step = Float::with_val(conf.prec, 0.0);
    play_terminal_game(game, freq, step)?;
    Ok(())
}

fn read_exponium_config() -> Result<Exponium> {
    if let Ok(cfg) = var("EXPCFG") {
        if let Ok(file) = File::open(&cfg) {
            return Ok(from_reader(file)?);
        }
        to_writer_pretty(File::create(cfg)?, &Exponium::default())?;
    }
    Ok(Default::default())
}

fn play_terminal_game(mut game: Game, mut freq: Float, mut step: Float) -> Result<Vec<Float>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    let mut hist = Vec::new();
    loop {
        if poll(Duration::from_millis(10))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => {
                            freq *= 2.0;
                        }
                        KeyCode::Down => {
                            freq /= 2.0;
                        }
                        KeyCode::Enter => {
                            game.harv(step.clone());
                            hist.push(step.clone());
                            step *= 0;
                        }
                        KeyCode::Esc => {
                            execute!(stdout(), Show, LeaveAlternateScreen)?;
                            disable_raw_mode()?;
                            break Ok(hist);
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
            println!("{}: {}", head[i], data[i]);
        }
        execute!(stdout(), MoveTo(0, head.len() as u16))?;
        println!("\r\n<Enter>: Harvest\r\n<Esc>: Exit\r\n<Up>: Speed +\r\n<Down>: Speed -");
    }
}
