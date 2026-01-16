use crate::Code;
pub use rug::Float;
pub struct Tact<const N: usize> {
    expr: Vec<Code>,
}
impl<const N: usize> Tact<N> {
    pub fn calc(&self, rate: &Float, cost: &Float, prin: &Float) -> Float {
        let prec = prin.prec();
        let mut mem = (0..1 << N).map(|_| Float::new(prec)).collect::<Vec<Float>>();
        mem[0] += 1;
        mem[1] += rate;
        mem[2] += cost;
        mem[3] += prin;
        for code in &self.expr {
            code.eval(prec, &mut mem);
        }
        mem[0].clone()
    }
}
