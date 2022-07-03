use super::dag::{DFOut, DAG};
use super::map::{self, Mapping};
use super::poincare_ball::PoincarePoint;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DAGVisualizer {
    dag: DAG<u32>,

    info: Vec<DFOut>,
}

#[wasm_bindgen]
impl DAGVisualizer {
    #[wasm_bindgen(constructor)]
    pub fn new(src: &[u32], dst: &[u32]) -> DAGVisualizer {
        let data = src.iter().cloned().zip(dst.iter().cloned());
        let dag = DAG::from_pairs(data);
        Self { dag, info: vec![] }
    }

    /// Returns a flattened vector of Vec<[f64;2]> coordinates of tree elements
    pub fn coordinates(&mut self, focus: usize) -> Vec<f64> {
        let (coords, info) = super::hyperbolic_project(&self.dag, focus);
        self.info = info.collect();
        coords.into_iter().flatten().collect()
    }

    /// Returns a flattened vector of `parent` -> `child` connections.
    pub fn connectivity(&self) -> Vec<usize> {
        self.info
            .iter()
            .filter_map(|dfout| dfout.parent_ref.map(|(p, _)| [p, dfout.dagid]))
            .flatten()
            .collect()
    }
}

#[wasm_bindgen]
pub struct Maps;

#[wasm_bindgen]
impl Maps {
    pub fn circle_to_square(us: &[f32], vs: &[f32], method: &str) -> Vec<f32> {
        assert_eq!(us.len(), vs.len(), "Length mismatch in us and vs");
        let mapping: Box<dyn Fn([f32; 2]) -> [f32; 2]> = match method {
            "simple" => Box::new(map::SimpleStretching::circle_to_square),
            "elliptical" => Box::new(map::EllipticalGrid::circle_to_square),
            "squircular" => Box::new(map::FGSquircular::circle_to_square),
            x => panic!("unknown mapping method {:?}", x),
        };
        us.iter()
            .zip(vs.iter())
            .map(|(&u, &v)| mapping([u, v]))
            .flatten()
            .collect()
    }

    pub fn shift(uvs: &[f32], x: f32, y: f32, r: f32) -> Vec<f32> {
        let shift = PoincarePoint::exp(&[x, y]);
        uvs.array_chunks::<2>()
            .map(|pt| PoincarePoint::from_raw(pt))
            .map(|pt| pt.mobius_add(&shift))
            .map(|pt| pt.rotate(r.to_radians()))
            .map(|pt| {
                let &[u, v] = pt.as_slice();
                [u, v]
            })
            .flatten()
            .collect()
    }
}
