use lazy_static::lazy_static;
use std::{array::from_fn, fmt::Display};

lazy_static! {
    /// Each letter gets a bit in a u32 based on its index
    /// ( a -> 0b10, b -> 0b100 ... z -> 0b100<--26 total zeros-->00 )
    static ref FLAGS: [u32; 26 + 1] = from_fn(|i| 1_u32 << i);
}

/// Convert and ascii byte into an indexed letter starting at one.
/// Ex: (a -> 1, b -> 2 ... z -> 26)
///
/// We leave 0 as an indicator of an invalid letter
fn byte_to_letter(byte: u8) -> usize {
    (byte - b'a' + 1) as usize
}

/// Over-engineered structure for capturing a rolling frame of data from a
/// stream and detecting a frame that is made up of unique letters
#[derive(Default)]
struct UniqueSequenceFinder<const L: usize>
where
    [u8; L]: Default,
{
    buffer: [u8; L],
    count: usize,
    presence_flags: u32,
    overflows: [u32; 27],
    overflow_count: u32,
}

impl<const L: usize> UniqueSequenceFinder<L>
where
    [u8; L]: Default,
{
    /// Branchless method for progressing the stream one letter and checking
    /// for a full frame of unique letters
    fn append_and_detect(&mut self, in_letter: usize) -> bool {
        let cursor = self.count % L;

        // Determine the letter that's going to be removed
        let out_letter = self.buffer[cursor] as usize;

        // Determine if removing the letter from the buffer would remove a
        // duplicate
        let orig_overflows = self.overflows[out_letter];
        let new_overflows = orig_overflows.saturating_sub(1);
        let duplicate_removed = orig_overflows - new_overflows > 0;

        // The mask to be applied to the presence flags will:
        //  1. Do nothing if removing the existing letter removes a duplicate
        //  2. Otherwise set the bit representing the existing letter to zero
        let out_mask = !((!duplicate_removed) as u32 * FLAGS[out_letter]);

        // Apply the changes to the state from removing the old letter
        self.overflow_count =
            self.overflow_count.saturating_sub(duplicate_removed as u32);
        self.overflows[out_letter] = new_overflows;
        self.presence_flags &= out_mask;

        // Replace the removed letter with the new one
        self.buffer[cursor] = in_letter as u8;

        // Check to see if adding the new letter adds a duplicate
        let in_mask = FLAGS[in_letter];
        let new_presence_flags = self.presence_flags | in_mask;
        let duplicate_added = self.presence_flags == new_presence_flags;

        // Apply the changes to the state from adding the new letter
        self.presence_flags = new_presence_flags;
        self.overflow_count += duplicate_added as u32;
        self.overflows[in_letter] += duplicate_added as u32;
        self.count += 1;

        return self.count >= L && self.overflow_count == 0;
    }
}

pub fn problem_1<I>(mut input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut detector = UniqueSequenceFinder::<4>::default();

    input_lines
        .next()
        .expect("One line guaranteed")
        .into_bytes()
        .into_iter()
        .map(byte_to_letter)
        .take_while(move |c| !detector.append_and_detect(*c))
        .count()
        + 1
}

/**** Problem 2 ******/

pub fn problem_2<I>(mut input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut detector = UniqueSequenceFinder::<14>::default();

    input_lines
        .next()
        .expect("One line guaranteed")
        .into_bytes()
        .into_iter()
        .map(byte_to_letter)
        .take_while(move |c| !detector.append_and_detect(*c))
        .count()
        + 1
}
