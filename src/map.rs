// Functions which map coordinates on a disk to a square and vice-versa
//
// Implemented based off of
// Analytical Methods for Squaring the Disc:
// https://arxiv.org/pdf/1509.06344.pdf

pub trait Mapping<FP> {
    fn circle_to_square(v: [FP; 2]) -> [FP; 2];
    fn square_to_circle(v: [FP; 2]) -> [FP; 2];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimpleStretching;

macro_rules! impl_mapping {
  (
  impl Mapping<$( $fp: ty $(,)?)+> for $name: ty {
  |$x: ident, $y: ident| $to_uv: expr,

  |$u: ident, $v: ident| $to_xy:expr $(,)?
  }) => {$(
    impl Mapping<$fp> for $name {
      fn square_to_circle([$x,$y]: [$fp; 2]) -> [$fp; 2] {
        $to_uv
      }
      fn circle_to_square([$u,$v]: [$fp; 2]) -> [$fp; 2] {
        $to_xy
      }
    }
  )+}
}

impl_mapping!(
  impl Mapping<f32, f64> for SimpleStretching {
    |x,y| {
      let x2 = x*x;
      let y2 = y*y;
      let k = 1.0/(x2+y2).sqrt();
      if x2 > y2 {
        let xs = x.signum();
        [xs * x2 * k, xs * x*y * k]
      } else {
        let ys = y.signum();
        [ys * x*y * k, ys * y2 * k]
      }
    },
    |u,v| {
      let u2 = u*u;
      let v2 = v*v;
      let k = (u2+v2).sqrt();
      let safe_div = |a,b| if b == 0. {
        0.
      } else {
        a/b
      };
      if u2 >= v2 {
        let us = u.signum();
        [us * k, us * safe_div(v, u) * k]
      } else {
        let vs = v.signum();
        [vs * safe_div(u, v) * k, vs * k]
      }
    },
  }
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipticalGrid;

impl_mapping!(
  impl Mapping<f32, f64> for EllipticalGrid {
    |x,y| [x*(1.-(y*y)/2.).sqrt(), y*(1.-(x*x)/2.).sqrt()],
    |u,v| {
      let u2 = u*u;
      let v2 = v*v;
      let u2rt2 = (u2 * 8.).sqrt().copysign(u);
      let v2rt2 = (v2 * 8.).sqrt().copysign(v);
      [
        0.5 * ((2.+u2-v2+u2rt2).sqrt()-(2.+u2-v2-u2rt2).sqrt()),
        0.5 * ((2.-u2+v2+v2rt2).sqrt()-(2.-u2+v2-v2rt2).sqrt()),
      ]
    },
  }
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FGSquircular;

impl_mapping!(
  impl Mapping<f32, f64> for FGSquircular {
    |x,y| {
      let x2 = x * x;
      let y2 = y * y;
      let k = (x2 + y2 - x2 * y2).sqrt()/(x2 + y2).sqrt();
      [x * k, y * k]
    },

    |u,v| {
      let u2 = u*u;
      let v2 = v*v;
      let sum = u2+v2;
      let sign_uv = u.signum() * v.signum();

      let k = sign_uv * (sum - (sum * (sum - 4. * u2 * v2)).sqrt()).sqrt();
      let safe_div = |a,b| if b == 0. { 0. } else { a / b };
      [
        safe_div(k, (2. * v2).sqrt().copysign(v)),
        safe_div(k, (2. * u2).sqrt().copysign(u)),
      ]
    },
  }
);
