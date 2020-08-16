use std::f64;
use rand::seq::{SliceRandom};
use rand::{Rng, thread_rng};

#[derive(Copy, Clone)]
enum Funcs {
    SinFunc,
    CosFunc,
    TimesFunc,
    X,
    Y
}

pub trait EvalFunc {
    fn eval(&self, x: f64, y: f64) -> f64;
}

impl From<(Funcs, f64)> for Box<dyn EvalFunc> {
    fn from(t: (Funcs, f64)) -> Self {
        match t.0 {
            Funcs::SinFunc => Box::new(SinFunc::new(t.1)),
            Funcs::CosFunc=> Box::new(CosFunc::new(t.1)),
            Funcs::TimesFunc => Box::new(TimesFunc::new(t.1)),
            Funcs::X => Box::new(X),
            Funcs::Y => Box::new(Y)
        }
    }
}

pub struct X;
pub struct Y;

impl EvalFunc for X {
    fn eval(&self, x: f64, _y: f64) -> f64 {
        x
    }
}

impl EvalFunc for Y {
    fn eval(&self, _x: f64, y: f64) -> f64 {
        y
    }
}

pub struct SinFunc {
    arg: Box<dyn EvalFunc>,
}

pub struct CosFunc {
    arg: Box<dyn EvalFunc>,
}

pub struct TimesFunc {
    lhs: Box<dyn EvalFunc>,
    rhs: Box<dyn EvalFunc>
}

impl SinFunc {
    fn new(prob: f64) -> SinFunc {
        SinFunc{arg: build_expr(prob * prob)}
    }
}

impl EvalFunc for SinFunc {
    fn eval(&self, x: f64, y: f64) -> f64 {
        f64::sin(f64::consts::PI * self.arg.eval(x, y))
    }
}

impl CosFunc {
    fn new(prob: f64) -> CosFunc {
        CosFunc{arg: build_expr(prob * prob)}
    }
}

impl EvalFunc for CosFunc {
    fn eval(&self, x: f64, y: f64) -> f64 {
        f64::cos(f64::consts::PI * self.arg.eval(x, y))
    }
}

impl  TimesFunc {
    fn new(prob: f64) -> TimesFunc {
        TimesFunc {
            lhs: build_expr(prob * prob),
            rhs: build_expr(prob * prob)
        }
    }
}

impl EvalFunc for TimesFunc {
    fn eval(&self, x: f64, y: f64) -> f64 {
        self.lhs.eval(x, y) * self.rhs.eval(x, y)
    }
}

pub fn build_expr(prob: f64) -> Box<dyn EvalFunc> {
    let rng_func = vec![Funcs::SinFunc, Funcs::CosFunc, Funcs::TimesFunc];
    let rng_val = vec![Funcs::X, Funcs::Y];

    let f;
    if thread_rng().gen_range(0.0, 1.0) < prob {
        f = rng_func.choose(&mut thread_rng()).unwrap();
    } else {
        f = rng_val.choose(&mut thread_rng()).unwrap();
    }

    (*f, prob).into()
}