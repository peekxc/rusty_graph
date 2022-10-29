use std::vec::Vec;
// use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use force_graph::{ForceGraph, Node, NodeData};
use ndarray;
use numpy::{IntoPyArray, ToPyArray, PyArray, PyArray1, PyArray2, PyArrayDyn, PyReadonlyArrayDyn};
//https://docs.rs/numpy/latest/numpy/

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
    for i in 1..5 {
        let n_idx = graph.add_node(NodeData { x: xc[i] as f32, y: yc[i] as f32, ..Default::default() });
        NI.push(n_idx);
    }
    graph.add_edge(NI[0], NI[2], Default::default());
    // XC
    let array = ndarray::Array::eye(a);
    array.into_pyarray()
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