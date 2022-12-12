use iter_tools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, not_line_ending, u32},
    combinator::{map, recognize, value},
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};
use std::{collections::BTreeMap, fmt::Display};

/// All the operations done while traversing the file system
#[derive(Debug, Clone)]
enum Operation {
    /// `cd ..`
    CdUp,

    /// `cd` to the contained relative path (One path component)
    CdDown(String),

    /// List the contents, but we only keep the total size of all files listed
    Ls(u32),
}

/********************* Awesome Nom Parsing **********************/

/// Consume a line showing a directory in the `ls` output
fn parse_listed_directory(input: &str) -> IResult<&'_ str, u32> {
    value(0, recognize(preceded(tag("dir "), alpha1)))(input)
}

/// Parse a line showing a file entry in the `ls` output
fn parse_listed_file(input: &str) -> IResult<&'_ str, u32> {
    terminated(u32, not_line_ending)(input)
}

/// Parse either a file or directory in the `ls` output
fn parse_ls_entry(input: &str) -> IResult<&'_ str, u32> {
    alt((parse_listed_directory, parse_listed_file))(input)
}

/// Parse the entire `ls` command and output
fn parse_ls(command_and_response: &str) -> IResult<&'_ str, Operation> {
    map(
        preceded(tag("ls\n"), separated_list0(char('\n'), parse_ls_entry)),
        |listings| Operation::Ls(listings.iter().sum::<u32>()),
    )(command_and_response)
}

/// Parse a `cd` command of any type
fn parse_cd(command_line: &str) -> IResult<&'_ str, Operation> {
    alt((
        value(Operation::CdUp, tag("cd ..")),
        map(preceded(tag("cd "), not_line_ending), |dir: &str| {
            Operation::CdDown(dir.to_owned())
        }),
    ))(command_line)
}

/// Parse the command and output into an operation
fn parse_op(command_and_output: String) -> Operation {
    preceded(tag("$ "), alt((parse_cd, parse_ls)))(&command_and_output)
        .expect("Bad input not allowed")
        .1
}

/***** Walking the file system and replaying the commands *******/

/// Recursively walk the directory structure by evaluating the operations.
/// Apply the given accumulator on each directory and return the total size of
/// the current directory
fn fs_walker<C, A>(dir: String, commands: &mut C, accumulator: &mut A) -> u32
where
    C: Iterator<Item = Operation>,
    A: FnMut(String, u32),
{
    use Operation::*;

    let mut curr_size = 0;

    while let Some(command) = commands.next() {
        match command {
            Ls(local_file_size) => curr_size += local_file_size,
            CdDown(target) => {
                curr_size += fs_walker(target, commands, accumulator)
            }
            CdUp => break,
        }
    }

    accumulator(dir, curr_size);

    curr_size
}

/// Convenience macro to process the raw input lines and convert them to ops,
/// and run the given accumulator. The macro will return the total size of all
/// files in the file system
macro_rules! walk_directories {
    ($input_lines:ident, $accumulator:expr) => {
        {
            let mut command_num = 0;
            let command_groups = $input_lines.group_by(move |line| {
                line.starts_with('$').then(|| command_num += 1);
                command_num
            });

            let mut commands = command_groups
                .into_iter()
                .map(|(_, mut command_lines)| parse_op(command_lines.join("\n")));

            fs_walker(String::new(), &mut commands, &mut $accumulator)
        }
    };
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut result = 0;
    walk_directories!(input_lines, |dir, size| {
        (size <= 100_000).then(|| result += size);
    });
    result
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut sizes = BTreeMap::new();
    let total_size = walk_directories!(input_lines, |dir, size| {
        sizes.insert(size, dir);
    });

    let free_space = 70_000_000 - total_size;
    let minimum_file_size = 30_000_000 - free_space;

    sizes
        .split_off(&minimum_file_size)
        .into_iter()
        .next()
        .expect("A valid answer should be in this range")
        .0
}
