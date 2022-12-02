use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
enum RPS {
    R = 1,
    P = 2,
    S = 3,
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
enum LDW {
    L = 0,
    D = 3,
    W = 6,
}

fn to_rps(i: u8) -> RPS {
    use RPS::*;
    match i {
        b'A' | b'X' => R,
        b'B' | b'Y' => P,
        b'C' | b'Z' => S,
        _ => unreachable!("Input guaranteed to be valid"),
    }
}

fn to_ldw(i: u8) -> LDW {
    use LDW::*;
    match i {
        b'X' => L,
        b'Y' => D,
        b'Z' => W,
        _ => unreachable!("Input guaranteed to be valid"),
    }
}

impl RPS {
    fn score_p1((other, mine): (Self, Self)) -> i32 {
        use RPS::*;
        match (other, mine) {
            (R, S) | (P, R) | (S, P) => 0 + mine as i32, // Lose
            (R, R) | (P, P) | (S, S) => 3 + mine as i32, // Draw
            (R, P) | (P, S) | (S, R) => 6 + mine as i32, // Win
        }
    }

    fn score_p2((other, target): (Self, LDW)) -> i32 {
        use LDW::*;
        use RPS::*;
        match (other, target) {
            (R, D) | (P, L) | (S, W) => 1 + target as i32, // Rock
            (R, W) | (P, D) | (S, L) => 2 + target as i32, // Paper
            (R, L) | (P, W) | (S, D) => 3 + target as i32, // Scissors
        }
    }
}

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(|line| line.into_bytes())
        .map(|bytes| (to_rps(bytes[0]), to_rps(bytes[2])))
        .map(RPS::score_p1)
        .sum::<i32>()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(|line| line.into_bytes())
        .map(|bytes| (to_rps(bytes[0]), to_ldw(bytes[2])))
        .map(RPS::score_p2)
        .sum::<i32>()
}
