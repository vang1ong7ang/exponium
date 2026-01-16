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
    let mut speed = Float::with_val(prec, 1.0);
    let mut waiting = Float::with_val(prec, 0.0);
    loop {
        if poll(Duration::from_millis(10))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => {
                            speed *= 2.0;
                        }
                        KeyCode::Down => {
                            speed /= 2.0;
                        }
                        KeyCode::Enter => {
                            game.harvest(waiting.clone());
                            waiting = Float::with_val(prec, 0.0);
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
        waiting += &speed;
        out.execute(MoveTo(0, 0))?;
        writeln!(out, "Last Harvest Time: {}", game.time())?;
        out.execute(MoveTo(0, 1))?;
        writeln!(out, "Time Since Last Harvest: {}", waiting)?;
        out.execute(MoveTo(0, 2))?;
        writeln!(out, "Principal: {}", game.prin())?;
        out.execute(MoveTo(0, 3))?;
        writeln!(out, "Unrealized Gain: {}", game.gain(waiting.clone()))?;
        out.execute(MoveTo(0, 4))?;
        writeln!(out, "Nominal Interest Rate: {}", game.rate())?;
        out.execute(MoveTo(0, 5))?;
        writeln!(out, "Harvest Cost: {}", game.cost())?;
        out.execute(MoveTo(0, 6))?;
        writeln!(out, "<Enter>: Harvest; <Esc>: Exit; <Up>: Increase Speed; <Down>: Decrease Speed")?;
    }
}
