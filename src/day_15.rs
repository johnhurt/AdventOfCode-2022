use std::{borrow::Borrow, collections::HashSet, fmt::Display};

use iter_tools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::i32,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::helpers::is_example;

#[derive(Debug)]
struct SensorBeaconPair {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

impl From<((i32, i32), (i32, i32))> for SensorBeaconPair {
    fn from((sensor, beacon): ((i32, i32), (i32, i32))) -> Self {
        Self { sensor, beacon }
    }
}

impl SensorBeaconPair {
    fn dist(&self) -> i32 {
        (self.sensor.0 - self.beacon.0).abs()
            + (self.sensor.1 - self.beacon.1).abs()
    }

    fn dist_to_h_line(&self, y: i32) -> i32 {
        (y - self.sensor.1).abs()
    }
}

fn parse_coordinates(input: &str) -> IResult<&'_ str, (i32, i32)> {
    preceded(tag("x="), separated_pair(i32, tag(", y="), i32))(input)
}

fn parse_pair(line: String) -> SensorBeaconPair {
    let result: IResult<&str, SensorBeaconPair> = map(
        preceded(
            tag("Sensor at "),
            separated_pair(
                parse_coordinates,
                tag(": closest beacon is at "),
                parse_coordinates,
            ),
        ),
        SensorBeaconPair::from,
    )(&line);

    result.expect("😭").1
}

/// Get a closure that can calculate the intersection between the exclusion
/// zone (created by the information about a given, closest sensor-beacon pair)
/// and the horizontal line at y = the given value
fn get_intersections_with_h_line<P>(
    y: i32,
) -> impl FnMut(P) -> Option<(i32, i32)>
where
    P: Borrow<SensorBeaconPair>,
{
    move |pair_ref| {
        let pair = pair_ref.borrow();
        let dist = pair.dist();
        let dist_to_line = pair.dist_to_h_line(y);

        let intersection_depth = dist - dist_to_line;
        let x = pair.sensor.0;

        (intersection_depth >= 0)
            .then_some((x - intersection_depth, x + intersection_depth))
    }
}

fn get_coverings_for_line<I, P>(pairs: I, test_line: i32) -> Vec<(i32, i32)>
where
    I: Iterator<Item = P>,
    P: Borrow<SensorBeaconPair>,
{
    let mut glob_count = 0;
    let mut curr_glob: Option<(i32, i32)> = None;

    pairs
        .filter_map(get_intersections_with_h_line(test_line))
        .sorted_by(|(left_1, _), (left_2, _)| left_1.cmp(left_2))
        .group_by(|(left, right)| {
            match curr_glob {
                Some((g_l, g_r)) if *left <= g_r => {
                    curr_glob = Some((g_l, (*right).max(g_r)));
                }
                _ => {
                    curr_glob = Some((*left, *right));
                    glob_count += 1;
                }
            };
            glob_count
        })
        .into_iter()
        .map(|(_, group)| {
            group.fold((i32::MAX, i32::MIN), |(min_l, max_r), (left, right)| {
                (min_l.min(left), max_r.max(right))
            })
        })
        .collect_vec()
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let test_line = if is_example() { 10 } else { 2_000_000 };
    let mut beacon_on_line_positions: HashSet<i32> = HashSet::new();

    let pairs = input_lines.map(parse_pair).map(|pair| {
        if pair.beacon.1 == test_line {
            beacon_on_line_positions.insert(pair.beacon.0);
        }
        pair
    });

    get_coverings_for_line(pairs, test_line)
        .into_iter()
        .map(|(left, right)| {
            let cover_count = right - left + 1;

            let beacon_count = beacon_on_line_positions
                .iter()
                .copied()
                .filter(|b_x| *b_x >= left && *b_x <= right)
                .count();

            cover_count - beacon_count as i32
        })
        .sum::<i32>()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let pairs = input_lines.map(parse_pair).collect_vec();

    let coord_max = if is_example() { 20 } else { 4_000_000 };

    (0..=coord_max)
        .map(|y| (y, get_coverings_for_line(pairs.iter(), y)))
        .map(|(y, covers)| {
            (
                y,
                covers
                    .into_iter()
                    .filter_map(|(left, right)| {
                        (right >= 0 || left <= coord_max)
                            .then_some((left.max(0), right.min(coord_max)))
                    })
                    .collect_vec(),
            )
        })
        .filter_map(|(y, covers)| {
            let mut last = -1;
            for (left, right) in covers {
                if left - last > 1 {
                    return Some((left - 1, y));
                }
                last = right
            }

            (last < coord_max).then_some((coord_max, y))
        })
        .next()
        .map(|(x, y)| x as i64 * 4_000_000 + y as i64)
        .expect("😰")
}
