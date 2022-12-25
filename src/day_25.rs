use std::fmt::Display;

use iter_tools::{EitherOrBoth, Itertools};

fn add_single_digit(left: char, right: char) -> (char, char) {
    match (left, right) {
        ('=', '=') => ('1', '-'), // -2 + -2 = -4 => -5 + 1 => -1
        ('=', '-') | ('-', '=') => ('2', '-'), // -2 + -1 = -3 => -5 + 2 => -2
        (s, '0') | ('0', s) => (s, '0'),
        ('=', '1') | ('1', '=') => ('-', '0'),
        ('=', '2') | ('2', '=') | ('-', '1') | ('1', '-') => ('0', '0'),
        ('-', '2') | ('2', '-') => ('1', '0'),
        ('-', '-') => ('=', '0'),
        ('1', '1') => ('2', '0'),
        ('2', '1') | ('1', '2') => ('=', '1'),
        ('2', '2') => ('-', '1'),
        _ => unreachable!("🤭"),
    }
}

fn add_snafu(left: &[char], right: &[char]) -> Vec<char> {
    use EitherOrBoth::*;
    let mut carry_over = '0';
    let mut r = left
        .iter()
        .zip_longest(right.iter())
        .map(|pair| match pair {
            Both(l, r) => (*l, *r),
            Left(v) | Right(v) => (*v, '0'),
        })
        .map(|(l, r)| {
            let (tot1, carry1) = add_single_digit(l, r);
            let (tot2, carry2) = add_single_digit(tot1, carry_over);
            carry_over = add_single_digit(carry1, carry2).0;
            tot2
        })
        .collect_vec();

    if carry_over != '0' {
        r.push(carry_over);
    }

    r
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(|line| line.chars().rev().collect_vec())
        .collect_vec()
        .into_iter()
        .rev()
        .fold(vec!['0'], |sum, curr| add_snafu(&sum, &curr))
        .into_iter()
        .rev()
        .collect::<String>()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    "Start the blender!".to_owned()
}
