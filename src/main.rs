use crossterm::ExecutableCommand;
use crossterm::cursor::Hide;
use crossterm::cursor::MoveTo;
use crossterm::cursor::Show;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use exponium::Game;
use rug::Float;
use std::io::Result;
use std::io::Write;
use std::io::stdout;
use std::time::Duration;
fn main() -> Result<()> {
    let prec = 128;
    let rate = Float::with_val(prec, 1.0);
    let cost = Float::with_val(prec, 1.0);
    let mut game = Game::new(prec, rate, cost);
    enable_raw_mode()?;
    let mut out = stdout();
    out.execute(EnterAlternateScreen)?;
    out.execute(Hide)?;
    let mut freq = Float::with_val(prec, 1.0);
    let mut step = Float::with_val(prec, 0.0);
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
                            step = Float::with_val(prec, 0.0);
                        }
                        KeyCode::Esc => {
                            out.execute(Show)?;
                            out.execute(LeaveAlternateScreen)?;
                            break disable_raw_mode();
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
            out.execute(MoveTo(0, i as u16))?;
            writeln!(out, "{}: {}", head[i], data[i])?;
        }
        out.execute(MoveTo(0, head.len() as u16))?;
        writeln!(out, "\r\n<Enter>: Harvest\r\n<Esc>: Exit\r\n<Up>: Speed +\r\n<Down>: Speed -")?;
    }
}
