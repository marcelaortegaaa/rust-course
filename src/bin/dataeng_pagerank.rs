// PageRank algorithm for graphs
// [X] Allow rank to receive a user's graph (with nodes and connections)
// [X] Normalize values so they sum to 1

use clap::{ArgAction, Parser};

fn parse_usize_list(s: &str) -> Result<Vec<usize>, String> {
    s.split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<usize>().map_err(|e| e.to_string()))
        .collect()
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Graph node as comma-separated links; repeat for each node.
    #[arg(short,
        long,
        help = "Outgoing links for a node",
        value_parser = parse_usize_list,
        action = ArgAction::Append)]
    graph_node: Vec<Vec<usize>>,

    #[arg(short, long, help = "PageRank damping", default_value_t = 0.85)]
    damping: f64,

    #[arg(short, long, help = "PageRank iterations", default_value_t = 100)]
    iterations: usize,

    #[arg(short, long, help = "Names for nodes", value_delimiter = ',')]
    names: Option<Vec<String>>,
}

// The PageRank struct holds the damping factor and the number of iterations to run the algorithm.
struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    // The new function creates a new instance of the PageRank struct.
    fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    // The rank function calculates and returns the PageRank for each node in the graph.
    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        // The number of nodes in the graph.
        let n = graph.len();

        // The initial PageRank value for each node.
        let mut ranks = vec![1.0 / (n as f64); n];

        // Iterates the specified number of times.
        for _ in 0..self.iterations {
            // A new vector to hold the updated PageRank values.
            let mut new_ranks = vec![0.0; n];

            // Iterates over each node and its edges in the graph.
            for (node, edges) in graph.iter().enumerate() {
                // The amount of PageRank value this node contributes to its linked nodes.
                let contribution = ranks[node] / (edges.len() as f64);

                // Distributes the PageRank value to the linked nodes.
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            // Updates the PageRank values using the damping factor.
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            // Replaces the old PageRank values with the new ones.
            ranks = new_ranks;
        }

        // Normalise
        let sum: f64 = ranks.iter().sum();
        if sum > 0.0 {
            for r in &mut ranks {
                *r /= sum;
            }
        }

        // Returns the final PageRank values.
        ranks
    }
}

fn main() {
    // The graph represents links between sports websites. Each index represents a website,and the values in the vectors are the indexes of the websites they link to.
    let graph = vec![
        vec![1, 2], // ESPN links to NFL, NBA
        vec![0],    // NFL links to ESPN
        vec![0, 3], // NBA links to ESPN, UFC
        vec![0],    // UFC links to ESPN
        vec![0, 1], // MLB links to ESPN, NFL
    ];

    let args = Args::parse();
    let user_graph = args.graph_node;

    let _original_names = vec!["ESPN", "NFL", "NBA", "UFC", "MLB"];

    // The names corresponding to the indexes of the websites.
    let names: Vec<String> = match args.names {
        Some(n) => n,
        None => (1..=user_graph.len())
            .map(|i| format!("Node {}", i))
            .collect(),
    };

    // Initializes the PageRank struct.
    let pagerank = PageRank::new(args.damping, args.iterations);

    // Calculates the PageRank values.
    let _ranks = pagerank.rank(&graph);
    let user_ranks = pagerank.rank(&user_graph);

    // Prints the PageRank values.
    for (i, rank) in user_ranks.iter().enumerate() {
        println!("The PageRank of {} is {}", names[i], rank);
    }

    let sum: f64 = user_ranks.iter().sum();
    println!("Sum of PageRank values: {}", sum);
}
