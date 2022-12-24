use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use iter_tools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn next(&self) -> Self {
        use Dir::*;
        match self {
            N => S,
            S => W,
            W => E,
            E => N,
        }
    }

    fn can_move(&self, directions: &[bool; 8]) -> bool {
        use Dir::*;

        match self {
            N => !(directions[7] || directions[0] || directions[1]),
            E => !(directions[1] || directions[2] || directions[3]),
            S => !(directions[3] || directions[4] || directions[5]),
            W => !(directions[5] || directions[6] || directions[7]),
        }
    }

    fn apply(&self, (x, y): (i32, i32)) -> (i32, i32) {
        use Dir::*;
        match self {
            N => (x, y - 1),
            S => (x, y + 1),
            W => (x - 1, y),
            E => (x + 1, y),
        }
    }

    fn find_next_move(
        &self,
        (x, y): (i32, i32),
        occupied: &HashSet<(i32, i32)>,
    ) -> Option<(i32, i32)> {
        // N, NE, E, SE, S, SW, W, NW
        let directions_blocked = [
            occupied.contains(&(x, y - 1)),
            occupied.contains(&(x + 1, y - 1)),
            occupied.contains(&(x + 1, y)),
            occupied.contains(&(x + 1, y + 1)),
            occupied.contains(&(x, y + 1)),
            occupied.contains(&(x - 1, y + 1)),
            occupied.contains(&(x - 1, y)),
            occupied.contains(&(x - 1, y - 1)),
        ];

        if directions_blocked.iter().all(|i| !*i) {
            return None;
        }

        let mut curr = *self;

        for _ in 0..4 {
            if curr.can_move(&directions_blocked) {
                return Some(curr.apply((x, y)));
            }
            curr = curr.next();
        }

        None
    }
}

fn parse_elf_locations(
    y: usize,
    line: String,
) -> impl Iterator<Item = (i32, i32)> {
    line.into_bytes()
        .into_iter()
        .enumerate()
        .filter_map(|(x, v)| (v == b'#').then_some(x))
        .map(move |x| (x as i32, y as i32))
}

fn run_simulation<I>(input_lines: I, max_rounds: Option<usize>) -> (i32, usize)
where
    I: Iterator<Item = String>,
{
    use Dir::*;

    let mut locations = HashSet::new();
    let mut elves = input_lines
        .enumerate()
        .flat_map(|(y, line)| parse_elf_locations(y, line))
        .map(|e| {
            locations.insert(e);
            (e, N, None)
        })
        .collect_vec();

    let mut moves = 1;
    let mut rounds = 0;

    let mut planned_moves = HashMap::new();

    while moves > 0 && max_rounds.map(|max| rounds < max).unwrap_or(true) {
        moves = 0;

        // Plan Moves
        planned_moves.clear();

        for i in 0..elves.len() {
            let (p, d, _) = elves[i];
            let plan_opt = d.find_next_move(p, &locations);

            if let Some(plan) = plan_opt {
                if let Some(prev_e) = planned_moves.insert(plan, i) {
                    // An elf already planned to move there, so no one moves
                    elves[prev_e].2 = None;
                } else {
                    elves[i].2 = Some(plan);
                }
            }

            elves[i].1 = d.next();
        }

        // Apply Moves
        elves.iter_mut().for_each(|(e, _, m_opt)| {
            if let Some(m) = m_opt {
                moves += 1;
                locations.remove(e);
                *e = *m;
                locations.insert(*e);
                *m_opt = None;
            }
        });

        rounds += 1;
    }

    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);

    elves.iter().map(|(p, _, _)| p).for_each(|(x, y)| {
        min.0 = min.0.min(*x);
        min.1 = min.1.min(*y);
        max.0 = max.0.max(*x);
        max.1 = max.1.max(*y);
    });

    let size = (max.0 - min.0 + 1) * (max.1 - min.1 + 1) - elves.len() as i32;

    (size, rounds)
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    run_simulation(input_lines, Some(10)).0
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    run_simulation(input_lines, None).1
}
