use std::vec::Vec;
// use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use force_graph::{ForceGraph, Node, NodeData};
use ndarray;
use numpy::{IntoPyArray, ToPyArray, PyArray, PyArray1, PyArray2, PyArrayDyn, PyReadonlyArrayDyn};
//https://docs.rs/numpy/latest/numpy/


fn fr(z: f64, k: f64) -> f64 { (k * k) / z }            // repulsive force
fn fa(z: f64, k: f64, x: f64) -> f64 { (x * x) / k }    // repulsive attraction 

fn update_positions(mut pos_x: Vec<f64>, mut pos_y: Vec<f64>, iterations: i32) -> PyArrayDyn< f64 > {
    // Implementing psuedo code from paper, image attached in repo
    // I should move these parameters somewhere else but placing for here now
    let W: f64 = 100.0;
    let L: f64 = 100.0;
    let area = L * W;
    let num_nodes: usize = pos_x.len();
    let mut k = area / num_nodes as f64;
    k = k.sqrt();

    let mut disp = [(0.0, 0.0); 5];

    for iter in 0..iterations {
        println!("{}", iter);
        for v in 0..num_nodes {
            for u in 0..num_nodes {
                println!("v, u = {}, {}", v, u);
                if v != u {
                    let mut delta = (
                        pos_x[v] - pos_x[u],
                        pos_y[v] - pos_y[u]
                    );
                    let delta_size = (delta.0 * delta.0 + delta.1 * delta.1).sqrt();
                    disp[v].0 = disp[v].0 + (delta.0 / delta_size) * fr(delta_size, k);
                    disp[v].1 = disp[v].1 + (delta.1 / delta_size) * fr(delta_size, k);
                    println!("disp = {}, {}", disp[v].0, disp[v].1)
                }
            }
        }
    }

    pos_x[0].0 = 5.0;

}

// https://itnext.io/how-to-bind-python-numpy-with-rust-ndarray-2efa5717ed21
// https://doc.rust-lang.org/std/vec/struct.Vec.html 
#[pyfunction]
fn create_force_graph(a: usize, b: usize) -> PyArrayDyn< f64 > { //Vec< f64 > {
    // create a force graph with default parameters
    let mut graph: ForceGraph = <ForceGraph>::new(Default::default());
    let xc: Vec<f64> = vec![250.0, 750.0, 250.0, 750.0, 500.0];
    let yc: Vec<f64>= vec![250.0, 250.0, 750.0, 750.0, 500.0];

    // https://docs.rs/force_graph/latest/force_graph/struct.NodeData.html
    let mut NI = Vec::new();
    for i: usize in 1..5 {
        let n_idx: NodeIndex = graph.add_node(NodeData { x: xc[i] as f32, y: yc[i] as f32, ..Default::default() });
        NI.push(n_idx);
    }
    graph.add_edge(NI[0], NI[2], Default::default());
    // XC
    let array = ndarray::Array::eye(a);
    // array.into_pyarray()
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rg(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(create_force_graph, m)?)?;
    Ok(())
}