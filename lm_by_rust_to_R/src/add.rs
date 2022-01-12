use extendr_api::prelude::*;

#[extendr]
fn main() {
  println!("Hello, world!");
}

#[extendr]
fn add(x: f64, y: f64) -> f64 {
  x + y
}

#[extendr]
fn lm(x, y){
  // return coefficients, t-value, and p-value
  let (coefficients, t, p) = linear_regression(x, y);
}

extendr_module! {
  mod rextendr;
  fn add;
  fn main;
}
