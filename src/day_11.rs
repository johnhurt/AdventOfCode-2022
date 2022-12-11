use iter_tools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, i32, space0, space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::{collections::HashMap, fmt::Display};

/// Operations a monkey can do?
#[derive(Debug, Clone, Copy)]
enum Op {
    Mul(i32),
    Square,
    Add(i32),
    Double,

    /// This is used to represent stress relief as real operation
    Div(i32),
}

impl Op {
    fn apply(&self, lhs: i32) -> i32 {
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
struct Item {
    initial_value: i32,
    direct_computation: bool,
    history: Vec<Op>,

    // This is only here to speed up the computation
    mod_cache: HashMap<i32, i32>,
}

impl Item {
    fn new(initial_value: i32) -> Self {
        Self {
            initial_value,
            direct_computation: false,
            history: vec![],
            mod_cache: Default::default(),
        }
    }

    /// Apply the given operation to this item's stress level. This operation
    /// will update any cached modulos' values
    fn apply(&mut self, op: Op) {
        if matches!(op, Op::Div(_)) {
            // If the history of the item includes a divide, we cannot use the
            // properties of modulo to calculate an answer for a big number,
            // we have to calculate directly
            self.direct_computation = true;
        }

        // Caching isn't useful if we are computing modulos directly
        if !self.direct_computation {
            self.mod_cache
                .iter_mut()
                .for_each(|(modulo, curr)| *curr = op.apply(*curr) % modulo);
        }

        self.history.push(op);
    }

    fn cache_calculated_modulo(&mut self, divisor: i32) -> i32 {
        let result = self
            .history
            .iter()
            .fold(self.initial_value % divisor, |prev, op| {
                op.apply(prev) % divisor
            });

        self.mod_cache.insert(divisor, result);

        result
    }

    /// Compute the remainder of dividing this item's current stress level by
    /// the given value. This will be done directly for part 1, but for part 2
    /// we use the properties:
    ///
    /// (a + b) % N = (a % N) + (b % N)
    /// (a * b) % N = (a % N) * (b % N)
    ///
    fn modulo(&mut self, divisor: i32) -> i32 {
        if self.direct_computation {
            let mut val = self.initial_value;
            self.history.iter().for_each(|op| val = op.apply(val));
            val % divisor
        } else {
            self.mod_cache
                .get(&divisor)
                .cloned()
                .unwrap_or_else(|| self.cache_calculated_modulo(divisor))
        }
    }
}

/// Hey, hey, it's a monkey
#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<Item>,
    op: Op,
    modulo: i32,
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
        separated_list1(tag(", "), map(i32, Item::new)),
        char('\n')
    )(input)
}

fn parse_op_literal(input: &str) -> IResult<&'_ str, Op> {
    map(
        between!(
            tag("Operation: new = old "),
            separated_pair(anychar, space1, i32),
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

fn parse_test(input: &str) -> IResult<&'_ str, i32> {
    between!(tag("Test: divisible by "), i32, char('\n'))(input)
}

fn parse_true_target(input: &str) -> IResult<&'_ str, u32> {
    between!(tag("If true: throw to monkey "), u32, char('\n'))(input)
}

fn parse_false_target(input: &str) -> IResult<&'_ str, u32> {
    preceded(tag("If false: throw to monkey "), u32)(input)
}

fn parse_monkey(block: String) -> Monkey {
    let parse_result: IResult<&str, (u32, Vec<Item>, Op, i32, u32, u32)> =
        tuple((
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

    ($input_lines:ident -> $rounds:literal rounds without stress relief) => {
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
    run_monkeys!(input_lines -> 10_000 rounds without stress relief)
}
