#![allow(incomplete_features)]
#![feature(adt_const_params)]
// Ignore non upper globals.
#![allow(non_upper_case_globals)]

use std::hash::Hash;

pub mod dag;
pub use dag::{TraversalOrder, DAG, DAGID};

pub mod poincare_ball;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AngleRange {
    /// Starting degree of angle range
    start: f64,
    /// Number of degrees in angle range
    size: f64,
}

impl AngleRange {
    /// Checks if this location has been modified from the default from the default;
    pub fn is_empty(&self) -> bool {
        self.start == 0. && self.size == 0.
    }
}

/// Projects a tree into hyperbolic space, returning the set of coordinates at which each point
/// is mapped to.
pub fn hyperbolic_project<T>(dag: &DAG<T>, focus: DAGID) -> Vec<[f64; 2]>
where
    T: Hash + Eq,
{
    let info: Vec<_> = dag
        .depth_first_iter::<{ TraversalOrder::PreOrder }>(focus)
        .collect();

    let mut angles = vec![AngleRange::default(); info.len()];

    for v in &info {
        if v.dagid == focus {
            angles[focus] = AngleRange {
                start: 0.,
                size: 360.,
            };
            continue;
        }
        let (parent, child_num) = v.parent_ref.unwrap();

        let parent_range = angles[parent];
        assert!(!angles[parent].is_empty(), "previous node was empty {:?}", &angles);

        let child_num = child_num as f64;
        let total_segments = dag.neighbors(parent).len();
        let segment_size = parent_range.size / (total_segments as f64);

        let start = parent_range.start + segment_size * child_num;
        let size = segment_size;
        angles[v.dagid] = AngleRange { start, size };
    }

    /*
    for v in &info {
      println!("{:?} : {:?}", &angles[v.dagid], v.dagid);
    }
    */

    let mut final_locations = vec![[0.; 2]; angles.len()];
    for v in &info {
        let depth = v.depth;
        if depth == 0 {
            // root is at center.
            continue;
        }
        let radius = 1.0 - (0.5f64).powi(depth as i32);
        let angle = angles[v.dagid].start + (angles[v.dagid].size / 2.0);
        let (sin, cos) = angle.to_radians().sin_cos();
        final_locations[v.dagid] = [radius * cos, radius * sin];
    }
    final_locations
}

#[test]
fn test_simple_tree() {
    let graph = [(0, 1), (0, 2), (0, 3), (1, 4), (1, 5), (5, 6), (5, 7)];
    let dag = DAG::from_pairs(graph);
    let points = hyperbolic_project(&dag, 0);
    for [x,y] in &points {
      println!("{:.2?}", (x,y));
    }
}
