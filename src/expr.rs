use crate::Game;
use rug::Float;
use rug::ops::Pow;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Expr {
    code: Vec<(String, String, String, String)>,
}
impl Expr {
    pub fn term(&self, prec: u32, rate: Float, cost: Float, term: Float) -> Option<Float> {
        let mut game = Game::new(prec, rate, cost);
        while game.prin() < &term {
            game.harv(self.eval(game.prin().clone(), game.rate().clone(), game.cost().clone())?);
        }
        Some(game.time().clone())
    }
    pub fn eval(&self, prin: Float, rate: Float, cost: Float) -> Option<Float> {
        let mut mem = HashMap::new();
        mem.insert("prin", prin);
        mem.insert("rate", rate);
        mem.insert("cost", cost);
        for (z, o, x, y) in &self.code {
            let x = mem.get(x.as_str())?.clone();
            let y = mem.get(y.as_str())?.clone();
            let z = z.as_str();
            if mem.contains_key(z) {
                return None;
            }
            match o.as_str() {
                "add" => {
                    mem.insert(z, x + y);
                }
                "mul" => {
                    mem.insert(z, x * y);
                }
                "log" => {
                    mem.insert(z, x.ln() / y.ln());
                }
                "pow" => {
                    mem.insert(z, x.pow(y));
                }
                _ => return None,
            }
        }
        mem.get("wait").cloned()
    }
}
impl From<Vec<(String, String, String, String)>> for Expr {
    fn from(code: Vec<(String, String, String, String)>) -> Self {
        Expr { code }
    }
}
impl From<Expr> for Vec<(String, String, String, String)> {
    fn from(expr: Expr) -> Self {
        expr.code
    }
}
