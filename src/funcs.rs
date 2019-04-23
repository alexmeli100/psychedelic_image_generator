use std::f64;
use rand::seq::{SliceRandom};
use rand::{Rng, thread_rng};

enum Funcs {
    SIN,
    COS,
    TIMES
}

enum Vals {
    XVAL,
    YVAL
}

pub struct X;
pub struct Y;

pub trait EvalFunc {
    fn eval(&self, x: f64, y: f64) -> f64;
}

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
    arg: Box<EvalFunc>,
}

pub struct CosFunc {
    arg: Box<EvalFunc>,
}

pub struct Times {
    lhs: Box<EvalFunc>,
    rhs: Box<EvalFunc>
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

impl<'a> CosFunc {
    fn new(prob: f64) -> CosFunc {
        CosFunc{arg: build_expr(prob * prob)}
    }
}

impl EvalFunc for CosFunc {
    fn eval(&self, x: f64, y: f64) -> f64 {
        f64::cos(f64::consts::PI * self.arg.eval(x, y))
    }
}

impl  Times {
    fn new(prob: f64) -> Times {
        Times {
            lhs: build_expr(prob * prob),
            rhs: build_expr(prob * prob)
        }
    }
}

impl EvalFunc for Times {
    fn eval(&self, x: f64, y: f64) -> f64 {
        self.lhs.eval(x, y) * self.rhs.eval(x, y)
    }
}

pub fn build_expr(prob: f64) -> Box<EvalFunc> {
    let rng_func = vec![Funcs::SIN, Funcs::COS, Funcs::TIMES];
    let rng_val = vec![Vals::XVAL, Vals::YVAL];


    if thread_rng().gen_range(0.0, 1.0) < prob {
        match rng_func.choose(&mut thread_rng()).unwrap() {
            Funcs::SIN => Box::new(SinFunc::new(prob)),
            Funcs::COS => Box::new(CosFunc::new(prob)),
            Funcs::TIMES => Box::new(Times::new(prob))
        }
    } else {
        match rng_val.choose(&mut thread_rng()).unwrap() {
            Vals::XVAL => Box::new(X),
            Vals::YVAL => Box::new(Y)
        }
    }
}