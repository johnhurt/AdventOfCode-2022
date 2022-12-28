use std::{collections::HashSet, fmt::Display, hash::Hash};

use iter_tools::Itertools;
use nom::{
    branch::alt, character::complete::anychar, character::complete::char,
    character::complete::u32, combinator::map, multi::many1, IResult,
};

use crate::helpers::is_example;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Portal {
    start: usize,
    end: usize,
    blocked: bool,
    orientation: Orientation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Wall,
    Empty,
    Port(Portal),
    DoublePort(Portal, Portal),
    Never,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(usize)]
enum Orientation {
    #[default]
    E,
    S,
    W,
    N,
}

#[derive(Debug)]
enum Move {
    L,
    R,
    Step(usize),
}

#[derive(Debug)]
struct State {
    board: Vec<Space>,
    width: usize,
    height: usize,
    position: usize,
    orientation: Orientation,
    moves: Vec<Move>,
    move_cursor: usize,
}

fn parse_board_line(line: String) -> Vec<Space> {
    let result: IResult<&str, Vec<Space>> = many1(map(anychar, |c| match c {
        '.' => Space::Empty,
        '#' => Space::Wall,
        ' ' => Space::Never,
        _ => unreachable!("Duh, wha?"),
    }))(&line);

    result.expect("🙀").1
}

fn parse_inst_line(line: String) -> Vec<Move> {
    use Move::*;

    let result: IResult<&str, Vec<Move>> = many1(alt((
        map(u32, |v| Step(v as usize)),
        map(char('L'), |_| L),
        map(char('R'), |_| R),
    )))(&line);

    result.expect("🥸").1
}

/// Fill in the line with the given shape to the board at the given row. This
/// Will take care of portals east and west
fn fill_board_line(
    row: usize,
    line: &[Space],
    board: &mut [Space],
    width: usize,
) {
    use Orientation::*;
    use Space::*;

    let row_start = row * width + 1;

    let mut first_valid = (usize::MAX, Never);
    let mut last_valid = (0, Never);

    (row_start..(row_start + line.len()))
        .enumerate()
        .for_each(|(s, t)| {
            if line[s] != Never {
                if first_valid.1 == Never {
                    first_valid = (t, line[s]);
                }
                last_valid = (t, line[s]);
            }
            board[t] = line[s];
        });

    match (first_valid, last_valid) {
        ((first, Empty), (last, Empty)) => {
            board[first - 1].add_portal(Portal::new_unblocked(first, last, W));
            board[last + 1].add_portal(Portal::new_unblocked(last, first, E));
        }
        ((first, Wall), (last, Empty)) => {
            board[last + 1].add_portal(Portal::new_blocked(last, first, E));
        }
        ((first, Empty), (last, Wall)) => {
            board[first - 1].add_portal(Portal::new_blocked(first, last, W));
        }
        ((_, Wall), (_, Wall)) => {}
        _ => unreachable!("That should have covered everything"),
    }
}

fn fill_vertical_lines(
    board: &mut [Space],
    width: usize,
    height: usize,
) -> usize {
    use Orientation::*;
    use Space::*;

    let mut start: usize = 0;

    for col in 1..(width - 1) {
        let mut first_valid = (0, Never);
        let mut last_valid = (0, Never);
        for row in 1..(height - 1) {
            let curr = row * width + col;

            if board[curr].in_bounds() {
                if start == 0 && row == 1 {
                    start = curr;
                }
                if first_valid.1 == Never {
                    first_valid = (curr, board[curr]);
                }
                last_valid = (curr, board[curr]);
            }
        }

        match (first_valid, last_valid) {
            ((first, Empty), (last, Empty)) => {
                board[first - width]
                    .add_portal(Portal::new_unblocked(first, last, N));
                board[last + width]
                    .add_portal(Portal::new_unblocked(last, first, S));
            }
            ((first, Wall), (last, Empty)) => {
                board[last + width]
                    .add_portal(Portal::new_blocked(last, first, S));
            }
            ((first, Empty), (last, Wall)) => {
                board[first - width]
                    .add_portal(Portal::new_blocked(first, last, N));
            }
            ((first, Wall), (last, Wall)) => {}
            _ => unreachable!("That should have covered everything"),
        }
    }

    start
}

impl State {
    fn new<I>(mut input_lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut max_width = 0;

        let lines = (&mut input_lines)
            .take_while(|line| {
                max_width = max_width.max(line.len());
                !line.is_empty()
            })
            .map(parse_board_line)
            .collect_vec();

        let instructions = input_lines.map(parse_inst_line).next().expect("🤢");

        let width = max_width + 2;
        let height = lines.len() + 2;

        let mut board = vec![Space::Never; width * height];

        lines.into_iter().enumerate().for_each(|(i, line)| {
            fill_board_line(i + 1, &line, &mut board, width);
        });

        let start = fill_vertical_lines(&mut board, width, height);

        State {
            board,
            height,
            width,
            position: start,
            move_cursor: 0,
            moves: instructions,
            orientation: Default::default(),
        }
    }

    fn run(&mut self) {
        use Move::*;

        for i in 0..self.moves.len() {
            match self.moves[i] {
                L => self.orientation.left(),
                R => self.orientation.right(),
                Step(n) => self.step(n),
            }
        }
    }

    fn step(&mut self, n: usize) {
        use Space::*;

        for _ in 0..n {
            let next_position =
                self.orientation.delta()(self.position, self.width);

            match &self.board[next_position] {
                Empty => self.position = next_position,
                Wall | Port(Portal { blocked: true, .. }) => break,
                Port(Portal {
                    end,
                    orientation,
                    blocked: false,
                    ..
                }) => {
                    self.position = *end;
                    self.orientation = *orientation;
                }
                DoublePort(port, other) | DoublePort(other, port)
                    if port.start == self.position =>
                {
                    if port.blocked {
                        break;
                    }
                    self.position = port.end;
                    self.orientation = port.orientation;
                }
                _ => unreachable!("Supposedly"),
            }
        }
    }

    fn row(&self) -> usize {
        self.position / self.width
    }

    fn col(&self) -> usize {
        self.position % self.width
    }

    fn portal_into(
        &self,
        target: usize,
        normal: Orientation,
    ) -> (usize, Portal) {
        let blocked = self.board[target] == Space::Wall;

        (
            normal.delta()(target, self.width),
            Portal {
                start: 0,
                end: target,
                blocked,
                orientation: normal.opposite(),
            },
        )
    }

    fn remap_to_cube(&mut self, side: usize) {
        use Face::*;
        use Orientation::*;

        let width = self.width;

        let mut edge_stack = vec![];

        let mut curr = self.position;

        // Start from the top left corner of the front face of the cube which
        // set as the starting point of the traversal, and walk the perimeter of
        // the board.
        let mut face = Front;
        let mut edge = Edge {
            from: (0, 0, 1),
            to: (1, 0, 1),
        };
        let mut orientation = E;
        let mut normal = N;

        let mut added_edges = HashSet::new();

        // Do a pass to remove existing portals
        self.board.iter_mut().for_each(|s| {
            if matches!(s, Space::Port(_) | Space::DoublePort(_, _)) {
                *s = Space::Never;
            }
        });

        loop {
            if !added_edges.contains(&edge.reverse()) {
                added_edges.insert(edge);
                for _ in 0..side {
                    edge_stack.push(self.portal_into(curr, normal));
                    curr = orientation.delta()(curr, width);
                }
            } else {
                for _ in 0..side {
                    let (p_1, mut p_1_portal) = edge_stack.pop().expect("😧");
                    let (p_2, mut p_2_portal) = self.portal_into(curr, normal);

                    p_1_portal.start = p_2_portal.end;
                    p_2_portal.start = p_1_portal.end;

                    self.board[p_2].add_portal(p_1_portal);
                    self.board[p_1].add_portal(p_2_portal);

                    curr = orientation.delta()(curr, width);
                }
            }

            if self.board[curr].in_bounds() {
                let out = normal.delta()(curr, width);

                if self.board[out].in_bounds() {
                    curr = out;
                    (edge, face) = edge.next_interior(face);
                    orientation.left();
                    normal.left();
                } else {
                    // Straight
                    (edge, face) = edge.next_straight(face);
                }
            } else {
                // Right Turn (exterior corner)
                curr = orientation.opposite().delta()(curr, width);
                orientation.right();
                normal.right();

                (edge, face) = edge.next_exterior(face);
            }

            if curr == self.position {
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

/// Simplistic and somehow over-generalized representation of an edge. It
/// tells where in 3d space the edge is going and from where
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    from: (u8, u8, u8),
    to: (u8, u8, u8),
}

impl Edge {
    /// Omg, there has to be a better way to do this
    fn next_exterior(&self, face: Face) -> (Edge, Face) {
        use Face::*;
        let to = match (face, self.from, self.to) {
            (Front | Back, (x1, y, z), (x2, _, _)) if x1 != x2 => {
                (x2, y, (z == 0) as u8)
            }
            (Front | Back, (x, y, z1), (_, _, z2)) if z1 != z2 => {
                ((x == 0) as u8, y, z2)
            }
            (Left | Right, (x, y, z1), (_, _, z2)) if z1 != z2 => {
                (x, (y == 0) as u8, z2)
            }
            (Left | Right, (x, y1, z), (_, y2, _)) if y1 != y2 => {
                (x, y2, (z == 0) as u8)
            }
            (Top | Bottom, (x, y1, z), (_, y2, _)) if y1 != y2 => {
                ((x == 0) as u8, y2, z)
            }
            (Top | Bottom, (x1, y, z), (x2, _, _)) if x1 != x2 => {
                (x2, (y == 0) as u8, z)
            }
            _ => unreachable!("I hope"),
        };

        (Edge { from: self.to, to }, face)
    }

    /// Seriously. This can't be the way
    fn next_straight(&self, face: Face) -> (Edge, Face) {
        use Face::*;
        let (to, new_face) = match (face, self.from, self.to) {
            (Front, (0, _, z), (1, _, _)) => ((1, 1, z), Right),
            (Front, (1, _, z), (0, _, _)) => ((0, 1, z), Left),
            (Front, (x, _, 0), (_, _, 1)) => ((x, 1, 1), Top),
            (Front, (x, _, 1), (_, _, 0)) => ((x, 1, 0), Bottom),

            (Back, (0, _, z), (1, _, _)) => ((1, 0, z), Right),
            (Back, (1, _, z), (0, _, _)) => ((0, 0, z), Left),
            (Back, (x, _, 0), (_, _, 1)) => ((x, 0, 1), Top),
            (Back, (x, _, 1), (_, _, 0)) => ((x, 0, 0), Bottom),

            (Left, (_, 0, z), (_, 1, _)) => ((1, 1, z), Back),
            (Left, (_, 1, z), (_, 0, _)) => ((1, 0, z), Front),
            (Left, (_, y, 0), (_, _, 1)) => ((1, y, 1), Top),
            (Left, (_, y, 1), (_, _, 0)) => ((1, y, 0), Bottom),

            (Right, (_, 0, z), (_, 1, _)) => ((0, 1, z), Back),
            (Right, (_, 1, z), (_, 0, _)) => ((0, 0, z), Front),
            (Right, (_, y, 0), (_, _, 1)) => ((0, y, 1), Top),
            (Right, (_, y, 1), (_, _, 0)) => ((0, y, 0), Bottom),

            (Bottom, (x, 0, _), (_, 1, _)) => ((x, 1, 1), Back),
            (Bottom, (x, 1, _), (_, 0, _)) => ((x, 0, 1), Front),
            (Bottom, (0, y, _), (1, _, _)) => ((1, y, 1), Right),
            (Bottom, (1, y, _), (0, _, _)) => ((1, y, 1), Left),

            (Top, (x, 0, _), (_, 1, _)) => ((x, 1, 0), Back),
            (Top, (x, 1, _), (_, 0, _)) => ((x, 0, 0), Front),
            (Top, (0, y, _), (1, _, _)) => ((1, y, 0), Right),
            (Top, (1, y, _), (0, _, _)) => ((1, y, 0), Left),
            _ => unreachable!("I hope"),
        };

        (Edge { from: self.to, to }, new_face)
    }

    /// I guess we're just going for it
    fn next_interior(&self, face: Face) -> (Edge, Face) {
        use Face::*;
        let new_face = match (face, self.from, self.to) {
            (Front | Right | Back | Left, (_, _, 1), (_, _, 1)) => Top,
            (Front | Right | Back | Left, (_, _, 0), (_, _, 0)) => Bottom,
            (Front | Top | Back | Bottom, (1, _, _), (1, _, _)) => Right,
            (Front | Top | Back | Bottom, (0, _, _), (0, _, _)) => Left,
            (Top | Right | Bottom | Left, (_, 1, _), (_, 1, _)) => Back,
            (Top | Right | Bottom | Left, (_, 0, _), (_, 0, _)) => Front,
            _ => unreachable!("I hope"),
        };

        (
            Edge {
                from: self.to,
                to: self.from,
            },
            new_face,
        )
    }

    fn reverse(&self) -> Self {
        Edge {
            from: self.to,
            to: self.from,
        }
    }
}

impl Orientation {
    fn left(&mut self) {
        use Orientation::*;
        match self {
            E => *self = N,
            S => *self = E,
            W => *self = S,
            N => *self = W,
        }
    }
    fn right(&mut self) {
        use Orientation::*;
        match self {
            E => *self = S,
            S => *self = W,
            W => *self = N,
            N => *self = E,
        }
    }
    fn delta(&self) -> impl Fn(usize, usize) -> usize {
        use Orientation::*;

        match self {
            E => |p, _| p + 1,
            S => |p, w| p + w,
            W => |p, _| p - 1,
            N => |p, w| p - w,
        }
    }
    fn opposite(&self) -> Self {
        use Orientation::*;

        match self {
            E => W,
            S => N,
            W => E,
            N => S,
        }
    }
}

impl Space {
    fn add_portal(&mut self, portal: Portal) {
        *self = match *self {
            Space::Port(portal_2) => Space::DoublePort(portal, portal_2),
            _ => Self::Port(portal),
        }
    }

    fn in_bounds(&self) -> bool {
        matches!(self, Space::Empty | Space::Wall)
    }
}

impl Portal {
    fn new_unblocked(start: usize, end: usize, dir: Orientation) -> Self {
        Self {
            start,
            end,
            orientation: dir,
            blocked: false,
        }
    }

    fn new_blocked(start: usize, end: usize, dir: Orientation) -> Self {
        Self {
            start,
            end,
            orientation: dir,
            blocked: true,
        }
    }
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut state = State::new(input_lines);
    state.run();

    println!("row: {}, col: {}", state.row(), state.col());
    state.row() * 1000 + state.col() * 4 + state.orientation as usize
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut state = State::new(input_lines);

    state.remap_to_cube(if is_example() { 4 } else { 50 });

    state.run();

    state.row() * 1000 + state.col() * 4 + state.orientation as usize
}
