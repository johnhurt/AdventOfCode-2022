use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use iter_tools::Itertools;
use nom::{
    branch::alt, character::complete::char, combinator::map, multi::many1,
    sequence::tuple, IResult,
};
use priority_queue::PriorityQueue;

enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn from_c(c: char) -> Option<Self> {
        match c {
            '^' => Some(Dir::N),
            'v' => Some(Dir::S),
            '<' => Some(Dir::W),
            '>' => Some(Dir::E),
            _ => None,
        }
    }
}

enum Line {
    Start(i32),
    End(i32),
    Blizzards(Vec<(i32, Dir)>),
}

fn parse_line(line: String) -> Line {
    let result: IResult<&str, Line> = map(
        tuple((
            many1(char('#')),
            many1(alt((char('.'), char('^'), char('>'), char('v'), char('<')))),
            many1(char('#')),
        )),
        |(pre, content, post)| {
            if post.len() > 1 {
                Line::Start(pre.len() as i32)
            } else if pre.len() > 1 {
                Line::End(pre.len() as i32)
            } else {
                Line::Blizzards(
                    content
                        .into_iter()
                        .enumerate()
                        .filter_map(|(i, c)| {
                            Dir::from_c(c).map(|d| (i as i32, d))
                        })
                        .collect_vec(),
                )
            }
        },
    )(&line);

    result.expect("🦖").1
}

struct Map {
    width: i32,
    height: i32,
    start: (i32, i32),
    finish: (i32, i32),
    h_storms: HashMap<i32, Vec<(i32, i32)>>,
    v_storms: HashMap<i32, Vec<(i32, i32)>>,
}

fn read_map<I>(input_lines: I) -> Map
where
    I: Iterator<Item = String>,
{
    use Dir::*;

    let mut start: (i32, i32) = (0, -1);
    let mut finish: (i32, i32) = (0, 0);
    let mut width = 0;
    let mut height = 0;

    let mut b_by_x: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut b_by_y: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    input_lines
        .map(parse_line)
        .enumerate()
        .for_each(|(y, line)| match line {
            Line::Start(s) => {
                start.0 = s - 1;
            }
            Line::Blizzards(bs) => bs.into_iter().for_each(|(x, d)| {
                let x = x as i32;
                let y = y as i32 - 1;
                match d {
                    N => b_by_x.entry(x).or_default().push((y, -1)),
                    S => b_by_x.entry(x).or_default().push((y, 1)),
                    E => b_by_y.entry(y).or_default().push((x, 1)),
                    W => b_by_y.entry(y).or_default().push((x, -1)),
                }
            }),
            Line::End(e) => {
                finish = (e - 1, y as i32 - 1);
                width = e;
                height = y as i32 - 1;
            }
        });

    Map {
        start,
        finish,
        height,
        width,
        h_storms: b_by_y,
        v_storms: b_by_x,
    }
}

fn get_possible_moves(
    (x, y): (i32, i32),
    t: i32,
    map: &Map,
) -> Vec<(i32, i32)> {
    let mut blocked = HashSet::new();

    [x - 1, x, x + 1]
        .into_iter()
        .map(|c| c.rem_euclid(map.width))
        .filter_map(|c| map.v_storms.get(&c).map(|v| (c, v)))
        .for_each(|(c, v)| {
            v.iter().for_each(|(start, delta)| {
                blocked.insert((c, (start + t * delta).rem_euclid(map.height)));
            })
        });

    [y - 1, y, y + 1]
        .into_iter()
        .map(|c| c.rem_euclid(map.height))
        .filter_map(|c| map.h_storms.get(&c).map(|v| (c, v)))
        .for_each(|(c, v)| {
            v.iter().for_each(|(start, delta)| {
                blocked.insert(((start + t * delta).rem_euclid(map.width), c));
            })
        });

    [(x, y), (x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y)]
        .into_iter()
        .filter(|(x, y)| {
            ((*x, *y) == map.start)
                || ((*x, *y) == map.finish)
                || (*x >= 0 && *y >= 0 && *x < map.width && *y < map.height)
        })
        .filter(|d| !blocked.contains(d))
        .collect_vec()
}

/// Run an A* search on the map from the start to the finish using the storm
/// rules on each step to determine possible moves
fn run_search(map: &Map, start_t: i32) -> i32 {
    let mut search = PriorityQueue::new();

    let priority = |p: (i32, i32), d: i32| {
        -((map.finish.0 - p.0).abs() + (map.finish.1 - p.1).abs() + d)
    };

    search.push((map.start, start_t), priority(map.start, start_t));

    loop {
        let ((p, mut t), _) = search.pop().expect("🐝");

        if p == map.finish {
            return t;
        }

        t += 1;

        get_possible_moves(p, t, map).into_iter().for_each(|n| {
            search.push((n, t), priority(n, t));
        });
    }
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let map = read_map(input_lines);
    run_search(&map, 0)
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut map = read_map(input_lines);
    let dt = run_search(&map, 0);

    std::mem::swap(&mut map.start, &mut map.finish);
    let dt = run_search(&map, dt);

    std::mem::swap(&mut map.start, &mut map.finish);
    run_search(&map, dt)
}
