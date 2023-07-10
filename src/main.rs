extern crate ndarray as nd;
use nd::{Axis, array, Array, Array1, Array2, Dim, ShapeError, concatenate};

fn get_knot(n_sample: i32, degree: i32) -> Result<Array<f64, Dim<[usize; 1]>>, ShapeError> {
  let a: Array1<f64> = Array::zeros(degree as usize);
  let b: Array1<f64> = Array::linspace(0., 1., (n_sample - 2*degree) as usize);
  let c: Array1<f64> = Array::ones(degree as usize);
  
  concatenate(Axis(0), &[a.view(), b.view(), c.view()])
}

fn basis(knot: &Array1<f64>, j: usize, k: usize, t: f64) -> f64 {
  if k == 0 {
    return ((knot[j] <= t) && (t < knot[j+1])) as i32 as f64;
  } else {
    let mut b = 0.;
    let b1 = basis(&knot, j, k-1, t);
    let b2 = basis(&knot, j+1, k-1, t);
    if b1 != 0. {
      b += (t - knot[j]) / (knot[j+k] - knot[j]) * b1;
    }
    if b2 != 0. {
      b += (knot[j+k+1] - t) / (knot[j+k+1] - knot[j+1]) * b2;
    }
    return b;
  }
}

fn sum(x: &Array2<f64>, t: f64) -> Array1<f64> {
  let knot: Array1<f64> = get_knot(9, 3).unwrap();
  let mut s: Array1<f64> = array![0., 0.];
  
  for i in 0..5 {
    let b = basis(&knot, i, 3, t);
    s[0] += x[[i, 0]] * b;
    s[1] += x[[i, 1]] * b;
  }

  s
}


fn main() {
  let x: Array2<f64> = array![[1., 1.], [2., 3.], [4., 1.], [3., -1.], [2., -2.]];
  let t_: Array1<f64> = Array::linspace(0., 0.999, 100);
  // let mut s_: Array2<f64> = array![[]];

  for t in t_ {
    println!("{}", sum(&x, t));
  }
}