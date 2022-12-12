use iter_tools::Itertools;
use std::{collections::LinkedList, fmt::Display, iter::once};

/// Basically an image
struct HeightMap {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

fn parse_map<I>(mut input_lines: I) -> HeightMap
where
    I: Iterator<Item = String>,
{
    let first_line = input_lines.next().expect("🧐");
    let width = first_line.len();

    let buffer = once(first_line)
        .chain(input_lines)
        .map(String::into_bytes)
        .flat_map(Vec::into_iter)
        .map(|c| match c {
            b'S' => b'a' - 1,
            b'E' => b'z' + 1,
            _ => c,
        })
        .collect::<Vec<_>>();

    let height = buffer.len() / width;

    HeightMap {
        buffer,
        width,
        height,
    }
}

struct MapNode {
    elev: u8,
    neighbors: Vec<usize>,
    visited_distance: Option<i32>,
}

impl HeightMap {
    /// Convert the map image into a graph and return the keys for the start and
    /// end nodes
    fn into_graph(self) -> (usize, usize, Vec<MapNode>) {
        let mut result = Vec::with_capacity(self.width * self.height);
        let mut start = 0_usize;
        let mut end = 0_usize;

        result.extend((0..result.capacity()).map(|i| {
            let node = self.to_node(i);
            if node.elev < b'a' {
                start = i;
            } else if node.elev > b'z' {
                end = i;
            }

            node
        }));

        (start, end, result)
    }

    /// Get the traversable neighbors of the point at the given index
    fn to_node(&self, index: usize) -> MapNode {
        let col = index % self.width;
        let row = index / self.width;
        let curr_elev = self.buffer[index];

        let neighbors = [
            (col > 0).then_some(index.saturating_sub(1)), // west
            (col < self.width - 1).then_some(index + 1),  // east
            (row < self.height - 1).then_some(index + self.width), // south,
            (row > 0).then_some(index.saturating_sub(self.width)), // north
        ]
        .into_iter()
        .flatten()
        .map(|i| (i, self.buffer[i]))
        .filter(|(_, elev)| *elev <= curr_elev + 1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

        MapNode {
            elev: curr_elev,
            neighbors,
            visited_distance: None,
        }
    }
}

/// Search the graph with the closest nodes first starting and ending where
/// specified and return the distance to the end node (if a route is possible)
fn bfs(start: usize, end: usize, graph: &mut [MapNode]) -> Option<i32> {
    // Clear the nodes before starting since we might be reusing this graph
    graph.iter_mut().for_each(|n| n.visited_distance = None);

    let mut queue = LinkedList::new();
    queue.push_back((start, 0));

    loop {
        if let Some((curr_idx, dist)) = queue.pop_front() {
            let curr = &mut graph[curr_idx];

            if curr.visited_distance.is_some() {
                // No need to repeat a visit
                continue;
            } else {
                curr.visited_distance = Some(dist)
            }

            if curr_idx == end {
                break Some(dist);
            }

            queue.extend(curr.neighbors.iter().map(|n| (*n, dist + 1)));
        } else {
            break None;
        }
    }
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let (start, end, mut graph) = parse_map(input_lines).into_graph();

    // bfs from the start to the end to get the final distance
    bfs(start, end, &mut graph).expect("🥸")
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let map = parse_map(input_lines);

    // This is kind of a brute force method. We look for all the `'a'`s in the
    // map and do a bfs for all of them and find the shortest path. A better way
    // would be to do a single bfs from the end and stop the search whenever an
    // `'a'` is hit
    let possible_starts = map
        .buffer
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (*c == b'a').then_some(i))
        .collect_vec();

    let (_, end, mut graph) = map.into_graph();

    possible_starts
        .into_iter()
        .filter_map(|start| bfs(start, end, &mut graph))
        .min()
        .expect("😡")
}
