use iter_tools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, space0, space1, u128, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::fmt::Display;

/// This big type made clippy angry
type ParseResult<'a> = IResult<&'a str, (u32, Vec<Item>, Op, u128, u32, u32)>;

/// Okay, so backstory. I originally did this problem by tracking the whole
/// history of an item's stress value, so that I could do the modulo operations
/// using the distributive property of modulo addition and multiplication.
/// Then I saw Steve and Saxon's solution and realized I could use the property
/// of modulo match that I hadn't though of which is:
///
/// (a % M) % N = a % N iff M % N == 0
///
/// So instead of dealing with multiple remainders for different divisors, I
/// could do all the math with a big modulo that was a multiple of all the
/// possible divisors used in this problem. Saxon and Steve used a specific set
/// of divisors, but I came up with this number (that I think will also work)
/// by finding the smallest number that had all number <= 46 as a factor.
///
/// The only only thing I don't like about this is using this method required
/// going to u128 whereas the original method was done with i32s
///
/// 2^5 * 3 ^3 * 5^2 * 7 * 11 * 13 * 17 * 19 * 23 * 29 * 31 * 37 * 41 * 43 * 47
const MAGIC_NUMBER: u128 = 9_419_588_158_802_421_600;

/// Operations a monkey can do?
#[derive(Debug, Clone, Copy)]
enum Op {
    Mul(u128),
    Square,
    Add(u128),
    Double,

    /// This is used to represent stress relief as real operation
    Div(u128),
}

impl Op {
    fn apply(&self, lhs: u128) -> u128 {
        match self {
            Op::Add(rhs) => lhs + rhs,
            Op::Mul(rhs) => lhs * rhs,
            Op::Square => lhs * lhs,
            Op::Double => lhs + lhs,
            Op::Div(rhs) => lhs / rhs,
        }
    }
}

/// Representation of an item's stress level
#[derive(Debug)]
#[repr(transparent)]
struct Item(u128);

impl Item {
    fn new(initial_value: u128) -> Self {
        Self(initial_value)
    }

    /// Apply the given operation to this item's stress level.
    fn apply(&mut self, op: Op) {
        self.0 = op.apply(self.0) % MAGIC_NUMBER;
    }

    fn modulo(&mut self, divisor: u128) -> u128 {
        self.0 % divisor
    }
}

/// Hey, hey, it's a monkey
#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<Item>,
    op: Op,
    modulo: u128,
    true_target: usize,
    false_target: usize,
    inspect_count: usize,
}

impl Monkey {
    /// have the current monkey remove all its current items and inspect them
    /// based on the rules in the problem
    fn inspect(
        &mut self,
        relieve_stress: bool,
    ) -> impl Iterator<Item = (usize, Item)> + '_ {
        self.items
            .drain(..)
            .map(|mut item| {
                item.apply(self.op);
                item
            })
            .map(move |mut item| {
                relieve_stress.then(|| item.apply(Op::Div(3)));
                item
            })
            .map(|mut item| {
                self.inspect_count += 1;
                if item.modulo(self.modulo) == 0 {
                    (self.true_target, item)
                } else {
                    (self.false_target, item)
                }
            })
    }
}

/****************** Parsing ******************/

macro_rules! between {
    ($before:expr, $pat:expr, $after:expr) => {
        preceded($before, terminated($pat, terminated($after, space0)))
    };
}

fn parse_monkey_id(input: &str) -> IResult<&'_ str, u32> {
    between!(tag("Monkey "), u32, tag(":\n"))(input)
}

fn parse_items(input: &str) -> IResult<&'_ str, Vec<Item>> {
    between!(
        tag("Starting items: "),
        separated_list1(tag(", "), map(u128, Item::new)),
        char('\n')
    )(input)
}

fn parse_op_literal(input: &str) -> IResult<&'_ str, Op> {
    map(
        between!(
            tag("Operation: new = old "),
            separated_pair(anychar, space1, u128),
            char('\n')
        ),
        |(op_char, rhs)| match op_char {
            '*' => Op::Mul(rhs),
            '+' => Op::Add(rhs),
            _ => unreachable!("😫"),
        },
    )(input)
}

fn parse_op_reference(input: &str) -> IResult<&'_ str, Op> {
    map(
        between!(tag("Operation: new = old "), anychar, tag(" old\n")),
        |op_char| match op_char {
            '*' => Op::Square,
            '+' => Op::Double,
            _ => unreachable!("🤯"),
        },
    )(input)
}

fn parse_test(input: &str) -> IResult<&'_ str, u128> {
    between!(tag("Test: divisible by "), u128, char('\n'))(input)
}

fn parse_true_target(input: &str) -> IResult<&'_ str, u32> {
    between!(tag("If true: throw to monkey "), u32, char('\n'))(input)
}

fn parse_false_target(input: &str) -> IResult<&'_ str, u32> {
    preceded(tag("If false: throw to monkey "), u32)(input)
}

fn parse_monkey(block: String) -> Monkey {
    let parse_result: ParseResult<'_> = tuple((
        parse_monkey_id,
        parse_items,
        alt((parse_op_literal, parse_op_reference)),
        parse_test,
        parse_true_target,
        parse_false_target,
    ))(&block);

    let (id, items, op, modulo, t, f) = parse_result.expect("🥸").1;

    Monkey {
        id: id as usize,
        items,
        op,
        modulo,
        true_target: t as usize,
        false_target: f as usize,
        inspect_count: 0,
    }
}

/// Convenience macro to read, parse, and evaluate the monkeys' actions for
/// a given number of rounds with/out stress
macro_rules! run_monkeys {
    ($input_lines:ident -> $rounds:literal rounds) => {
        run_monkeys!(__internal, $input_lines, $rounds, true)
    };

    ($input_lines:ident -> $rounds:literal rounds without stress relief $e:expr) => {
        run_monkeys!(__internal, $input_lines, $rounds, false)
    };

    (__internal, $input_lines:ident, $rounds:literal, $stress_relief:literal) => {{
        let mut line_group = 0;
        let groups = $input_lines.group_by(|line| {
            line.is_empty().then(|| line_group += 1);
            line_group
        });

        let mut monkeys = groups
            .into_iter()
            .map(|group| group.1.filter(|line| !line.is_empty()).join("\n"))
            .map(parse_monkey)
            .collect::<Vec<_>>();

        let mut throws = vec![];

        (0..$rounds).for_each(|_| {
            for monkey_id in 0..monkeys.len() {
                throws.extend(monkeys[monkey_id].inspect($stress_relief));
                throws
                    .drain(..)
                    .for_each(|(id, item)| monkeys[id].items.push(item));
            }
        });

        monkeys
            .iter()
            .map(|m| m.inspect_count)
            .sorted()
            .rev()
            .take(2)
            .product::<usize>()
    }};
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    run_monkeys!(input_lines -> 20 rounds)
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    run_monkeys!(input_lines -> 10_000 rounds without stress relief |a| a)
}
