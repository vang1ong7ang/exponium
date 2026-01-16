use rug::Float;
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
    pub fn harvest(&mut self, wait: Float) {
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
