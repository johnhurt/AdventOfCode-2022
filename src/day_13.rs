use iter_tools::{EitherOrBoth, Itertools};
use nom::{
    branch::alt,
    character::complete::{char, i32},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};
use std::{cmp::Ordering, fmt::Display};

/// Representation of a packet
#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Num(i32),
    Nest(Vec<Packet>),
}

/************* Nom nom nom nom! ***********/

macro_rules! between {
    ($before:expr, $pat:expr, $after:expr) => {
        preceded($before, terminated($pat, $after))
    };
}

fn parse_number_packet(input: &str) -> IResult<&'_ str, Packet> {
    map(i32, Packet::Num)(input)
}
fn parse_nested_packet(input: &str) -> IResult<&'_ str, Packet> {
    between!(
        char('['),
        map(separated_list0(char(','), parse_packet), Packet::Nest),
        char(']')
    )(input)
}

fn parse_packet(input: &str) -> IResult<&'_ str, Packet> {
    alt((parse_nested_packet, parse_number_packet))(input)
}

fn parse_packet_line(line: String) -> Packet {
    parse_packet(&line).expect("😇").1
}

/*************** Actual logic ***********/

fn compare_vec(left: &[Packet], right: &[Packet]) -> Ordering {
    use EitherOrBoth::*;
    use Ordering::*;
    let mut result = Equal;

    left.iter()
        .zip_longest(right.iter())
        .map(|pair| match pair {
            Both(l_p, r_p) => compare(l_p, r_p),
            Left(_) => Greater,
            Right(_) => Less,
        })
        .take_while(|r| {
            result = *r;
            matches!(r, Equal)
        })
        .for_each(|_| ());

    result
}

/// determine if the first/left packet is less than the second/right
fn compare(left: &Packet, right: &Packet) -> Ordering {
    use Packet::*;

    match (left, right) {
        (Num(i), Num(j)) => i.cmp(j),
        (Num(i), Nest(v)) => compare_vec(&[Num(*i)], v),
        (Nest(v), Num(i)) => compare_vec(v, &[Num(*i)]),
        (Nest(v_l), Nest(v_r)) => compare_vec(v_l, v_r),
    }
}

/// Add some rustisms to make sorting in iterators work
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("🥲")
    }
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .filter(|line| !line.is_empty())
        .map(parse_packet_line)
        .tuples::<(Packet, Packet)>()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            (compare(&left, &right) != Ordering::Greater).then_some(i + 1)
        })
        .sum::<usize>()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let markers = [
        parse_packet_line("[[2]]".to_owned()),
        parse_packet_line("[[6]]".to_owned()),
    ];

    markers
        .clone()
        .into_iter()
        .chain(
            input_lines
                .filter(|line| !line.is_empty())
                .map(parse_packet_line),
        )
        .sorted()
        .enumerate()
        .filter_map(|(i, p)| markers.contains(&p).then_some(i + 1))
        .product::<usize>()
}
