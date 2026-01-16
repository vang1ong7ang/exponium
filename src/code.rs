use rug::Float;
#[derive(Debug, Clone)]
pub enum Code {
    Nope,
    Set { dst: usize, val: Float },
    Add { dst: usize, x: usize, y: usize },
    Mul { dst: usize, x: usize, y: usize },
    Sqrt { dst: usize, x: usize },
}

impl Code {
    pub fn eval(&self, prec: u32, mem: &mut [Float]) {
        match self {
            Code::Nope => (),
            Code::Set { dst, val } => mem[*dst] = Float::with_val(prec, val),
            Code::Add { dst, x, y } => mem[*dst] = Float::with_val(prec, &mem[*x] + &mem[*y]),
            Code::Mul { dst, x, y } => mem[*dst] = Float::with_val(prec, &mem[*x] * &mem[*y]),
            Code::Sqrt { dst, x } => mem[*dst] = Float::with_val(prec, mem[*x].sqrt_ref()),
        }
    }
}
