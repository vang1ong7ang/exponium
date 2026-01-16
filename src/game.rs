use crate::Tact;
use rug::Float;
use std::iter::from_fn;
use std::iter::once;
#[derive(Debug, Clone)]
pub struct Game {
    rate: Float,
    cost: Float,
    prin: Float,
    time: Float,
}
impl Game {
    pub fn new(prec: u32, rate: Float, cost: Float) -> Self {
        let rate = Float::with_val(prec, rate);
        let cost = Float::with_val(prec, cost);
        let prin = Float::with_val(prec, 1);
        let time = Float::with_val(prec, 0);
        Game { rate, cost, prin, time }
    }
    pub fn comp(&self, tact_x: &Tact, tact_y: &Tact, mut limit: usize) -> Option<bool> {
        let mut itx = self.clone().tact(tact_x);
        let mut ity = self.clone().tact(tact_y);
        let mut tx = self.time().clone();
        let mut ty = self.time().clone();
        let mut px = self.prin().clone();
        let mut py = self.prin().clone();
        let mut ft = tx < ty;
        let mut fp = px < py;
        while ft == fp && limit > 0 {
            limit -= 1;
            if ft {
                (tx, px) = itx.next().unwrap();
            } else {
                (ty, py) = ity.next().unwrap();
            }
            ft = tx < ty;
            fp = px < py;
        }
        if limit == 0 { None } else { Some(ft) }
    }
    pub fn tact(mut self, tact: &Tact) -> impl Iterator<Item = (Float, Float)> {
        once((self.time.clone(), self.prin.clone())).chain(from_fn(move || {
            self.harv(tact.calc(self.rate(), self.cost(), self.prin()));
            Some((self.time.clone(), self.prin.clone()))
        }))
    }
    pub fn harv(&mut self, wait: Float) {
        self.time += &wait;
        self.prin += wait * self.rate() * self.prin() - self.cost();
    }
    pub fn gain(&self, wait: Float) -> Float {
        wait * self.rate() * self.prin()
    }
    pub fn rate(&self) -> &Float {
        &self.rate
    }
    pub fn cost(&self) -> &Float {
        &self.cost
    }
    pub fn prin(&self) -> &Float {
        &self.prin
    }
    pub fn time(&self) -> &Float {
        &self.time
    }
}
