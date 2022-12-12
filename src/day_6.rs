use std::fmt::Display;

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
    letters_consumed: usize,
    letter_counts: [i32; 26 + 1],
    overflow_count: i32,
}

impl<const L: usize> UniqueSequenceFinder<L>
where
    [u8; L]: Default,
{
    /// Branchless method for progressing the stream one letter and checking
    /// for a full frame of unique letters
    fn append_and_detect(&mut self, in_letter: usize) -> bool {
        let cursor = self.letters_consumed % L;
        let out_letter = self.buffer[cursor] as usize;

        // Decrement and check if removing the old letter removes a duplicate
        self.letter_counts[out_letter] -= 1;
        let duplicate_removed = self.letter_counts[out_letter] > 0;

        // Reduce the number of duplicates by one if a duplicate was removed
        self.overflow_count -= (duplicate_removed && out_letter > 0) as i32;

        // Replace the removed letter with the new one
        self.buffer[cursor] = in_letter as u8;

        // Increment and check to see if adding the new letter adds a duplicate
        self.letter_counts[in_letter] += 1;
        let duplicate_added = self.letter_counts[in_letter] > 1;

        // Apply the changes to the state from adding the new letter
        self.overflow_count += duplicate_added as i32;
        self.letters_consumed += 1;

        // A unique sequence is detected if no duplicates are detected after
        // filling the buffer
        self.letters_consumed >= L && self.overflow_count == 0
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
