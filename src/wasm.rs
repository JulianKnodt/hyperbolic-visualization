use super::dag::DAG;
use super::map::{self, Mapping};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DAGVisualizer {
    dag: DAG<u32>,
}

#[wasm_bindgen]
impl DAGVisualizer {
    #[wasm_bindgen(constructor)]
    pub fn new(src: &[u32], dst: &[u32]) -> DAGVisualizer {
        let data = src.iter().cloned().zip(dst.iter().cloned());
        let dag = DAG::from_pairs(data);
        Self { dag }
    }

    pub fn coordinates(&self, focus: usize) -> Vec<f64> {
        let coords = super::hyperbolic_project(&self.dag, focus);
        coords.into_iter().flatten().collect()
    }
}

#[wasm_bindgen]
pub struct Maps;

#[wasm_bindgen]
impl Maps {
    #[wasm_bindgen]
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
}
