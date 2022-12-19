use iter_tools::Itertools;
use std::fmt::{format, Display};

const HORIZ: [u16; 4] = [0b000111100, 0, 0, 0];
const PLUS: [u16; 4] = [0b000010000, 0b000111000, 0b000010000, 0];
const BEND: [u16; 4] = [0b000111000, 0b000001000, 0b000001000, 0];
const VERT: [u16; 4] = [0b000100000, 0b000100000, 0b000100000, 0b000100000];
const SQR: [u16; 4] = [0b000110000, 0b000110000, 0, 0];

const WALLS: u16 = 0b100000001;
const FLOOR: u16 = 0b111111111;

const DEPTH: usize = 1_000;

#[derive(Debug, Clone, Copy)]
enum Delta {
    Left,
    Right,
    Down,
}

impl From<u8> for Delta {
    fn from(c: u8) -> Self {
        match c {
            b'>' => Delta::Right,
            b'<' => Delta::Left,
            _ => unreachable!("👻"),
        }
    }
}

struct WindSource {
    line: Vec<u8>,
    cursor: usize,
}

impl WindSource {
    fn new(line: String) -> Self {
        Self {
            line: line.into_bytes(),
            cursor: 0,
        }
    }
}

impl Iterator for WindSource {
    type Item = Delta;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.line[self.cursor % self.line.len()];
        self.cursor += 1;
        Some(result.into())
    }
}

fn replace(target: &mut [u16], source: &[u16]) {
    target
        .iter_mut()
        .zip(source.iter())
        .for_each(|(t, s)| *t = *s);
}

#[derive(Debug, Clone, Copy, Default)]
enum Rock {
    Horiz,
    Plus,
    Bend,
    Vert,

    #[default]
    Sqr,
}

impl Rock {
    fn lines(&self) -> usize {
        use Rock::*;

        match self {
            Horiz => 1,
            Plus => 3,
            Bend => 3,
            Vert => 4,
            Sqr => 2,
        }
    }

    fn next(&self) -> Self {
        use Rock::*;

        match self {
            Horiz => Plus,
            Plus => Bend,
            Bend => Vert,
            Vert => Sqr,
            Sqr => Horiz,
        }
    }

    fn start(&self) -> [u16; 4] {
        use Rock::*;

        match self {
            Horiz => HORIZ,
            Plus => PLUS,
            Bend => BEND,
            Vert => VERT,
            Sqr => SQR,
        }
    }
}

#[derive(Debug, Default)]
struct RockSource {
    last_rock: Rock,
}

impl Iterator for RockSource {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_rock = self.last_rock.next();
        Some(self.last_rock)
    }
}

struct MoveIter<'a> {
    wind: &'a mut WindSource,
    fell_last: bool,
}

impl<'a> Iterator for MoveIter<'a> {
    type Item = Delta;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fell_last {
            self.fell_last = false;
            self.wind.next()
        } else {
            self.fell_last = true;
            Some(Delta::Down)
        }
    }
}

#[derive(Debug, Default)]
struct FallingRock {
    space: [u16; 4],
    bottom: usize,
    lines: usize,
}

impl FallingRock {
    fn new(rock: Rock, bottom: usize) -> Self {
        FallingRock {
            space: rock.start(),
            bottom,
            lines: rock.lines(),
        }
    }

    fn apply(&mut self, delta: Delta) {
        use Delta::*;

        match delta {
            Down => self.bottom -= 1,
            Left => self.space.iter_mut().for_each(|r| *r <<= 1),
            Right => self.space.iter_mut().for_each(|r| *r >>= 1),
        }
    }

    fn undo(&mut self, delta: Delta) {
        use Delta::*;

        match delta {
            Down => self.bottom += 1,
            Left => self.space.iter_mut().for_each(|r| *r >>= 1),
            Right => self.space.iter_mut().for_each(|r| *r <<= 1),
        }
    }
}

struct Chamber {
    board: Vec<u16>,
    height: usize,
    len: usize,
}

impl Chamber {
    fn new() -> Self {
        let mut result = Self {
            board: vec![0; DEPTH],
            height: 1,
            len: 1,
        };

        result.board[0] = FLOOR;

        result
    }

    fn check_resize(&mut self, level: usize) {
        if level <= self.len {
            return;
        }
        let diff = level - self.len;

        let new_len = self.len + diff;

        for i in self.len..new_len {
            self.board[i % DEPTH] = WALLS;
        }

        self.len = new_len;
    }

    fn check_interference(&self, rock: &FallingRock) -> bool {
        for i in 0..rock.lines {
            if self.board[(rock.bottom + i) % DEPTH] & rock.space[i] > 0 {
                return true;
            }
        }

        false
    }

    fn finalize(&mut self, rock: FallingRock) -> usize {
        for i in 0..rock.lines {
            self.board[(rock.bottom + i) % DEPTH] |= rock.space[i];
        }

        self.height = self.height.max(rock.bottom + rock.lines);

        // println!(
        //     "\n{}\n",
        //     self.board
        //         .iter()
        //         .skip(1)
        //         .rev()
        //         .map(|v| format!("{v:b}"))
        //         .filter(|s| s.len() > 1)
        //         .map(|s| s[1..8].replace("0", ".").replace("1", "#"))
        //         .join("\n")
        // );

        self.height - 1
    }

    fn apply(&mut self, wind: &mut WindSource, rock: Rock) -> usize {
        let bottom = self.height + 3;
        let top = bottom + rock.lines();
        let mut rock = FallingRock::new(rock, bottom);

        self.check_resize(top);

        let move_src = MoveIter {
            wind,
            fell_last: true,
        };

        for delta in move_src {
            rock.apply(delta);

            // println!("\n{delta:?}");

            // println!(
            //     "\n{}\n",
            //     rock.space
            //         .iter()
            //         .rev()
            //         .map(|v| format!("{v:#011b}"))
            //         .filter(|s| s.len() > 1)
            //         .map(|s| s[3..11].replace("0", ".").replace("1", "#"))
            //         .join("\n")
            // );

            if self.check_interference(&rock) {
                rock.undo(delta);
                if matches!(delta, Delta::Down) {
                    break;
                }
            }
        }

        self.finalize(rock)
    }
}

/**** Problem 1 ******/

pub fn problem_1<I>(mut input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut wind = WindSource::new(input_lines.next().expect("😱"));
    let mut chamber = Chamber::new();
    let rocks = RockSource::default();

    rocks
        .enumerate()
        .map(|(i, rock)| {
            println!("{i}");
            chamber.apply(&mut wind, rock)
        })
        .nth(2021)
        .expect("🤢")
}

/**** Problem 2 ******/

pub fn problem_2<I>(mut input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut wind = WindSource::new(input_lines.next().expect("😱"));
    let mut chamber = Chamber::new();
    let rocks = RockSource::default();

    rocks
        .enumerate()
        .map(|(i, rock)| {
            if i % 100_000_000 == 0 {
                println!("{}/1T -> {}%", i, i as f64 / 1e12 * 100.);
            }
            chamber.apply(&mut wind, rock)
        })
        .nth(1_000_000_000_000 - 1)
        .expect("🤢")
}
