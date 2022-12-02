use std::fmt::Display;

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(|line| line.parse::<i32>().ok())
        .chain(Some(None)) // Count the last elf
        .fold((0, 0), |(max, curr), calories_opt| match calories_opt {
            Some(calories) => (max, curr + calories),
            None => (max.max(curr), 0),
        })
        .0
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let (a, b, c, _) = input_lines
        .map(|line| line.parse::<i32>().ok())
        .chain(Some(None)) // Count the last elf
        .fold((0, 0, 0, 0), |(max_1, max_2, max_3, curr), v| match v {
            Some(c) => (max_1, max_2, max_3, curr + c),
            None if curr > max_1 => (curr, max_1, max_2, 0),
            None if curr > max_2 => (max_1, curr, max_2, 0),
            None if curr > max_3 => (max_1, max_2, curr, 0),
            _ => (max_1, max_2, max_3, 0),
        });

    a + b + c
}
