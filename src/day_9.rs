use std::{cell::RefCell, fmt::Display, rc::Rc};

use iter_tools::Itertools;
use nom::{
    character::complete::{anychar, char, u32},
    multi::many1,
    sequence::separated_pair,
    IResult,
};

const RENDERING_ENABLED: bool = false;

/// Frivolous structure for drawing the state of the world for debugging
struct Canvas {
    contents: Vec<char>,
    width: i32,
    height: i32,
    top_left: (i32, i32),
}

impl Canvas {
    fn new() -> Self {
        Self {
            contents: vec!['s'],
            width: 1,
            height: 1,
            top_left: (0, 0),
        }
    }

    /// Calculate the index in the image buffer that corresponds to the given
    /// point
    fn index_for_coordinate(&self, coord: (i32, i32)) -> usize {
        let row_start = ((coord.1 - self.top_left.1) * self.width) as usize;
        let distance_into_row = (coord.0 - self.top_left.0) as usize;

        row_start + distance_into_row
    }

    /// Get the coordinates of the point at the given index in the image buffer
    fn coordinate_from_index(&self, i: usize) -> (i32, i32) {
        (
            i as i32 % self.width + self.top_left.0,
            i as i32 / self.width + self.top_left.1,
        )
    }

    /// Resize the canvas if needed to allow the new point to be displayed
    fn resize_if_needed(&mut self, new_point: (i32, i32)) {
        let x_bound = (self.top_left.0)..(self.top_left.0 + self.width);
        let y_bound = (self.top_left.1)..(self.top_left.1 + self.height);

        // No resize needed
        if x_bound.contains(&new_point.0) && y_bound.contains(&new_point.1) {
            return;
        }

        // Determine the new viewport
        let new_top_left = (
            self.top_left.0.min(new_point.0),
            self.top_left.1.min(new_point.1),
        );

        let top_left_shift = (
            new_top_left.0 - self.top_left.0,
            new_top_left.1 - self.top_left.1,
        );

        let new_width = (self.width - top_left_shift.0)
            .max(new_point.0 - self.top_left.0 + 1);
        let new_height = (self.height - top_left_shift.1)
            .max(new_point.1 - self.top_left.1 + 1);

        let mut new_canvas = Canvas {
            top_left: new_top_left,
            width: new_width,
            height: new_height,
            contents: vec!['.'; (new_width * new_height) as usize],
        };

        // Add the old contents to the new contents
        self.contents.iter().enumerate().for_each(|(i, c)| {
            let coord = self.coordinate_from_index(i);
            let new_i = new_canvas.index_for_coordinate(coord);
            new_canvas.contents[new_i] = *c;
        });

        *self = new_canvas;
    }

    fn move_entry(&mut self, id: char, from: (i32, i32), to: (i32, i32)) {
        if !RENDERING_ENABLED {
            return;
        }

        self.resize_if_needed(to);

        let from_i = self.index_for_coordinate(from);
        let to_i = self.index_for_coordinate(to);

        if self.contents[from_i] == id {
            self.contents[from_i] = '.';
        }

        self.contents[to_i] = id;

        let origin = self.index_for_coordinate((0, 0));
        if self.contents[origin] == '.' {
            self.contents[origin] = 's';
        }
    }

    fn render(&self) {
        if !RENDERING_ENABLED {
            return;
        }
        println!();
        self.contents
            .chunks(self.width as usize)
            .map(|chunk| chunk.iter().join(""))
            .for_each(|line| println!("{line}"));
    }
}

/// Parse a line of instructions into a direction tuple and a number of repeats
fn parse_move_line(line: String) -> (u32, (i32, i32)) {
    let result: IResult<&str, (char, u32)> =
        separated_pair(anychar, many1(char(' ')), u32)(&line);

    match result.expect("Valid input brah").1 {
        ('U', count) => (count, (0, -1)),
        ('D', count) => (count, (0, 1)),
        ('L', count) => (count, (-1, 0)),
        ('R', count) => (count, (1, 0)),
        _ => unreachable!("Brah, do you even validate?"),
    }
}

/// Create a closure that can be used in a `map` operation simulate the direct
/// movement of a knot starting from 0,0. It will apply the delta given and
/// return both the delta and the new position of the knot
fn evaluate_moves(
    id: char,
    canvas: Rc<RefCell<Canvas>>,
) -> impl FnMut((i32, i32)) -> ((i32, i32), (i32, i32)) {
    let mut curr = (0, 0);
    move |delta| {
        let prev = curr;

        curr.0 += delta.0;
        curr.1 += delta.1;

        canvas.borrow_mut().move_entry(id, prev, curr);

        (delta, curr)
    }
}

/// Create a closure that can be used in a `map` operation simulate the follow
/// behavior of a knot starting from 0,0. It will apply the rope rules between
/// the parent knot's position and movement to get the current knot's position
/// and change in position. The input and output are the same so this function
/// can be chained to simulate multiple knots in a rope
fn follow_head(
    id: char,
    canvas: Rc<RefCell<Canvas>>,
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

        canvas.borrow_mut().move_entry(id, curr, new_loc);

        curr = new_loc;
        (tail_delta, new_loc)
    }
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let canvas = Rc::new(RefCell::new(Canvas::new()));
    let mut visited_canvas = Canvas::new();

    let result = input_lines
        .map(parse_move_line)
        .flat_map(|(count, dir)| (0..count).map(move |_| dir))
        .map(evaluate_moves('H', canvas.clone()))
        .map(follow_head('T', canvas.clone()))
        .map(|(_, p)| p)
        .map(|v| {
            canvas.borrow().render();
            v
        })
        .unique()
        .map(|coord| visited_canvas.move_entry('#', (0, 0), coord))
        .count();

    visited_canvas.render();

    result
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let canvas = Rc::new(RefCell::new(Canvas::new()));
    let mut visited_canvas = Canvas::new();

    let result = input_lines
        .map(parse_move_line)
        .flat_map(|(count, dir)| (0..count).map(move |_| dir))
        .map(evaluate_moves('H', canvas.clone()))
        .map(follow_head('1', canvas.clone())) // Knot 1
        .map(follow_head('2', canvas.clone())) // Knot 2
        .map(follow_head('3', canvas.clone())) // Knot 3
        .map(follow_head('4', canvas.clone())) // Knot 4
        .map(follow_head('5', canvas.clone())) // Knot 5
        .map(follow_head('6', canvas.clone())) // Knot 6
        .map(follow_head('7', canvas.clone())) // Knot 7
        .map(follow_head('8', canvas.clone())) // Knot 8
        .map(follow_head('9', canvas.clone())) // Knot 9
        .map(|v| {
            canvas.borrow().render();
            v
        })
        .map(|(_, p)| p)
        .unique()
        .map(|coord| visited_canvas.move_entry('#', (0, 0), coord))
        .map(|coord| coord)
        .count();

    visited_canvas.render();

    result
}
