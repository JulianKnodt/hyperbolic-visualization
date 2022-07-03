use std::array;

type FP = f32;
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct PoincarePoint<const N: usize = 2>([FP; N]);

/// Computes the norm of a vector.
fn sqr_norm(v: &[FP]) -> FP {
    v.iter().map(|v| v * v).sum::<FP>()
}

fn norm(v: &[FP]) -> FP {
    sqr_norm(v).sqrt()
}

fn inner_prod<const N: usize>(l: &[FP; N], r: &[FP; N]) -> FP {
    l.iter().zip(r.iter()).map(|(a, b)| a * b).sum()
}

fn kmul<const N: usize>(k: FP, v: &[FP; N]) -> [FP; N] {
    v.map(|v| k * v)
}
fn kdiv<const N: usize>(v: &[FP; N], k: FP) -> [FP; N] {
    v.map(|v| v / k)
}

impl<const N: usize> PoincarePoint<N> {
    pub fn from_raw(v: &[FP; N]) -> Self {
        /*
        assert!(
            norm(v.as_slice()) <= 1.0,
            "Cannot convert points outside of Poincare ball (||v|| > 1)"
        );
        */
        Self(v.clone())
    }
    pub fn as_slice(&self) -> &[FP; N] {
        &self.0
    }
    /// Convert points from euclidean space into a point on the Poincare Ball.
    pub fn exp(v: &[FP; N]) -> Self {
        let v_norm = norm(v) + 1e-5;
        let k = v_norm.tanh() / v_norm;
        Self(v.map(|v| v * k))
    }
    pub fn log(&self) -> [FP; N] {
        let norm = norm(&self.0);
        let k = norm.atanh() / norm;
        self.0.map(|v| v * k)
    }

    pub fn left_k_mobius_mul(&self, k: FP) -> Self {
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
    pub fn dist(&self, o: &Self) -> FP {
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

impl PoincarePoint<2> {
    /// Perform rotation in poincare ball
    pub fn mobius_rotate(&self, theta: FP) -> Self {
        let [u, v] = self.0;
        let sinh = theta.sinh();
        let cosh = theta.cosh();
        Self([cosh * u + sinh * v, -sinh * u + cosh * v])
    }
    /// Perform euclidean rotation of entire poincare ball
    pub fn rotate(&self, theta: FP) -> Self {
        let [u, v] = self.0;
        let (sin, cos) = theta.sin_cos();
        Self([cos * u + sin * v, - sin * u + cos * v])
    }
}

macro_rules! create_elemwise_fn {
  ($name: ident, $op: tt) => {
    pub fn $name<const N: usize>(l: &[FP; N], r: &[FP; N]) -> [FP; N] {
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
