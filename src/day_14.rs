use iter_tools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::char, character::complete::i32,
    multi::separated_list1, sequence::separated_pair, IResult,
};
use std::fmt::Display;

use crate::canvas::Canvas;

/// Yup
fn parse_line(line: String) -> Vec<(i32, i32)> {
    let result: IResult<&str, Vec<(i32, i32)>> = separated_list1(
        tag(" -> "),
        separated_pair(i32, char(','), i32),
    )(&line);

    result.expect("😫").1
}

/// Create a canvas to display the state of the sand
fn create_canvas<I>(lines: I, bottom_line: bool) -> Canvas
where
    I: Iterator<Item = Vec<(i32, i32)>>,
{
    let mut result = Canvas::new(' ', (500, 0));

    // Draw all the rocks defined in the problem input
    lines
        .flat_map(|vec| {
            vec.into_iter().tuple_windows::<((i32, i32), (i32, i32))>()
        })
        .for_each(|(start, end)| result.draw_line(start, end, '█'));

    if bottom_line {
        let floor_y = result.top_left.1 + result.height + 1;
        let floor_length = result.height + 2;

        result.draw_line(
            (500 - floor_length, floor_y),
            (500 + floor_length, floor_y),
            '█',
        );
    }

    // Draw a boundary on all sides, whenever a sand grain hits that boundary
    // our search is over
    let new_top_left = (result.top_left.0 - 1, result.top_left.1 - 1);
    let new_bottom_right = (
        result.top_left.0 + result.width,
        result.top_left.1 + result.height,
    );

    [
        (new_top_left, (new_top_left.0, new_bottom_right.1)),
        (new_top_left, (new_bottom_right.0, new_top_left.1)),
        (new_bottom_right, (new_top_left.0, new_bottom_right.1)),
        (new_bottom_right, (new_bottom_right.0, new_top_left.1)),
    ]
    .into_iter()
    .for_each(|(start, end)| {
        result.draw_line(start, end, '~');
    });

    result
}

#[derive(Debug)]
struct GraphNode {
    end: bool,
    children: Vec<usize>,
}

fn create_graph(canvas: &Canvas) -> (usize, Vec<GraphNode>) {
    let mut result = Vec::with_capacity(canvas.contents.len());
    let start = canvas.index_for_coordinate((500, 0));

    result.extend(canvas.contents.iter().enumerate().map(|(i, p)| {
        if *p != ' ' {
            return GraphNode {
                end: true,
                children: vec![],
            };
        }

        let down = i + canvas.width as usize;
        let down_and_left = down - 1;
        let down_and_right = down + 1;
        let children = [down, down_and_left, down_and_right]
            .into_iter()
            .map(|j| (j, canvas.contents[j]))
            .filter(|(_, c)| *c != '█')
            .map(|(j, _)| j)
            .collect_vec();

        GraphNode {
            children,
            end: false,
        }
    }));

    (start, result)
}

/// Perform a dfs of the graph from the given start point and return the number
/// of steps to get to a node marked `end`
fn dfs(
    start: usize,
    graph: &[GraphNode],
    canvas: &mut Canvas,
    stop_on_end: bool,
) -> usize {
    let mut stack = vec![start];
    let mut count = 0;

    let mut children_to_check = vec![];
    while !stack.is_empty() {
        let curr_index = *(stack.last().expect("🥶"));
        let curr = &graph[curr_index];

        if curr.end {
            if stop_on_end {
                break;
            }
            stack.pop();
            continue;
        }

        children_to_check.extend(
            curr.children
                .iter()
                .map(|c| (*c, canvas.contents[*c]))
                .filter(|(_, v)| stop_on_end || *v != '~')
                .filter(|(_, v)| *v != 'o')
                .map(|(c, _)| c)
                .rev(),
        );

        if children_to_check.is_empty() {
            stack.pop();
            count += 1;
            canvas.contents[curr_index] = 'o';
        } else {
            stack.append(&mut children_to_check);
        }
    }

    count
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut canvas = create_canvas(input_lines.map(parse_line), false);

    let (start, graph) = create_graph(&canvas);

    let result = dfs(start, &graph, &mut canvas, true);

    canvas.render();

    result
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut canvas = create_canvas(input_lines.map(parse_line), true);

    let (start, graph) = create_graph(&canvas);

    let result = dfs(start, &graph, &mut canvas, false);

    canvas.render();

    result
}
