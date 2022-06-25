#![allow(incomplete_features)]
#![feature(adt_const_params)]
// Ignore non upper globals.
#![allow(non_upper_case_globals)]

pub mod dag;
pub use dag::{DAG, DAGID, TraversalOrder};

/// Projects a tree into hyperbolic space, returning the set of coordinates at which each point
/// is mapped to.
pub fn hyperbolic_project<T>(dag: &DAG<T>, focus: DAGID) -> Vec<(DAGID, (f64, f64))> {
  todo!();
}
