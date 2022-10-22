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

    positions = update_positions(positions, iterations);

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
    let nodes = [1, 2, 3, 4, 5];
    let edges = [(1, 2), (2, 3), (3, 4), (4, 5), (5, 1)];
    (nodes, edges)
}

fn update_positions(mut positions: [(f64, f64); 5], iterations: i32) -> [(f64, f64); 5] {
    positions[0].0 = 5.0;

    positions
}
