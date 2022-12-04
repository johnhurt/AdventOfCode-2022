use std::fmt::Display;

use iter_tools::Itertools;

/// parse a line from the schedule into a tuple of the 4 numbers represented
/// by the schedule: (start_1, end_1, start_2, end_2)
fn parse_schedule_line(line: String) -> (u32, u32, u32, u32) {
    line.splitn(4, &['-', ','])
        .map(|str_val| str_val.parse::<u32>().expect("Valid input guaranteed"))
        .next_tuple()
        .expect("Valid input guaranteed")
}

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(parse_schedule_line)
        .filter_map(|(s_1, e_1, s_2, e_2)| {
            match (s_1 > s_2, s_1 < s_2, e_1 > e_2, e_1 < e_2) {
                (true, _, true, _) | (_, true, _, true) => None,
                _ => Some(1),
            }
        })
        .sum::<i32>()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(parse_schedule_line)
        .filter_map(|(s_1, e_1, s_2, e_2)| {
            (!(e_1 < s_2 || e_2 < s_1)).then(|| 1)
        })
        .sum::<i32>()
}
