use std::fmt::Display;

use iter_tools::Itertools;

#[derive(Debug, Clone, Copy)]
struct MixerNode {
    i: usize,
    prev: usize,
    next: usize,
    value: i64,
}

impl MixerNode {
    fn new(i: usize, v: i64, total: usize) -> Self {
        MixerNode {
            i,
            prev: (i + total - 1) % total,
            next: (i + 1) % total,
            value: v,
        }
    }
}

struct Mixer {
    nodes: Vec<MixerNode>,
    zero: usize,
}

impl Mixer {
    fn new<I>(values: I) -> Self
    where
        I: Iterator<Item = i64>,
    {
        let numbers = values.collect_vec();
        let len = numbers.len();

        let mut zero = 0_usize;

        Mixer {
            nodes: numbers
                .into_iter()
                .enumerate()
                .map(|(i, v)| {
                    if v == 0 {
                        assert!(zero == 0);
                        zero = i;
                    }
                    MixerNode::new(i, v, len)
                })
                .collect_vec(),
            zero,
        }
    }

    fn mix_step_left(&mut self, source: usize, delta: usize) {
        let source_node = self.nodes[source];
        self.nodes[source_node.prev].next = source_node.next;
        self.nodes[source_node.next].prev = source_node.prev;

        let mut target = source;

        // mod on `len - 1` here because we remove the source item while we
        // shift it
        for i in 0..(delta % (self.nodes.len() - 1)) {
            target = self.nodes[target].prev;
        }

        let target_node = self.nodes[target];
        self.nodes[source].prev = target_node.prev;
        self.nodes[source].next = target;
        self.nodes[target_node.prev].next = source;
        self.nodes[target].prev = source;
    }

    fn mix_step_right(&mut self, source: usize, delta: usize) {
        let source_node = self.nodes[source];
        self.nodes[source_node.prev].next = source_node.next;
        self.nodes[source_node.next].prev = source_node.prev;

        let mut target = source;

        // mod on `len - 1` here because we remove the source item while we
        // shift it
        for i in 0..(delta % (self.nodes.len() - 1)) {
            target = self.nodes[target].next;
        }

        let target_node = self.nodes[target];
        self.nodes[source].next = target_node.next;
        self.nodes[source].prev = target;
        self.nodes[target_node.next].prev = source;
        self.nodes[target].next = source;
    }

    fn mix(&mut self) {
        for mix_i in 0..self.nodes.len() {
            let delta: i64 = self.nodes[mix_i].value;
            match () {
                () if delta < 0 => self.mix_step_left(mix_i, (-delta) as usize),
                () if delta > 0 => self.mix_step_right(mix_i, delta as usize),
                _ => {}
            }
        }
    }

    fn get_at_index(&self, i: usize) -> i64 {
        let mut result = self.zero;
        for _ in 0..(i % self.nodes.len()) {
            result = self.nodes[result].next
        }

        self.nodes[result].value
    }

    fn print(&self) {
        let mut report = String::new();
        let mut curr = self.zero;
        for _ in 0..(self.nodes.len()) {
            report += &format!("{} ", self.nodes[curr].value);
            curr = self.nodes[curr].next;
        }

        println!("{}", report);
    }
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut mixer =
        Mixer::new(input_lines.map(|line| line.parse::<i64>().expect("🎃")));

    mixer.mix();

    [1000, 2000, 3000]
        .into_iter()
        .map(|i| mixer.get_at_index(i))
        .sum::<i64>()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut mixer = Mixer::new(
        input_lines
            .map(|line| line.parse::<i64>().expect("🎃"))
            .map(|v| v * 811589153),
    );

    for _ in 0..10 {
        mixer.mix();
    }

    [1000, 2000, 3000]
        .into_iter()
        .map(|i| mixer.get_at_index(i))
        .sum::<i64>()
}
