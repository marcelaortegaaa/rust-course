use petgraph::algo::astar;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn degree_centrality(
    graph: &mut UnGraph<&Fighter, f32>,
    fighter_nodes: &Vec<NodeIndex>,
    fighters: &[Fighter],
) {
    let mut scores: Vec<(String, f32)> = fighter_nodes
        .iter()
        .enumerate()
        .map(|(i, &node)| {
            let name = fighters[i].name.clone();
            let degree = graph.neighbors(node).count() as f32;
            let degree_centrality = degree / ((graph.node_count() - 1) as f32);
            (name, degree_centrality)
        })
        .collect();

    // Sort from highest to lowest
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("Degree Centrality (highest → lowest):");
    for (name, score) in scores {
        println!("{:<25} {:.3}", name, score);
    }

    // for (i, &node) in fighter_nodes.iter().enumerate() {
    //     let name = &fighters[i].name;
    //     let degree = graph.neighbors(node).count() as f32;
    //     let degree_centrality = degree / ((graph.node_count() - 1) as f32);
    //     println!(
    //         "The degree centrality of {} is {:.3}",
    //         name, degree_centrality
    //     );

    // // Explanation
    // match name.as_str() {
    //     "Conor McGregor" => println!(
    //         "{} has the highest degree centrality because he has fought with all other fighters in the network.",
    //         name
    //     ),
    //     "Dustin Poirier" | "Nate Diaz" => println!(
    //         "{} has a degree centrality of {:.3}, implying he had less fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
    //         name, degree_centrality
    //     ),
    //     "Khabib Nurmagomedov" | "Jose Aldo" => println!(
    //         "{} has the lowest centrality of {:.3} as he has fought with the least number of fighters.",
    //         name, degree_centrality
    //     ),
    //     _ => {}
    // }
    // println!("-----------------");
}

fn betweenness(g: &UnGraph<&Fighter, f32>) {
    use std::cmp::Ordering;

    let n = g.node_count();
    let mut cb = vec![0.0f64; n];

    // Brandes betweenness (unweighted, undirected)
    for s in g.node_indices() {
        let mut stack = Vec::new();
        let mut pred = vec![Vec::new(); n];
        let mut sigma = vec![0.0; n];
        let mut dist = vec![-1isize; n];

        sigma[s.index()] = 1.0;
        dist[s.index()] = 0;

        let mut q = VecDeque::new();
        q.push_back(s);

        while let Some(v) = q.pop_front() {
            stack.push(v);
            let dv = dist[v.index()];
            for w in g.neighbors(v) {
                if dist[w.index()] < 0 {
                    dist[w.index()] = dv + 1;
                    q.push_back(w);
                }
                if dist[w.index()] == dv + 1 {
                    sigma[w.index()] += sigma[v.index()];
                    pred[w.index()].push(v);
                }
            }
        }

        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &pred[w.index()] {
                if sigma[w.index()] > 0.0 {
                    delta[v.index()] +=
                        (sigma[v.index()] / sigma[w.index()]) * (1.0 + delta[w.index()]);
                }
            }
            if w != s {
                cb[w.index()] += delta[w.index()];
            }
        }
    }

    // Undirected graphs count each pair twice
    for c in &mut cb {
        *c *= 0.5;
    }

    // Normalise
    if n > 2 {
        let factor = 2.0 / ((n as f64 - 1.0) * (n as f64 - 2.0));
        for c in &mut cb {
            *c *= factor;
        }
    }

    let mut ranking: Vec<(String, f64)> = g
        .node_indices()
        .map(|node| (g[node].to_string(), cb[node.index()]))
        .collect();

    ranking.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

    println!("Betweenness Centrality (highest → lowest):");
    for (name, score) in ranking {
        println!("{:<25} {:.3}", name, score);
    }
}

fn eigenvector_centrality(g: &UnGraph<&Fighter, f32>) {
    let tolerance: f64 = 1e-9;
    let max_iters: usize = 100;

    let mut x = vec![1.0f64; g.node_count()]; // initial vector
    let mut y = vec![0.0f64; g.node_count()];

    for _ in 0..max_iters {
        // y = A * x
        for i in 0..g.node_count() {
            y[i] = 0.0;
        }
        for v in g.node_indices() {
            let vi = v.index();
            for w in g.neighbors(v) {
                y[vi] += x[w.index()];
            }
        }
        // normalize y (L2)
        let norm = y.iter().map(|t| t * t).sum::<f64>().sqrt();
        if norm == 0.0 {
            break;
        }
        for i in 0..g.node_count() {
            y[i] /= norm;
        }

        // check convergence
        let diff = y
            .iter()
            .zip(&x)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);
        x.clone_from_slice(&y);
        if diff < tolerance {
            break;
        }
    }

    // Normalise
    let s: f64 = x.iter().sum();
    if s > 0.0 {
        for xi in &mut x {
            *xi /= s;
        }
    }

    // Print
    let mut ranking: Vec<(String, f64)> = g
        .node_indices()
        .map(|v| (g[v].to_string(), x[v.index()]))
        .collect();

    ranking.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("Eigenvector Centrality (highest → lowest):");
    for (name, score) in ranking {
        println!("{:<25} {:.3}", name, score);
    }
}

fn shortest_path(
    g: &UnGraph<&Fighter, f32>,
    origin_name: &str,
    dest_name: &str,
) -> Result<(), String> {
    let find = |name: &str| -> Option<NodeIndex> { g.node_indices().find(|&v| g[v].name == name) };
    let start = find(origin_name).ok_or_else(|| format!("Unknown fighter: {}", origin_name))?;
    let goal = find(dest_name).ok_or_else(|| format!("Unknown fighter: {}", dest_name))?;

    if start == goal {
        println!("Origin and destination fighter is the same: {}", g[start]);
        return Ok(());
    }

    // A* with zero heuristic == Dijkstra, but returns the path
    // Prints "A -> B -> C" and the number of hops.
    if let Some((cost, path)) = astar(
        g,
        start,
        |n| n == goal,
        |_| 1.0f32, // each fight costs 1 hop
        |_| 0.0f32, // no heuristic available → zero
    ) {
        let names: Vec<String> = path.iter().map(|&n| g[n].to_string()).collect();
        println!(
            "It takes {} fights to get from {origin_name} to {dest_name}:\n{}",
            cost,
            names.join(" -> ")
        );
        Ok(())
    } else {
        Err(format!("No route found from {} to {}", g[start], g[goal]))
    }
}

fn main() -> Result<(), String> {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier"),        // 0
        Fighter::new("Khabib Nurmagomedov"),   // 1
        Fighter::new("Jose Aldo"),             // 2
        Fighter::new("Conor McGregor"),        // 3
        Fighter::new("Nate Diaz"),             // 4
        Fighter::new("Max Holloway"),          // 5
        Fighter::new("Tony Ferguson"),         // 6
        Fighter::new("Justin Gaethje"),        // 7
        Fighter::new("Michael Chandler"),      // 8
        Fighter::new("Charles Oliveira"),      // 9
        Fighter::new("Islam Makhachev"),       // 10
        Fighter::new("Alexander Volkanovski"), // 11
        Fighter::new("Rafael dos Anjos"),      // 12
        Fighter::new("Eddie Alvarez"),         // 13
        Fighter::new("Donald Cerrone"),        // 14
        Fighter::new("Michael Johnson"),       // 15
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    let fights: &[(usize, usize)] = &[
        // Original seven fights
        (0, 1), // Poirier vs Khabib
        (1, 3), // Khabib vs McGregor
        (3, 0), // McGregor vs Poirier
        (3, 2), // McGregor vs Aldo
        (3, 4), // McGregor vs Diaz
        (0, 4), // Poirier vs Diaz
        (2, 4), // Aldo vs Diaz
        // DLC
        (5, 3),   // Holloway vs McGregor
        (5, 11),  // Holloway vs Volkanovski
        (5, 2),   // Holloway vs Aldo
        (5, 0),   // Holloway vs Poirier
        (6, 7),   // Ferguson vs Gaethje
        (7, 8),   // Gaethje vs Chandler
        (7, 9),   // Gaethje vs Oliveira
        (7, 15),  // Gaethje vs Johnson
        (9, 8),   // Oliveira vs Chandler
        (10, 9),  // Makhachev vs Oliveira
        (12, 13), // RDA vs Alvarez
        (12, 14), // RDA vs Cerrone
        (12, 4),  // RDA vs Diaz
        (13, 3),  // Alvarez vs McGregor
        (13, 0),  // Alvarez vs Poirier
        (14, 4),  // Cerrone vs Diaz
        (14, 7),  // Cerrone vs Gaethje
        (0, 15),  // Poirier vs Johnson
        (2, 11),  // Aldo vs Volkanovski
                  // (10, 3), Makhachev vs McGregor fight to see how values change
    ];

    // Bulk-add edges
    for &(a, b) in fights {
        add_edge(&mut graph, &fighter_nodes, a, b);
    }

    degree_centrality(&mut graph, &fighter_nodes, &fighters);
    println!("----------");
    betweenness(&graph);
    println!("----------");
    eigenvector_centrality(&graph);
    println!("----------");
    shortest_path(&graph, "Alexander Volkanovski", "Khabib Nurmagomedov")?;

    Ok(())
}
