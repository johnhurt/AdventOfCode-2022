use iter_tools::Itertools;
use lazy_static::lazy_static;
use std::{array::from_fn, fmt::Display};

lazy_static! {
    /// Each letter gets a bit in a u64 based on its point value
    static ref FLAGS: [u64; 26 * 2 + 1] = from_fn(|i| 1_u64 << i);
}

/// Convert an item letter into the points it represents
fn to_points(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => unreachable!("Invalid input not allowed"),
    }
}

/// Extract the information about which items are present in the form of a
/// u64 where each item is signified by a single bit
fn to_flags(items: &[u8]) -> u64 {
    items.iter().fold(0_u64, |flags, item| {
        flags | FLAGS[to_points(*item) as usize]
    })
}

fn check_for_p1_errors(items: &[u8]) -> u32 {
    let (left, right) = items.split_at(items.len() / 2);

    let left_flags = to_flags(left);
    let right_flags = to_flags(right);

    (left_flags & right_flags).trailing_zeros()
}

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(|line| check_for_p1_errors(line.as_bytes()))
        .sum::<u32>()
}

/**** Problem 2 ******/

fn find_badge(elf_1: &[u8], elf_2: &[u8], elf_3: &[u8]) -> u32 {
    let flags_1 = to_flags(elf_1);
    let flags_2 = to_flags(elf_2);
    let flags_3 = to_flags(elf_3);

    (flags_1 & flags_2 & flags_3).trailing_zeros()
}

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .tuples::<(String, String, String)>()
        .into_iter()
        .map(|(elf_1, elf_2, elf_3)| {
            find_badge(elf_1.as_bytes(), elf_2.as_bytes(), elf_3.as_bytes())
        })
        .sum::<u32>()
}

#[test]
fn test_to_points() {
    assert_eq!(1, to_points(b'a'));
    assert_eq!(26, to_points(b'z'));
    assert_eq!(27, to_points(b'A'));
    assert_eq!(52, to_points(b'Z'));
}

#[test]
fn test_to_flags() {
    assert_eq!(0b10_u64, to_flags(b"a"));
    assert_eq!(0b111000000000000000000000001110_u64, to_flags(b"abcABC"));
}

#[test]
fn test_check_for_p1_errors() {
    assert_eq!(check_for_p1_errors(b"aa"), 1);
}
