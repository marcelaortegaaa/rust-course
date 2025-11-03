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
    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.neighbors(node).count() as f32;
        let degree_centrality = degree / ((graph.node_count() - 1) as f32);
        println!(
            "The degree centrality of {} is {:.3}",
            name, degree_centrality
        );

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
}

fn betweenness<N: fmt::Display, E>(g: &UnGraph<N, E>) {
    let n = g.node_count();
    let mut cb = vec![0.0; n];

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

    // Normalise
    for c in &mut cb {
        *c *= 0.5;
    }

    if n > 2 {
        let factor = 2.0 / ((n as f64 - 1.0) * (n as f64 - 2.0));
        for c in &mut cb {
            *c *= factor;
        }
    }

    // Print results
    println!("Betweenness centrality scores:");
    for node in g.node_indices() {
        println!("{}'s betweenness: {:.3}", g[node], cb[node.index()]);
    }
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    degree_centrality(&mut graph, &fighter_nodes, &fighters);
    println!("----------");
    betweenness(&graph);
}
