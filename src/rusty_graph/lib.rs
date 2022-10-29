use std::vec::Vec;
use pyo3::prelude::*;
use force_graph::{ForceGraph, Node, NodeData};


// https://doc.rust-lang.org/std/vec/struct.Vec.html 
#[pyfunction]
fn create_force_graph(a: usize, b: usize) -> Vec< f32 > {
    // create a force graph with default parameters
    let mut graph = <ForceGraph>::new(Default::default());
    let XC = vec![250.0, 750.0, 250.0, 750.0, 500.0];
    let YC = vec![250.0, 250.0, 750.0, 750.0, 500.0];

    // https://docs.rs/force_graph/latest/force_graph/struct.NodeData.html
    let mut NI = Vec::new();
    for i in 1..5 {
        let n_idx = graph.add_node(NodeData { x: XC[i], y: YC[i], ..Default::default() });
        NI.push(n_idx);
    }
    graph.add_edge(NI[0], NI[2], Default::default());
    XC
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