use crate::Game;
use rug::Float;
use rug::ops::Pow;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Expr {
    code: Vec<(char, char, char, char)>,
}
impl Expr {
    pub fn term<const N: usize>(&self, prec: u32, rate: Float, cost: Float, term: Float) -> Option<Float> {
        let game = (0..N).fold(Some(Game::new(prec, rate, cost)), |game, _| {
            if let Some(mut game) = game {
                if game.prin() < &term {
                    game.harv(self.eval(game.prin().clone(), game.rate().clone(), game.cost().clone())?)
                }
                Some(game)
            } else {
                None
            }
        })?;
        if game.prin() < &term { None } else { Some(game.time().clone()) }
    }
    pub fn eval(&self, prin: Float, rate: Float, cost: Float) -> Option<Float> {
        let mut mem = HashMap::new();
        mem.insert('P', prin);
        mem.insert('R', rate);
        mem.insert('C', cost);
        for (z, o, x, y) in &self.code {
            let x = mem.get(x)?.clone();
            let y = mem.get(y)?.clone();
            if mem.contains_key(z) {
                return None;
            }
            match o {
                '+' => {
                    mem.insert(*z, x + y);
                }
                '*' => {
                    mem.insert(*z, x * y);
                }
                'v' => {
                    mem.insert(*z, y.ln() / x.ln());
                }
                '^' => {
                    mem.insert(*z, x.pow(y));
                }
                _ => return None,
            }
        }
        mem.get(&'T').cloned()
    }
}
impl From<Vec<(char, char, char, char)>> for Expr {
    fn from(code: Vec<(char, char, char, char)>) -> Self {
        Expr { code }
    }
}
impl From<Expr> for Vec<(char, char, char, char)> {
    fn from(expr: Expr) -> Self {
        expr.code
    }
}
