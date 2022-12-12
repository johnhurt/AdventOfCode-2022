use std::{cell::RefCell, fmt::Display, rc::Rc};

use iter_tools::Itertools;
use nom::{
    character::complete::{alpha1, anychar, char, i32},
    combinator::opt,
    multi::many0,
    sequence::{separated_pair, terminated},
    IResult,
};

/// Hmm, a suspiciously small set of instructions
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn expand(self, start_cycle: usize) -> Cycle {
        Cycle {
            instruction: self,
            number: start_cycle,
            phase: CyclePhase::Mid,
            inner_index: 0,
        }
    }

    fn cycle_count(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

impl Iterator for Cycle {
    type Item = Cycle;

    fn next(&mut self) -> Option<Self::Item> {
        use CyclePhase::*;

        (0..self.instruction.cycle_count())
            .contains(&self.inner_index)
            .then(|| {
                let result = *self;

                // Progress to the next phase and instruction
                match self.phase {
                    Mid => self.phase = End,
                    End => {
                        self.phase = Mid;
                        self.inner_index += 1;
                        self.number += 1;
                    }
                };

                result
            })
    }
}

/// Over-exaggerated instruction lifecycle phases
#[derive(Debug, Clone, Copy, PartialEq)]
enum CyclePhase {
    Mid,
    End,
}

/// Seriously overly-engineered representation of a compute cycle
#[derive(Debug, Clone, Copy)]
struct Cycle {
    number: usize,
    phase: CyclePhase,
    instruction: Instruction,
    inner_index: usize,
}

/// Create a closure that will track the cycle number and
fn expand_instructions_to_cycles() -> impl FnMut(Instruction) -> Cycle {
    let mut cycle_num = 1;
    move |instruction| {
        let result = instruction.expand(cycle_num);
        cycle_num += result.instruction.cycle_count();
        result
    }
}

/// Parse an instruction from a line of "assembly"
fn parse_move_line(line: String) -> Instruction {
    let result: IResult<&str, (char, Option<i32>)> = separated_pair(
        terminated(anychar, alpha1),
        many0(char(' ')),
        opt(i32),
    )(&line);

    match result.expect("That's no moon; it's a parsing error").1 {
        ('n', None) => Instruction::Noop,
        ('a', Some(dx)) => Instruction::Addx(dx),
        _ => unreachable!("😤"),
    }
}

/// Create a closure that evaluates the "assembly" and emits the current value
/// of 'x' at each phase of the instruction's lifecycle
fn evaluate() -> impl FnMut(Cycle) -> (Cycle, i32) {
    use CyclePhase::*;
    use Instruction::*;

    let mut x = 1;

    move |cycle| {
        if let Cycle {
            instruction: Addx(dx),
            phase: End,
            inner_index: 1,
            ..
        } = cycle
        {
            x += dx;
        }
        (cycle, x)
    }
}

/// Wrapper around an i32 to allow us to add a pixel checking function
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct Sprite(i32);

impl Sprite {
    /// Check if the given x coordinate is covered by the sprite
    fn contains(&self, x: i32) -> bool {
        ((self.0 - 1)..=(self.0 + 1)).contains(&x)
    }
}

/// Create a rendering system that is composed of a closure for drawing and a
/// readable buffer for viewing
fn create_renderer() -> (Rc<RefCell<Vec<char>>>, impl FnMut((Cycle, Sprite))) {
    let buffer = Rc::new(RefCell::new(vec![' '; 6 * 40]));

    (buffer.clone(), move |(Cycle { number, .. }, sprite)| {
        let i = number - 1;
        let x = i % 40;

        if sprite.contains(x as i32) {
            buffer.borrow_mut()[i] = '█';
        }
    })
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(parse_move_line)
        .flat_map(expand_instructions_to_cycles())
        .map(evaluate())
        .filter(|c| c.0.phase == CyclePhase::Mid)
        .map(|(Cycle { number, .. }, x)| (number, x))
        .filter(|(n, _)| (n + 60) % 40 == 0)
        .map(|(n, x)| n * x as usize)
        .sum::<usize>()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let (buffer, renderer) = create_renderer();

    input_lines
        .map(parse_move_line)
        .flat_map(expand_instructions_to_cycles())
        .map(evaluate())
        .filter(|c| c.0.phase == CyclePhase::Mid)
        .map(|(c, x)| (c, Sprite(x)))
        .for_each(renderer);

    format!(
        "\n           \t                       {}",
        buffer
            .borrow()
            .chunks(40)
            .map(|chunk| chunk.iter().join(""))
            .join("\n           \t                       ")
    )
}
