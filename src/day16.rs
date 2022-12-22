use sscanf::scanf;
use std::{collections::HashSet, str::FromStr};

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

#[derive(Eq, PartialEq, Hash)]
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

struct Volcano {
    nodes: HashSet<Node>,
    edges: HashSet<Edge>,
}

impl FromStr for Volcano {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();

        s.lines()
            .filter_map(|l| l.split_once("; "))
            .for_each(|(l, r)| {
                let neighbour_nodes = r.splitn(5, ' ').last().unwrap();
                edges.extend(neighbour_nodes.split(", ").map(|v| Edge::new(l, v)));

                let node = Node::from_str(l).unwrap();
                nodes.insert(node);
            });

        Ok(Self { nodes, edges })
    }
}

fn main() {}
