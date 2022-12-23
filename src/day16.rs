use advent_of_code_2022::{print_2d_array, print_u8_2d_array_with_delim, read_file_to_string};
use array2d::Array2D;
use itertools::Itertools;
use sscanf::scanf;
use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

#[derive(Eq, PartialEq, Hash, Debug)]
struct Node {
    name: String,
    flow_rate: u32,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, flow_rate): (String, u32) =
            scanf!(s, "Valve {String} has flow rate={u32}").map_err(|_| "Invalid format")?;

        Ok(Self { name, flow_rate })
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Edge {
    u: String,
    v: String,
}

impl Edge {
    fn new(u: &str, v: &str) -> Self {
        if u < v {
            Self {
                u: u.to_string(),
                v: v.to_string(),
            }
        } else {
            Self {
                u: v.to_string(),
                v: u.to_string(),
            }
        }
    }

    fn get_other_side(&self, name: &str) -> Option<&str> {
        if name == self.u {
            Some(&self.v)
        } else if name == self.v {
            Some(&self.u)
        } else {
            None
        }
    }
}

fn calculate_shortest_paths(
    nodes_with_indices: &BTreeMap<String, usize>,
    edges: &HashSet<Edge>,
) -> Array2D<u32> {
    let n = nodes_with_indices.len();

    let mut shortest_paths = Array2D::filled_with(u32::MAX / 2, n, n);

    for i in 0..n {
        shortest_paths.set(i, i, 0).unwrap();
    }

    for Edge { u, v } in edges {
        let u_idx = *nodes_with_indices.get(u).unwrap();
        let v_idx = *nodes_with_indices.get(v).unwrap();

        shortest_paths.set(u_idx, v_idx, 1).unwrap();
        shortest_paths.set(v_idx, u_idx, 1).unwrap();
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let new_len = shortest_paths.get(i, k).unwrap() + shortest_paths.get(k, j).unwrap();
                if shortest_paths.get(i, j).unwrap() > &new_len {
                    shortest_paths.set(i, j, new_len).unwrap();
                }
            }
        }
    }

    shortest_paths
}

struct Volcano {
    nodes: BTreeMap<String, Node>,
    edges: HashSet<Edge>,
    nodes_with_indices: BTreeMap<String, usize>,
    shortest_paths: Array2D<u32>,
}

impl FromStr for Volcano {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = BTreeMap::new();
        let mut edges = HashSet::new();

        s.lines()
            .filter_map(|l| l.split_once("; "))
            .for_each(|(l, r)| {
                let node = Node::from_str(l).unwrap();

                let neighbour_nodes = r.splitn(5, ' ').last().unwrap();
                edges.extend(
                    neighbour_nodes
                        .split(", ")
                        .map(|v| Edge::new(&node.name, v)),
                );

                nodes.insert(node.name.clone(), node);
            });

        let nodes_with_indices = nodes
            .keys()
            .cloned()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<BTreeMap<_, _>>();

        let shortest_paths = calculate_shortest_paths(&nodes_with_indices, &edges);

        Ok(Self {
            nodes,
            edges,
            nodes_with_indices,
            shortest_paths,
        })
    }
}

impl Volcano {
    fn calculate(&self) -> u32 {
        let mut max_pressure = 0;
        let nodes_with_flow_rate = self
            .nodes
            .values()
            .filter(|n| n.flow_rate > 0)
            .collect::<Vec<_>>();

        for permutation in nodes_with_flow_rate
            .iter()
            .permutations(nodes_with_flow_rate.len())
        {
            let mut remaining_time = 30;
            let mut pressure = 0;
            let mut prev_node = self.nodes.get("AA").unwrap();
            for &current_node in permutation {
                let prev_idx = *self.nodes_with_indices.get(&prev_node.name).unwrap();
                let current_idx = *self.nodes_with_indices.get(&current_node.name).unwrap();

                let distance = *self.shortest_paths.get(prev_idx, current_idx).unwrap() + 1;
                if distance > remaining_time {
                    break;
                }

                remaining_time -= distance as u32;
                pressure += current_node.flow_rate * remaining_time;

                prev_node = current_node;
            }

            max_pressure = max_pressure.max(pressure);
        }
        max_pressure
    }
}

fn main() {
    let input = read_file_to_string("input/day16.txt");
    let volcano = Volcano::from_str(&input).unwrap();
    let result = volcano.calculate();
    dbg!(result);
}
