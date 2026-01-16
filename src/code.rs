use rug::Float;
#[derive(Debug, Clone)]
pub enum Code {
    Nope,
    Set { dst: usize, val: Float },
    Add { dst: usize, x: usize, y: usize },
    Mul { dst: usize, x: usize, y: usize },
    Lt { dst: usize, x: usize, y: usize },
    Le { dst: usize, x: usize, y: usize },
    Gt { dst: usize, x: usize, y: usize },
    Ge { dst: usize, x: usize, y: usize },
    Eq { dst: usize, x: usize, y: usize },
    Ne { dst: usize, x: usize, y: usize },
    Sqrt { dst: usize, x: usize },
}

impl Code {
    pub fn eval(&self, prec: u32, mem: &mut [Float]) {
        match self {
            Code::Nope => (),
            Code::Set { dst, val } => mem[*dst] = Float::with_val(prec, val),
            Code::Add { dst, x, y } => mem[*dst] = Float::with_val(prec, &mem[*x] + &mem[*y]),
            Code::Mul { dst, x, y } => mem[*dst] = Float::with_val(prec, &mem[*x] * &mem[*y]),
            Code::Lt { dst, x, y } => mem[*dst] = Float::with_val(prec, if mem[*x] < mem[*y] { 1 } else { 0 }),
            Code::Le { dst, x, y } => mem[*dst] = Float::with_val(prec, if mem[*x] <= mem[*y] { 1 } else { 0 }),
            Code::Gt { dst, x, y } => mem[*dst] = Float::with_val(prec, if mem[*x] > mem[*y] { 1 } else { 0 }),
            Code::Ge { dst, x, y } => mem[*dst] = Float::with_val(prec, if mem[*x] >= mem[*y] { 1 } else { 0 }),
            Code::Eq { dst, x, y } => mem[*dst] = Float::with_val(prec, if mem[*x] == mem[*y] { 1 } else { 0 }),
            Code::Ne { dst, x, y } => mem[*dst] = Float::with_val(prec, if mem[*x] != mem[*y] { 1 } else { 0 }),
            Code::Sqrt { dst, x } => mem[*dst] = Float::with_val(prec, mem[*x].sqrt_ref()),
        }
    }
}
