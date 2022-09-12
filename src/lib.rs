#![allow(incomplete_features)]
#![feature(adt_const_params, array_chunks)]
// Ignore non upper globals.
#![allow(non_upper_case_globals)]

use std::hash::Hash;

pub mod dag;
pub use dag::{DFOut, TraversalOrder, DAG, DAGID};

pub mod map;
pub mod poincare_ball;
pub use poincare_ball::PoincarePoint;

// TODO feature gate this under wasm
pub mod wasm;

pub type FP = f64;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct AngleRange {
    /// Starting degree of angle range
    start: FP,
    /// Number of degrees in angle range
    size: FP,
}

impl AngleRange {
    /// Checks if this location has been modified from the default from the default;
    pub fn is_empty(&self) -> bool {
        self.start == 0. && self.size == 0.
    }
}

/// Projects a tree into hyperbolic space, returning the set of coordinates at which each point
/// is mapped to, as well as the connectivity.
pub fn hyperbolic_project<T>(
    dag: &DAG<T>,
    focus: DAGID,
) -> (Vec<[FP; 2]>, impl Iterator<Item = DFOut>)
where
    T: Hash + Eq,
{
    let mut info: Vec<_> = vec![Default::default(); dag.num_nodes()];
    let mut order = vec![];
    dag.breadth_first_visit(focus, |dfout| {
        info[dfout.dagid] = dfout;
        order.push(dfout.dagid);
    });

    let mut angles = vec![AngleRange::default(); dag.num_nodes()];
    let mut max_depth = 0;

    for &o in &order {
        let v = &info[o];
        if v.dagid == focus {
            angles[v.dagid] = AngleRange {
                start: 0.,
                size: 360.,
            };
            continue;
        }
        let (parent, child_num) = v.parent_ref.unwrap();

        let parent_range = angles[parent];
        assert!(
            !angles[parent].is_empty(),
            "previous node of Node({:?}) was empty",
            v,
        );

        let child_num = child_num as FP;
        let total_segments = 1 + dag.neighbors(parent).len();
        assert_ne!(total_segments, 0);
        let segment_size = parent_range.size / (total_segments as FP);

        let start = parent_range.start + segment_size * child_num;
        let size = segment_size;
        max_depth = max_depth.max(v.depth);
        angles[v.dagid] = AngleRange { start, size };
    }

    let mut final_locations = vec![[0.; 2]; angles.len()];
    for &o in &order {
        let v = &info[o];
        let depth = v.depth;
        if depth == 0 {
            // root is at center.
            continue;
        }
        let radius = 0.98 * (depth as FP / max_depth as FP);
        let angle = angles[v.dagid].start + (angles[v.dagid].size / 2.0);
        let (sin, cos) = angle.to_radians().sin_cos();
        final_locations[v.dagid] = [radius * cos, radius * sin];
    }
    (final_locations, order.into_iter().map(move |o| info[o]))
}

#[test]
fn test_simple_tree() {
    let graph = [(0, 1), (0, 2), (0, 3), (1, 4), (1, 5), (5, 6), (5, 7)];
    let dag = DAG::from_pairs(graph);
    let points = hyperbolic_project(&dag, 0).0;
    for [x, y] in &points {
        println!("{:.2?}", (x, y));
    }
}

#[test]
fn test_cit_data() {
    use std::fs::read_to_string;
    use std::path::Path;
    let path = Path::new(file!())
        .parent()
        .and_then(|p| p.parent())
        .unwrap()
        .join("app/cit-DBLP.edges");
    let data = read_to_string(path).unwrap();
    let lines: Vec<_> = data
        .lines()
        .map(|l| {
            let mut items = l.split_whitespace().map(|e| e.parse::<u32>().unwrap());
            let src = items.next().unwrap();
            (src, items.next().unwrap())
        })
        .collect();
    let dag = DAG::from_pairs(lines);
    let _ = hyperbolic_project(&dag, 0);
}
