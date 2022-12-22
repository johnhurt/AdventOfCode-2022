use std::{
    collections::{HashMap, HashSet, LinkedList},
    fmt::Display,
};

use iter_tools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::i32,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Default)]
struct DirectSpaceGraphNode {
    flow: i32,
    label: u16,
    neighbors: Vec<u16>,
}

#[derive(Debug, Clone)]
struct SpaceGraphNode {
    id: usize,
    flow: i32,
    label: u16,
    neighbors: Vec<usize>,
}

/// Turn the first 2 bytes in a slice to a u16
fn to_u16(v: &str) -> u16 {
    let b = v.as_bytes();
    ((b[0] as u16) << 8) + b[1] as u16
}

fn to_label(v: u16) -> String {
    format!(
        "{}{}",
        ((v >> 8) & 0xff) as u8 as char,
        (v & 0xff) as u8 as char
    )
}

impl DirectSpaceGraphNode {
    fn new(label: &str, flow: i32, neighbors: Vec<&'_ str>) -> Self {
        Self {
            flow,
            label: to_u16(label),
            neighbors: neighbors.into_iter().map(to_u16).collect_vec(),
        }
    }
}

fn parse_space_node(line: String) -> DirectSpaceGraphNode {
    let parsed: IResult<&str, DirectSpaceGraphNode> = map(
        preceded(
            tag("Valve "),
            tuple((
                alpha1,
                preceded(tag(" has flow rate="), i32),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), alpha1),
                ),
            )),
        ),
        |(label, flow, neighbors)| {
            DirectSpaceGraphNode::new(label, flow, neighbors)
        },
    )(&line);

    parsed.expect("🥶").1
}

fn build_space_graph<I>(input_lines: I) -> (usize, Vec<SpaceGraphNode>)
where
    I: Iterator<Item = String>,
{
    let temp_result = input_lines.map(parse_space_node).collect_vec();

    let label_to_id: HashMap<u16, usize> = temp_result
        .iter()
        .enumerate()
        .map(|(i, n)| (n.label, i))
        .collect();

    let mut result = Vec::with_capacity(temp_result.len());

    result.extend(temp_result.into_iter().enumerate().map(
        |(
            i,
            DirectSpaceGraphNode {
                label,
                flow,
                neighbors,
            },
        )| {
            SpaceGraphNode {
                id: i,
                label,
                flow,
                neighbors: neighbors
                    .into_iter()
                    .filter_map(|l| label_to_id.get(&l).cloned())
                    .collect_vec(),
            }
        },
    ));

    (label_to_id.get(&to_u16("AA")).cloned().expect("😭"), result)
}

fn all_pairs_shorted(graph: &[SpaceGraphNode]) -> Vec<i32> {
    let len = graph.len();
    let mut dist = vec![i32::MAX / 32; graph.len() * graph.len()];

    graph.iter().enumerate().for_each(|(i, node)| {
        dist[i * len + i] = 0;
        for j in &node.neighbors {
            dist[i * len + j] = 1;
            dist[j * len + i] = 1;
        }
    });

    for k in 0..len {
        for i in 0..len {
            for j in 0..len {
                if dist[i * len + j] > dist[i * len + k] + dist[k * len + j] {
                    dist[i * len + j] = dist[i * len + k] + dist[k * len + j]
                }
            }
        }
    }

    dist
}

struct SimplifiedNode {
    orig_id: usize,
    flow: i32,
    label: String,
    neighbors: Vec<(i32, usize)>,
}

fn simplify_graph(
    start: usize,
    graph: Vec<SpaceGraphNode>,
) -> (usize, Vec<SimplifiedNode>) {
    let len = graph.len();
    let all_pairs = all_pairs_shorted(&graph);

    let mut result = graph
        .iter()
        .enumerate()
        .filter(|(_, n)| n.flow > 0)
        .map(|(i, n)| SimplifiedNode {
            orig_id: i,
            flow: n.flow,
            label: to_label(n.label),
            neighbors: vec![],
        })
        .collect_vec();

    for i in 0..result.len() {
        let i_orig = result[i].orig_id;
        let neighbors = result
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(j, n)| {
                let j_orig = n.orig_id;
                (all_pairs[i_orig * len + j_orig] + 1, j)
            })
            .collect_vec();
        result[i].neighbors = neighbors;
    }

    let start_node = SimplifiedNode {
        orig_id: 10000,
        label: "Start".to_owned(),
        flow: 0,
        neighbors: result
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != start)
            .map(|(i, n)| (all_pairs[start * len + n.orig_id] + 1, i))
            .collect_vec(),
    };

    result.push(start_node);

    (result.len() - 1, result)
}

fn search(start: usize, mins: i32, graph: &[SimplifiedNode]) -> i32 {
    let mut queue = LinkedList::new();

    let mut max = (start, mins, 0, 0_u64, "Root".to_owned());

    queue.push_back(max.clone());

    let mut new_paths = vec![];

    while !queue.is_empty() {
        let (curr_idx, mins_left, total_flow, state, label) =
            queue.pop_front().expect("🥸");
        let curr = &graph[curr_idx];

        new_paths.extend(
            curr.neighbors
                .iter()
                .filter(|(d, n)| {
                    mins_left - d >= 0 && state & (1 << *n as u64) == 0
                })
                .map(|(d, n)| {
                    (
                        *n,
                        mins_left - d,
                        total_flow + graph[*n].flow * (mins_left - d),
                        state | (1 << *n as u64),
                        graph[*n].label.clone(),
                    )
                }),
        );

        if new_paths.is_empty() && max.2 < total_flow {
            max = (curr_idx, mins_left, total_flow, state, label);
        }

        new_paths.drain(..).for_each(|v| queue.push_back(v));
    }

    max.2
}

fn partition(
    start: usize,
    graph: &[SimplifiedNode],
    left_nodes: &HashSet<usize>,
) -> (Vec<SimplifiedNode>, Vec<SimplifiedNode>) {
    let left = graph
        .iter()
        .map(|n| {
            let neighbors = n
                .neighbors
                .iter()
                .copied()
                .filter(|(_, i)| *i == start || left_nodes.contains(i))
                .collect_vec();

            SimplifiedNode {
                orig_id: n.orig_id,
                flow: n.flow,
                label: n.label.clone(),
                neighbors,
            }
        })
        .collect_vec();
    let right = graph
        .iter()
        .map(|n| {
            let neighbors = n
                .neighbors
                .iter()
                .copied()
                .filter(|(_, i)| *i == start || !left_nodes.contains(i))
                .collect_vec();

            SimplifiedNode {
                orig_id: n.orig_id,
                flow: n.flow,
                label: n.label.clone(),
                neighbors,
            }
        })
        .collect_vec();

    (left, right)
}

fn subsets(set: HashSet<usize>) -> Vec<HashSet<usize>> {
    let len = set.len();
    let mut result = Vec::with_capacity(1 << len);

    result.push(HashSet::new());

    for v in set {
        for i in 0..result.len() {
            let mut curr = result[i].clone();
            curr.insert(v);
            result.push(curr);
        }
    }

    result
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let (start, space_graph) = build_space_graph(input_lines);

    let (start, graph) = simplify_graph(start, space_graph);

    search(start, 30, &graph)
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let (start, space_graph) = build_space_graph(input_lines);

    let (start, graph) = simplify_graph(start, space_graph);

    let subsets = subsets(
        (0..graph.len())
            .filter(|i| *i != start)
            .collect::<HashSet<_>>(),
    );

    let min_subset_size = (graph.len() - 1) * 40 / 100;

    subsets
        .into_iter()
        .filter(|p| {
            p.len() > min_subset_size
                && p.len() < (graph.len() - min_subset_size)
        })
        .map(|p| partition(start, &graph, &p))
        .map(|(left, right)| {
            search(start, 26, &left) + search(start, 26, &right)
        })
        .max()
        .expect("👎")
}
