struct Node {
    id: u32,
    x: f64,
    y: f64,
}

struct Edge {
    u: Node,
    v: Node,
}

struct Graph<'a> {
    nodes: &'a [Node],
    edges: &'a [Edge],
}

fn main() {
    let g = build_graph();
    let mut positions = [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];
    let iterations = 100;

    // Print nodes
    for x in g.0 {
        println!("node = {}", x)
    }

    // Print positions
    println!("Before");
    for pos in positions {
        println!("(x,y) position = ({},{})", pos.0, pos.1)
    }

    positions = update_positions(g, positions, iterations);

    // Print positions
    println!("After");
    for pos in positions {
        println!("(x,y) position = ({},{})", pos.0, pos.1)
    }

    // Print edges
    for x in g.1 {
        println!("from = {}, to = {}", x.0, x.1)
    }
}

fn build_graph() -> ([i32; 5], [(i32, i32); 5]) {
    let nodes = [0, 1, 2, 3, 4];
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
    (nodes, edges)
}

fn update_positions(
    mut graph: ([i32; 5], [(i32, i32); 5]),
    mut positions: [(f64, f64); 5],
    iterations: i32,
) -> [(f64, f64); 5] {
    // Implementing psuedo code from paper, image attached in repo
    // I should move these parameters somewhere else but placing for here now
    let W = 100.0;
    let L = 100.0;
    let area = L * W;
    let num_nodes = graph.0.len();
    let num_edges = graph.1.len();
    let mut k = area / num_nodes as f64;
    k = k.sqrt();

    let mut disp = [(0.0, 0.0); 5];

    for iter in 0..iterations {
        // Calculate repulsive forces
        println!("{}", iter);
        for v in 0..num_nodes {
            for u in 0..num_nodes {
                println!("v, u = {}, {}", v, u);
                if v != u {
                    let mut delta = (
                        positions[v].0 - positions[u].0,
                        positions[v].1 - positions[u].1,
                    );
                    let delta_size = (delta.0 * delta.0 + delta.1 * delta.1).sqrt();
                    disp[v].0 = disp[v].0 + (delta.0 / delta_size) * fr(delta_size, k);
                    disp[v].1 = disp[v].1 + (delta.1 / delta_size) * fr(delta_size, k);
                    println!("disp = {}, {}", disp[v].0, disp[v].1)
                }
            }
        }

        // Calculate attractive forces
        for e in 0..num_edges {
            let v = graph.1[e].0 as usize;
            let u = graph.1[e].1 as usize;
            let mut delta = (
                positions[v].0 - positions[u].0,
                positions[v].1 - positions[u].1,
            );
            let delta_size = (delta.0 * delta.0 + delta.1 * delta.1).sqrt();
            disp[v].0 = disp[v].0 + (delta.0 / delta_size) * fa(delta_size, k);
            disp[v].1 = disp[v].1 + (delta.1 / delta_size) * fa(delta_size, k);
            disp[u].0 = disp[u].0 + (delta.0 / delta_size) * fa(delta_size, k);
            disp[u].1 = disp[u].1 + (delta.1 / delta_size) * fa(delta_size, k);
            println!("disp e.u = {}, {}", disp[u].0, disp[u].1);
            println!("disp e.v = {}, {}", disp[v].0, disp[v].1);
        }

        // use displacement to update positions
        let mut t = 9999999.0; // to do
        for v in 0..num_nodes {
            let disp_size = (disp[v].0 * disp[v].0 + disp[v].1 * disp[v].1).sqrt();

            positions[v].0 = positions[v].0 + (disp[v].0 / disp_size) * f64::min(disp[v].0, t);
            positions[v].1 = positions[v].1 + (disp[v].1 / disp_size) * f64::min(disp[v].1, t);
            // Limit positions to viewing area
            positions[v].0 = f64::min(W / 2.0, f64::max(-W / 2.0, positions[v].0));
            positions[v].1 = f64::min(L / 2.0, f64::max(-L / 2.0, positions[v].1));
        }
    }

    positions[0].0 = 5.0;
    positions
}

fn fr(z: f64, k: f64) -> f64 {
    (k * k) / z
}

fn fa(z: f64, k: f64) -> f64 {
    (z * z) / k
}
