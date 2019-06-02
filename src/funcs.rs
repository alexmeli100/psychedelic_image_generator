use std::f64;
use rand::seq::{SliceRandom};
use rand::{Rng, thread_rng};

enum Funcs {
    SinFunc,
    CosFunc,
    TimesFunc,
    X,
    Y
}

impl Funcs {
  pub fn from(&self, prob: f64) -> Box<EvalFunc> {
      match self {
          Funcs::SinFunc => Box::new(SinFunc::new(prob)),
          Funcs::CosFunc=> Box::new(CosFunc::new(prob)),
          Funcs::TimesFunc => Box::new(TimesFunc::new(prob)),
          Funcs::X => Box::new(X),
          Funcs::Y => Box::new(Y)
      }
  }  
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

pub struct TimesFunc {
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

pub fn build_expr(prob: f64) -> Box<EvalFunc> {
    let rng_func = vec![Funcs::SinFunc, Funcs::CosFunc, Funcs::TimesFunc];
    let rng_val = vec![Funcs::X, Funcs::Y];


    if thread_rng().gen_range(0.0, 1.0) < prob {
        rng_func.choose(&mut thread_rng()).unwrap().from(prob)
    } else {
        rng_val.choose(&mut thread_rng()).unwrap().from(prob)
    }
}