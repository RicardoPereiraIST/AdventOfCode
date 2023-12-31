// Idea from https://github.com/dommyrock/aoc/blob/main/aoc_2023/day-25/src/bin/part1.rs

use std::{fs, collections::HashMap};
use rustworkx_core::{petgraph::graph::{NodeIndex, UnGraph}, connectivity::stoer_wagner_min_cut};

const FILE_PATH : &str = "input.txt";
const CAP: i64 = 1;

type Nodes = HashMap<String, NodeIndex>;
type Graph = UnGraph<String, i64>;

fn create_graph(contents: &String) -> (Graph, Nodes)
{
    let mut graph: Graph = Graph::new_undirected();
    let mut nodes: Nodes = Nodes::new();

    for line in contents.lines()
    {
        let data_str: Vec<&str> = line.split(": ").collect();
        let nodes_str: Vec<&str> = data_str[1].split(" ").collect();
        let origin_name = data_str[0].to_string();

        for node_name in nodes_str
        {
            let origin = *nodes.entry(origin_name.clone()).or_insert_with(|| graph.add_node(origin_name.clone()));
            let destination = *nodes.entry(node_name.to_string()).or_insert_with(|| graph.add_node(node_name.to_string()));
            graph.add_edge(origin, destination, CAP);
            graph.add_edge(destination, origin, CAP);
        }
    }

    return (graph, nodes);
}

fn part1(graph: &Graph, nodes: &Nodes) -> u64
{
    let min_cut: rustworkx_core::Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_, group_1) = min_cut.unwrap().unwrap();
    let group_1_len = group_1.len() as u64;
    let group_2_len = nodes.len() as u64 - group_1_len;

    return group_1_len * group_2_len;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
                .expect("Should have been able to read the file");

    let (graph, nodes) = create_graph(&contents);

    let part1_result: u64 = part1(&graph, &nodes);
    
    println!("{}", part1_result);
}