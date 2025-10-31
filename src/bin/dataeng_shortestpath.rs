// Shortest Path algo with Dijkstra
// [X] Allow usser to specify start and end nodes
// [X] Include more landmarks and connections

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

    // New locations DLC
    let ccb = graph.add_node("Belém Cultural Center (CCB)");
    let maat = graph.add_node("MAAT Museum");
    let time_out_market = graph.add_node("Time Out Market");
    let chiado = graph.add_node("Chiado");
    let bairro_alto = graph.add_node("Bairro Alto");
    let santa_justa = graph.add_node("Santa Justa Lift");
    let rossio_square = graph.add_node("Rossio Square");
    let sao_jorge_castle = graph.add_node("São Jorge Castle");
    let alfama = graph.add_node("Alfama");
    let eduardo_vii_park = graph.add_node("Eduardo VII Park");
    let gulbenkian = graph.add_node("Gulbenkian Museum");
    let parque_nacoes = graph.add_node("Parque das Nações");
    let oceanario = graph.add_node("Oceanário de Lisboa");
    let oriente_station = graph.add_node("Oriente Station");

    graph.extend_with_edges([
        (belem_tower, monastery, 1), // The distance from Belem Tower to Jerónimos Monastery is 1 km
        (belem_tower, lx_factory, 3),
        (belem_tower, commerce_square, 7),
        (monastery, lx_factory, 3),
        (monastery, commerce_square, 6),
        (lx_factory, commerce_square, 5),
        (commerce_square, lisbon_cathedral, 1),
        // New connections
        (belem_tower, monastery, 1),
        (belem_tower, lx_factory, 3),
        (belem_tower, commerce_square, 7),
        (monastery, lx_factory, 3),
        (monastery, commerce_square, 6),
        (lx_factory, commerce_square, 5),
        (commerce_square, lisbon_cathedral, 1),
        (belem_tower, ccb, 1),
        (ccb, monastery, 1),
        (ccb, maat, 1),
        (maat, lx_factory, 3),
        (lx_factory, time_out_market, 3),
        (time_out_market, commerce_square, 1),
        (commerce_square, chiado, 1),
        (chiado, bairro_alto, 1),
        (chiado, santa_justa, 1),
        (santa_justa, rossio_square, 1),
        (lisbon_cathedral, sao_jorge_castle, 1),
        (sao_jorge_castle, alfama, 1),
        (alfama, commerce_square, 2),
        (rossio_square, eduardo_vii_park, 2),
        (eduardo_vii_park, gulbenkian, 1),
        (gulbenkian, parque_nacoes, 7),
        (parque_nacoes, oceanario, 1),
        (oceanario, oriente_station, 1),
        (time_out_market, chiado, 1),
        (bairro_alto, rossio_square, 1),
        (alfama, lisbon_cathedral, 1),
        (chiado, rossio_square, 1),
    ]);

    let name_index = build_name_index(&graph);
    calculate_route(&graph, &name_index, &args)
}
