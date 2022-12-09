use std::fmt::Display;

use iter_tools::Itertools;
use nom::{
    character::complete::{anychar, char, u32},
    multi::many1,
    sequence::separated_pair,
    IResult,
};

fn parse_move_line(line: String) -> (u32, (i32, i32)) {
    let result: IResult<&str, (char, u32)> =
        separated_pair(anychar, many1(char(' ')), u32)(&line);

    match result.expect("Valid input brah").1 {
        ('U', count) => (count, (0, 1)),
        ('D', count) => (count, (0, -1)),
        ('L', count) => (count, (-1, 0)),
        ('R', count) => (count, (1, 0)),
        _ => unreachable!("Brah, do you even validate?"),
    }
}

fn evaluate_moves(
    id: char,
) -> impl FnMut((i32, i32)) -> ((i32, i32), (i32, i32)) {
    let mut curr = (0, 0);
    move |delta| {
        curr.0 += delta.0;
        curr.1 += delta.1;
        (delta, curr)
    }
}

fn follow_head(
    id: char,
) -> impl FnMut(((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)) {
    let mut offset_to_head = (0, 0);
    let mut curr = (0, 0);

    move |(delta, head)| {
        offset_to_head.0 = offset_to_head.0 + delta.0;
        offset_to_head.1 = offset_to_head.1 + delta.1;

        // If the new offset has a _single_ component diff with abs > 2, we need
        // to snap to the direction dominated by the 2.
        offset_to_head = match offset_to_head {
            (-1..=1, 2) => (0, 1),
            (-1..=1, -2) => (0, -1),
            (2, -1..=1) => (1, 0),
            (-2, -1..=1) => (-1, 0),
            (x, y) => (x.signum(), y.signum()),
        };

        let new_loc = (head.0 - offset_to_head.0, head.1 - offset_to_head.1);
        let tail_delta = (new_loc.0 - curr.0, new_loc.1 - curr.1);
        curr = new_loc;
        (tail_delta, new_loc)
    }
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(parse_move_line)
        .flat_map(|(count, dir)| (0..count).map(move |_| dir))
        .map(evaluate_moves('H'))
        .map(follow_head('T'))
        .map(|(_, p)| p)
        .unique()
        .count()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(parse_move_line)
        .flat_map(|(count, dir)| (0..count).map(move |_| dir))
        .map(evaluate_moves('H'))
        .map(follow_head('1')) // Knot 1
        .map(follow_head('2')) // Knot 2
        .map(follow_head('3')) // Knot 3
        .map(follow_head('4')) // Knot 4
        .map(follow_head('5')) // Knot 5
        .map(follow_head('6')) // Knot 6
        .map(follow_head('7')) // Knot 7
        .map(follow_head('8')) // Knot 8
        .map(follow_head('9')) // Knot 9
        .map(|(_, p)| p)
        .unique()
        .count()
}
