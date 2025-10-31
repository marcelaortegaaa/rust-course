// Shortest Path algo with Dijkstra
// [X] Allow usser to specify start and end nodes
// [ ] Include more landmarks and connections

use clap::Parser;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_name = "ORIGIN", required = true)]
    origin: String,

    #[arg(short, long, value_name = "DESTINATION", required = true)]
    destination: String,
}

fn build_name_index<'a>(graph: &'a Graph<&'a str, u32, Undirected>) -> HashMap<&'a str, NodeIndex> {
    let mut map = HashMap::new();
    for i in graph.node_indices() {
        map.insert(graph[i], i);
    }
    map
}

fn calculate_route(
    graph: &Graph<&str, u32, Undirected>,
    index: &HashMap<&str, NodeIndex>,
    args: &Args,
) -> Result<(), String> {
    let origin = *index
        .get(args.origin.as_str())
        .ok_or_else(|| format!("Unknown origin: {}", args.origin))?;
    let destination = *index
        .get(args.destination.as_str())
        .ok_or_else(|| format!("Unknown destination: {}", args.destination))?;

    let dist = dijkstra(graph, origin, Some(destination), |e| *e.weight());

    if let Some(d) = dist.get(&destination) {
        println!(
            "The shortest distance from {} to {} is {} km",
            graph[origin], graph[destination], d
        );
    } else {
        println!(
            "No route found from {} to {}",
            graph[origin], graph[destination]
        );
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    let belem_tower = graph.add_node("Belem Tower");
    let monastery = graph.add_node("Jerónimos Monastery");
    let lx_factory = graph.add_node("LX Factory");
    let commerce_square = graph.add_node("Commerce Square");
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    graph.extend_with_edges([
        (belem_tower, monastery, 1), // The distance from Belem Tower to Jerónimos Monastery is 1 km
        (belem_tower, lx_factory, 3), // The distance from Belem Tower to LX Factory is 3 km
        (belem_tower, commerce_square, 7), // The distance from Belem Tower to Commerce Square is 7 km
        (monastery, lx_factory, 3), // The distance from Jerónimos Monastery to LX Factory is 3 km
        (monastery, commerce_square, 6), // The distance from Jerónimos Monastery to Commerce Square is 6 km
        (lx_factory, commerce_square, 5), // The distance from LX Factory to Commerce Square is 5 km
        (commerce_square, lisbon_cathedral, 1), // The distance from Commerce Square to Lisbon Cathedral is 1 km
    ]);

    let name_index = build_name_index(&graph);
    calculate_route(&graph, &name_index, &args)
}
