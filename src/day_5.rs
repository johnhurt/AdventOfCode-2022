use iter_tools::Itertools;
use std::{fmt::Display, iter::once};

type Ship = Vec<Vec<u8>>;

// Parse a single line of the cargo-stack declaration section
// [A]    [B]  => [(1, A), (3, B)]
fn parse_cargo_line(line: String) -> impl Iterator<Item = (usize, u8)> {
    line.into_bytes()
        .into_iter()
        .chain(once(b' ')) // IDE stripped trailing spaces off input XD
        .tuples::<(u8, u8, u8, u8)>()
        .enumerate()
        .filter_map(move |(column, (c1, c2, _, _))| match (c1, c2) {
            (b'[', cargo) => Some((column + 1, cargo)),
            _ => None,
        })
}

/// Generate the contents of the ship and consume from the iterator only the
/// lines describing the ship (not the moves)
fn parse_ship<I>(lines: &mut I) -> Ship
where
    I: Iterator<Item = String>,
{
    // Take all the lines up until the empty line that separates setup from
    // instructions, and reverse them because the way it is defined in the file
    // works great visually, it's backward for what we need to allocating and
    // populating stacks
    let mut ship_lines = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev();

    // Find the number of stacks of crates by looking at the line labeling the
    // stacks.
    let label_line = ship_lines.next().expect("Label line guaranteed");
    let max_stack_index = label_line
        .into_bytes()
        .into_iter()
        .filter(|c| *c != b' ')
        .last()
        .map(|c| (c - b'0') as usize)
        .expect("At least one stack guaranteed");

    // Initialize the ship as a vec of empty vecs to accomodate all stack
    // _indices_ (Ignore the zero column, so we can use the indexes specified
    // in the move instructions)
    let mut ship: Ship = vec![vec![]; max_stack_index + 1];

    ship_lines
        .into_iter()
        .flat_map(parse_cargo_line)
        .for_each(|(column, cargo)| ship[column].push(cargo));

    ship
}

/// A struct to contain all the info we need from a single move instruction
#[derive(Debug)]
struct Move {
    repeats: i32,
    from: usize,
    to: usize,
}

/// Parse a single move instruction line
/// "move 23 from 6 to 4" => Move { repeat: 23, from: 6, to: 4 }
fn parse_move(move_line: String) -> Move {
    let mut cursor = move_line[5..].splitn(2, ' ');
    let repeats = cursor
        .next()
        .map(|repeats_str| repeats_str.parse::<i32>().ok())
        .flatten()
        .expect("Valid input guaranteed");
    let remainder = cursor.next().expect("Valid input guaranteed").as_bytes();

    let from = (remainder[5] - b'0') as usize;
    let to = (remainder[10] - b'0') as usize;

    Move { repeats, from, to }
}

/// Create a string with the labels of the crates on top of each stack in order
fn get_top_crates(ship: Ship) -> String {
    ship.into_iter()
        .filter_map(|mut stack| stack.pop())
        .map(|c| c as char)
        .collect::<String>()
}

pub fn problem_1<I>(mut input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let ship = parse_ship(&mut input_lines);

    let final_ship = input_lines.map(parse_move).fold(
        ship,
        |mut ship, Move { repeats, from, to }| {
            for _ in 0..repeats {
                let cargo = ship[from].pop().expect("Unexpected Noop move");
                ship[to].push(cargo);
            }

            ship
        },
    );

    get_top_crates(final_ship)
}

/**** Problem 2 ******/

pub fn problem_2<I>(mut input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let ship = parse_ship(&mut input_lines);

    let final_ship = input_lines.map(parse_move).fold(
        ship,
        |mut ship, Move { repeats, from, to }| {
            let remaining = ship[from].len() - repeats as usize;
            let mut to_move = ship[from].drain(remaining..).collect::<Vec<_>>();
            ship[to].append(&mut to_move);
            ship
        },
    );

    get_top_crates(final_ship)
}
