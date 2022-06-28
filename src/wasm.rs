use super::dag::DAG;
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
pub fn convert(u: u32) -> f32 {
    (u as f32).sqrt()
}
