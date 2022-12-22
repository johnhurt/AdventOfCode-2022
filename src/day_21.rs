use std::{collections::HashMap, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, i64},
    combinator::map,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Operand {
    Name(String),
    Value(i64),
}

#[derive(Debug)]
enum MonkeyNode {
    Literal(i64),
    Statement(Operand, Op, Operand),
    Req(Operand, Operand),
    Human,
}

fn parse_literal_node(input: &str) -> IResult<&'_ str, MonkeyNode> {
    map(i64, MonkeyNode::Literal)(input)
}

fn parse_operand(input: &str) -> IResult<&'_ str, Operand> {
    alt((
        map(i64, Operand::Value),
        map(alpha1, |name: &str| Operand::Name(name.to_owned())),
    ))(input)
}

fn parse_op(input: &str) -> IResult<&'_ str, Op> {
    map(
        preceded(char(' '), terminated(anychar, char(' '))),
        |c| match c {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            _ => unreachable!("🙈"),
        },
    )(input)
}

fn parse_stmt_node(input: &str) -> IResult<&'_ str, MonkeyNode> {
    map(
        tuple((parse_operand, parse_op, parse_operand)),
        |(lhs, op, rhs)| MonkeyNode::Statement(lhs, op, rhs),
    )(input)
}

fn parse_monkey_node(input: &str) -> IResult<&'_ str, MonkeyNode> {
    preceded(tag(": "), alt((parse_literal_node, parse_stmt_node)))(input)
}

fn parse_monkey(line: String) -> (String, MonkeyNode) {
    let result: IResult<&str, (String, MonkeyNode)> =
        tuple((map(alpha1, str::to_owned), parse_monkey_node))(&line);

    result.expect("👹").1
}

impl Op {
    fn apply(self, lhs: i64, rhs: i64) -> i64 {
        use Op::*;

        match self {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs,
        }
    }

    // Take the inverse of this operation to find the rhs of the expression
    // ans = lhs {op} ?
    fn solve_for_right(self, ans: i64, lhs: i64) -> i64 {
        use Op::*;

        match self {
            Add => ans - lhs,
            Sub => lhs - ans,
            Mul => ans / lhs,
            Div => lhs / ans,
        }
    }

    // Take the inverse of this operation to find the lhs of the expression
    // ans = ? {op} rhs
    fn solve_for_left(self, ans: i64, rhs: i64) -> i64 {
        use Op::*;

        match self {
            Add => ans - rhs,
            Sub => rhs + ans,
            Mul => ans / rhs,
            Div => rhs * ans,
        }
    }
}

impl Operand {
    fn eval(&self, knowns: &HashMap<String, i64>) -> Option<i64> {
        use Operand::*;
        match self {
            Name(name) => knowns.get(name).copied(),
            Value(val) => Some(*val),
        }
    }
}

impl MonkeyNode {
    fn eval(&self, knowns: &HashMap<String, i64>) -> Option<i64> {
        use MonkeyNode::*;
        match self {
            Literal(val) => Some(*val),
            Statement(lhs, op, rhs) => {
                let left = lhs.eval(knowns)?;
                let right = rhs.eval(knowns)?;
                Some(op.apply(left, right))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SolverNode {
    LeftKnown(i64, Op, usize),
    RightKnown(usize, Op, i64),
    Root(usize, i64),
    Humn,
}

impl SolverNode {
    fn new(
        orig: MonkeyNode,
        knowns: &HashMap<String, i64>,
        name_to_id: &HashMap<String, usize>,
    ) -> Self {
        use MonkeyNode::*;
        use Operand::*;
        use SolverNode::*;

        let extract_id = |operand: Operand| -> usize {
            if let Name(name) = operand {
                name_to_id.get(&name).copied().expect("🤠")
            } else {
                unreachable!("The node must be present in unknown list")
            }
        };

        match orig {
            Human => Humn,
            Statement(lhs, op, rhs) => {
                match (lhs.eval(knowns), rhs.eval(knowns)) {
                    (Some(val), None) => LeftKnown(val, op, extract_id(rhs)),
                    (None, Some(val)) => RightKnown(extract_id(lhs), op, val),
                    _ => unreachable!("Nothing else makes sense here"),
                }
            }
            Req(lhs, rhs) => match (lhs.eval(knowns), rhs.eval(knowns)) {
                (Some(val), None) => Root(extract_id(rhs), val),
                (None, Some(val)) => Root(extract_id(lhs), val),
                _ => unreachable!("Nothing else makes sense here"),
            },
            _ => unreachable!("There shouldn't be any literals here"),
        }
    }
}

fn solve_for_humn(root: usize, nodes: Vec<SolverNode>) -> i64 {
    use SolverNode::*;

    let mut curr = root;
    let mut result = 0;

    loop {
        match nodes.get(curr).expect("🦀") {
            Root(next, val) => {
                curr = *next;
                result = *val;
            }
            LeftKnown(val, op, next) => {
                curr = *next;
                result = op.solve_for_right(result, *val);
            }
            RightKnown(next, op, val) => {
                curr = *next;
                result = op.solve_for_left(result, *val);
            }
            Humn => break,
        }
    }

    result
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut knowns = HashMap::new();
    let mut unknowns = input_lines
        .map(parse_monkey)
        .filter_map(|(name, node)| {
            if let Some(val) = node.eval(&knowns) {
                knowns.insert(name, val);
                None
            } else {
                Some((name, node))
            }
        })
        .collect::<Vec<_>>();

    let mut new_unknowns = vec![];
    while !knowns.contains_key("root") {
        new_unknowns.extend(unknowns.drain(..).filter_map(|(name, node)| {
            if let Some(val) = node.eval(&knowns) {
                knowns.insert(name, val);
                None
            } else {
                Some((name, node))
            }
        }));

        std::mem::swap(&mut new_unknowns, &mut unknowns);
    }

    *knowns.get("root").expect("🐒")
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut knowns = HashMap::new();

    let mut unknowns = input_lines
        .map(parse_monkey)
        .filter_map(|(name, mut node)| {
            if &name == "root" {
                if let MonkeyNode::Statement(lhs, _, rhs) = node {
                    node = MonkeyNode::Req(lhs, rhs)
                } else {
                    unreachable!("root shouldn't be a constant");
                }
            }
            if &name == "humn" {
                node = MonkeyNode::Human;
            }

            if let Some(val) = node.eval(&knowns) {
                knowns.insert(name, val);
                None
            } else {
                Some((name, node))
            }
        })
        .collect::<Vec<_>>();

    let mut new_unknowns = vec![];

    let mut resolutions = 1;

    while resolutions > 0 {
        resolutions = 0;
        new_unknowns.extend(unknowns.drain(..).filter_map(|(name, node)| {
            if let Some(val) = node.eval(&knowns) {
                knowns.insert(name, val);
                resolutions += 1;
                None
            } else {
                Some((name, node))
            }
        }));

        std::mem::swap(&mut new_unknowns, &mut unknowns);
    }

    // At this point all nodes that can be evaluated have been evaluated.
    // We need to find the chain that looks like
    //
    // root -> m1 -> m2 -> m3 -> humn
    //
    // To find out what humn's value is we need to rearrange each node to solve
    // for the unknown in the stmt.

    let name_to_id = unknowns
        .iter()
        .enumerate()
        .map(|(i, (n, _))| (n.clone(), i))
        .collect::<HashMap<_, _>>();

    let to_solve = unknowns
        .into_iter()
        .map(|(_, node)| SolverNode::new(node, &knowns, &name_to_id))
        .collect::<Vec<_>>();

    let root = *name_to_id.get("root").expect("🦧");
    solve_for_humn(root, to_solve)
}
