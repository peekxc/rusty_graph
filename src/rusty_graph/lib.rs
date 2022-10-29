use pyo3::prelude::*;
use force_graph::{ForceGraph, Node, NodeData};


#[pyfunction]
fn create_force_graph(a: usize, b: usize) -> PyResult<String> {
    // create a force graph with default parameters
    let mut graph = <ForceGraph>::new(Default::default());
    let XC: [f32; 5] = [250.0, 750.0, 250.0, 750.0, 500.0];
    let YC: [f32; 5] = [250.0, 250.0, 750.0, 750.0, 500.0];

    // let NI: [NodeIndex; 5];
    for i in 1..5 {
        let n_idx = graph.add_node(NodeData { x: XC[i], y: YC[i], ..Default::default() });
        // NI[i] = n_idx
    }
    // set up links between nodes
    // graph.add_edge(n1_idx, n5_idx, Default::default());
    // graph.add_edge(n2_idx, n5_idx, Default::default());
    // graph.add_edge(n3_idx, n5_idx, Default::default());
    // graph.add_edge(n4_idx, n5_idx, Default::default());
    Ok((3).to_string())
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