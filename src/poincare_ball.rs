use std::array;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct PoincarePoint<const N: usize = 2>([f64; N]);

/// Computes the norm of a vector.
fn sqr_norm(v: &[f64]) -> f64 {
    v.iter().map(|v| v * v).sum::<f64>()
}

fn norm(v: &[f64]) -> f64 {
    sqr_norm(v).sqrt()
}

fn inner_prod<const N: usize>(l: &[f64; N], r: &[f64; N]) -> f64 {
    l.iter().zip(r.iter()).map(|(a, b)| a * b).sum()
}

fn kmul<const N: usize>(k: f64, v: &[f64; N]) -> [f64; N] {
    v.map(|v| k * v)
}
fn kdiv<const N: usize>(v: &[f64; N], k: f64) -> [f64; N] {
    v.map(|v| v / k)
}

impl<const N: usize> PoincarePoint<N> {
    pub fn from_raw(v: &[f64; N]) -> Self {
        assert!(
            norm(v.as_slice()) <= 1.0,
            "Cannot convert points outside of Poincare ball (||v|| > 1)"
        );
        Self(v.clone())
    }
    /// Convert points from euclidean space into a point on the Poincare Ball.
    pub fn exp(v: &[f64; N]) -> Self {
        let v_norm = norm(v);
        let k = v_norm.tanh() / v_norm;
        Self(v.map(|v| v * k))
    }
    pub fn log(&self) -> [f64; N] {
        let norm = norm(&self.0);
        let k = norm.atanh() / norm;
        self.0.map(|v| v * k)
    }

    pub fn left_k_mobius_mul(&self, k: f64) -> Self {
        Self::exp(&self.log().map(|v| k * v))
    }

    pub fn mobius_add(&self, o: &Self) -> Self {
        let ip = inner_prod(&self.0, &o.0);
        let x_norm = sqr_norm(&self.0);
        let y_norm = sqr_norm(&o.0);
        let numer = add(
            &kmul(1.0 + 2.0 * ip + y_norm, &self.0),
            &kmul(1.0 - x_norm, &o.0),
        );
        let denom = 1.0 + 2.0 * ip + x_norm * y_norm;
        Self::from_raw(&kdiv(&numer, denom))
    }
    pub fn mobius_sub(&self, o: &Self) -> Self {
        self.mobius_add(&o.neg())
    }
    pub fn neg(&self) -> Self {
        Self(self.0.map(|v| -v))
    }
    pub fn dist(&self, o: &Self) -> f64 {
        let x_norm = sqr_norm(&self.0);
        let y_norm = sqr_norm(&o.0);
        let k = sqr_norm(&sub(&self.0, &o.0)) / ((1.0 - x_norm) * (1.0 - y_norm));
        (1.0 + 2. * k).acosh()
    }

    pub fn is_valid(&self) -> bool {
        norm(&self.0) <= 1.
    }
    pub const fn zero() -> Self {
        Self([0.; N])
    }
}

macro_rules! create_elemwise_fn {
  ($name: ident, $op: tt) => {
    pub fn $name<const N: usize>(l: &[f64; N], r: &[f64; N]) -> [f64; N] {
      array::from_fn(|i| l[i] $op r[i])
    }
  }
}

create_elemwise_fn!(add, +);
create_elemwise_fn!(sub, -);

#[test]
fn test_mobius() {
    let f = PoincarePoint::from_raw(&[0.5, 0.5]);
    let zero = PoincarePoint::zero();
    assert_eq!(f, f.mobius_add(&zero));
}
