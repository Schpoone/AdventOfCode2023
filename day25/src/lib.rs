use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    IResult,
};
use petgraph::{
    algo::{connected_components, tarjan_scc},
    graph::{DefaultIx, Graph, NodeIndex, UnGraph},
    visit::{EdgeRef, IntoNodeReferences},
    Undirected,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::{hash_map::RandomState, HashMap, HashSet};

fn parse_line(text: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (text, component) = alpha1(text)?;
    let (text, _) = tag(": ")(text)?;
    let (text, connections) = separated_list1(space1, alpha1)(text)?;
    Ok((text, (component, connections)))
}

fn parse(text: &str) -> IResult<&str, UnGraph<&str, usize>> {
    let (text, lines) = separated_list1(line_ending, parse_line)(text)?;
    let mut graph = Graph::new_undirected();
    let mut node_map = HashMap::new();
    for (component, connections) in lines {
        let node_id = *node_map
            .entry(component)
            .or_insert_with(|| graph.add_node(component));
        for connection in connections {
            let other_node_id = *node_map
                .entry(connection)
                .or_insert_with(|| graph.add_node(connection));
            graph.add_edge(node_id, other_node_id, 1);
        }
    }
    Ok((text, graph))
}

// Pretty much brute force
pub fn part1(text: String) -> usize {
    let (_, graph) = parse(text.as_str()).unwrap();
    graph
        .edge_indices()
        .combinations(3)
        .par_bridge()
        .progress_count(5600742476)
        .find_map_any(|cut_wires| {
            let mut graph = graph.clone();
            graph.retain_edges(|_, edge| !cut_wires.contains(&edge));
            if connected_components(&graph) == 2 {
                Some(
                    tarjan_scc(&graph)
                        .into_iter()
                        .map(|comp| comp.len())
                        .product(),
                )
            } else {
                None
            }
        })
        .unwrap()
}

fn connectivity<N>(
    graph: &UnGraph<N, usize>,
    target_set: &HashSet<NodeIndex<DefaultIx>>,
    node: NodeIndex<DefaultIx>,
) -> usize {
    target_set
        .iter()
        .map(|&other| {
            let edge = graph.find_edge(node, other);
            if let Some(edge) = edge {
                graph.edge_weight(edge).unwrap_or(&0)
            } else {
                &0
            }
        })
        .sum()
}

struct CutOfThePhase<'a> {
    partition_product: usize,
    weight: usize,
    graph: UnGraph<Vec<&'a str>, usize>,
}

fn min_cut_phase(graph: UnGraph<Vec<&str>, usize>) -> CutOfThePhase {
    let mut most_connected_group = HashSet::new();
    let mut least_connected_group = HashSet::<_, RandomState>::from_iter(graph.node_indices());
    let first = *least_connected_group.iter().next().unwrap();
    most_connected_group.insert(first);
    least_connected_group.remove(&first);
    while most_connected_group.len() < graph.node_count() - 2 {
        let next_most_connected = *least_connected_group
            .iter()
            .max_by_key(|&&node| connectivity(&graph, &most_connected_group, node))
            .unwrap();
        most_connected_group.insert(next_most_connected);
        least_connected_group.remove(&next_most_connected);
    }
    let mut penultimate = *least_connected_group
        .iter()
        .max_by_key(|&&node| connectivity(&graph, &most_connected_group, node))
        .unwrap();
    most_connected_group.insert(penultimate);
    least_connected_group.remove(&penultimate);

    let maybe_last = least_connected_group.into_iter().next();
    let last = match maybe_last {
        Some(node) => node,
        None => {
            let node = penultimate;
            penultimate = first;
            node
        }
    };
    let mut new_graph = Graph::new_undirected();
    let mut node_map = HashMap::new();
    for node in graph.node_references() {
        if node.0 != last && node.0 != penultimate {
            node_map.insert(node.1.to_vec(), new_graph.add_node(node.1.to_vec()));
        }
    }
    let mut merged_node = graph.node_weight(penultimate).unwrap().to_vec();
    merged_node.extend(graph.node_weight(last).unwrap().to_vec());
    let merged_node_idx = new_graph.add_node(merged_node);
    for edge in graph.edge_references() {
        let source = graph.node_weight(edge.source()).unwrap();
        let target = graph.node_weight(edge.target()).unwrap();
        match (
            edge.source() == penultimate || edge.source() == last,
            edge.target() == penultimate || edge.target() == last,
        ) {
            (true, false) => {
                let target_id = *node_map.get(target).unwrap();
                let current_edge = new_graph.find_edge(target_id, merged_node_idx);
                if let Some(current_edge) = current_edge {
                    let current_weight = new_graph.edge_weight(current_edge).unwrap();
                    new_graph.update_edge(
                        target_id,
                        merged_node_idx,
                        current_weight + edge.weight(),
                    );
                } else {
                    new_graph.add_edge(target_id, merged_node_idx, *edge.weight());
                }
            }
            (false, true) => {
                let source_id = *node_map.get(source).unwrap();
                let current_edge = new_graph.find_edge(source_id, merged_node_idx);
                if let Some(current_edge) = current_edge {
                    let current_weight = new_graph.edge_weight(current_edge).unwrap();
                    new_graph.update_edge(
                        source_id,
                        merged_node_idx,
                        current_weight + edge.weight(),
                    );
                } else {
                    new_graph.add_edge(source_id, merged_node_idx, *edge.weight());
                }
            }
            (true, true) => {}
            (false, false) => {
                let source_id = *node_map.get(source).unwrap();
                let target_id = *node_map.get(target).unwrap();
                new_graph.add_edge(source_id, target_id, *edge.weight());
            }
        }
    }
    let partition_product = graph
        .node_references()
        .filter_map(|(node_idx, weight)| {
            if node_idx == last {
                None
            } else {
                Some(weight.len())
            }
        })
        .sum::<usize>()
        * graph.node_weight(last).unwrap().len();
    CutOfThePhase {
        partition_product,
        weight: connectivity(&graph, &most_connected_group, last),
        graph: new_graph,
    }
}

// Do part 1 again but implement the Stoer-Wagner algorithm
pub fn part2(text: String) -> usize {
    let (_, original_graph) = parse(text.as_str()).unwrap();
    let mut new_graph = Graph::<Vec<&str>, usize, Undirected>::new_undirected();
    let mut node_map = HashMap::new();
    for edge in original_graph.edge_references() {
        let source = *original_graph.node_weight(edge.source()).unwrap();
        let target = *original_graph.node_weight(edge.target()).unwrap();
        let source_id = *node_map
            .entry(vec![source])
            .or_insert_with(|| new_graph.add_node(vec![source]));
        let target_id = *node_map
            .entry(vec![target])
            .or_insert_with(|| new_graph.add_node(vec![target]));
        new_graph.add_edge(source_id, target_id, *edge.weight());
    }
    let mut graph = new_graph;
    let mut min_cut = (0, usize::MAX);
    while graph.node_count() > 1 {
        let cotp = min_cut_phase(graph);
        if cotp.weight < min_cut.1 {
            min_cut = (cotp.partition_product, cotp.weight);
        }
        graph = cotp.graph;
    }
    min_cut.0
}
