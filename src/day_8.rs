use std::{fmt::Display, ops::Range};

use iter_tools::Itertools;

/// Structure that can be used to iterate over a slice of a forest
struct ForestScan<'a> {
    forest: &'a Forest,
    bounds: Range<usize>,
    curr: usize,
    step: isize,
}

impl<'a> Iterator for ForestScan<'a> {
    type Item = ((usize, usize), u8);

    /// Get the next element in the scan if there is one
    fn next(&mut self) -> Option<Self::Item> {
        self.bounds.contains(&self.curr).then(|| {
            // create the coordinates based on the current index and sizes
            let coords =
                (self.curr / self.forest.width, self.curr % self.forest.width);

            // Current tree height
            let result = self.forest.contents[self.curr];

            // step to the next tree
            if self.step > 0 {
                self.curr += self.step as usize;
            } else {
                self.curr = self.curr.overflowing_sub((-self.step) as usize).0;
            }

            (coords, result)
        })
    }
}

/// Representation of the heights of trees in a grid forest (basically an image)
struct Forest {
    width: usize,
    height: usize,
    contents: Vec<u8>,
}

impl Forest {
    /// Build a new forest based on the input
    fn new<I>(mut input_lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        // Load the first line of trees to get the width
        let first_line = input_lines.next().expect("Should be at least one");
        let first_row = first_line.into_bytes();
        let width = first_row.len();

        let contents = first_row
            .into_iter()
            .chain(input_lines.flat_map(|line| line.into_bytes().into_iter()))
            .collect::<Vec<_>>();

        // This assumes all rows are the same length
        let height = contents.len() / width;

        Forest {
            width,
            height,
            contents,
        }
    }

    /// Get an iterator over all horizontal slices of trees in the forest from
    /// left to right
    fn easts(&self) -> impl Iterator<Item = ForestScan<'_>> {
        (0..self.height).map(|row| ForestScan {
            forest: self,
            bounds: (row * self.width)..((row + 1) * self.width),
            step: 1,
            curr: row * self.width,
        })
    }

    /// Get an iterator over all horizontal slices of trees in the forest from
    /// right to left
    fn wests(&self) -> impl Iterator<Item = ForestScan<'_>> {
        (0..self.height).map(|row| ForestScan {
            forest: self,
            bounds: (row * self.width)..((row + 1) * self.width),
            step: -1,
            curr: (row + 1) * self.width - 1,
        })
    }

    /// Get an iterator over all vertical slices of trees in the forest from
    /// top to bottom
    fn souths(&self) -> impl Iterator<Item = ForestScan<'_>> {
        (0..self.width).map(move |col| ForestScan {
            forest: self,
            bounds: 0..(self.contents.len()),
            step: self.width as isize,
            curr: col,
        })
    }

    /// Get an iterator over all vertical slices of trees in the forest from
    /// top to bottom
    fn norths(&self) -> impl Iterator<Item = ForestScan<'_>> {
        (0..self.width).map(|col| ForestScan {
            forest: self,
            bounds: 0..(self.contents.len()),
            step: -(self.width as isize),
            curr: (self.height - 1) * self.width + col,
        })
    }

    /// Scan all directions in the forest (east, west, north, south) which
    /// translates to getting an iterator on every row and column backwards and
    /// forwards
    fn all_directions(&self) -> impl Iterator<Item = ForestScan<'_>> {
        self.easts()
            .chain(self.wests())
            .chain(self.souths())
            .chain(self.norths())
    }

    fn rays_from_trees(
        &self,
    ) -> impl Iterator<Item = (u8, impl Iterator<Item = ForestScan<'_>>)> {
        self.contents.iter().enumerate().map(|(tree, height)| {
            let row = tree / self.width;

            let east = ForestScan {
                forest: self,
                bounds: (row * self.width)..((row + 1) * self.width),
                step: 1,
                curr: tree,
            };
            let west = ForestScan {
                forest: self,
                bounds: (row * self.width)..((row + 1) * self.width),
                step: -1,
                curr: tree,
            };
            let south = ForestScan {
                forest: self,
                bounds: 0..(self.height * self.width),
                step: self.width as isize,
                curr: tree,
            };
            let north = ForestScan {
                forest: self,
                bounds: 0..(self.height * self.width),
                step: -(self.width as isize),
                curr: tree,
            };
            (*height, vec![east, west, south, north].into_iter())
        })
    }
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    Forest::new(input_lines)
        .all_directions()
        .flat_map(|scan| {
            let mut max = 0_u8;
            scan.filter(move |(_, size)| {
                (*size > max).then(|| max = *size).is_some()
            })
        })
        .unique()
        .count()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    Forest::new(input_lines)
        .rays_from_trees()
        .map(|(center_height, rays)| {
            rays.map(|scan| {
                let mut total = 0;

                scan.skip(1) // skip the starting tree
                    .take_while(|(_, height)| {
                        total += 1; // <- Ugly kluge because of ugly scoring
                        *height < center_height
                    })
                    .for_each(|_| {}); // Just to run the iterator

                total
            })
            .product::<usize>()
        })
        .max()
        .expect("Please tell me there's one tree")
}
