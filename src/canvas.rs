use iter_tools::Itertools;

/// Frivolous structure for drawing and debugging
pub struct Canvas {
    pub empty_char: char,
    pub contents: Vec<char>,
    pub width: i32,
    pub height: i32,
    pub top_left: (i32, i32),
    pub draw_enabled: bool,
    pub render_enabled: bool,
}

impl Canvas {
    pub fn new(empty_char: char, center: (i32, i32)) -> Self {
        Self {
            empty_char,
            contents: vec![empty_char],
            width: 1,
            height: 1,
            top_left: center,
            draw_enabled: true,
            render_enabled: false,
        }
    }

    /// Calculate the index in the image buffer that corresponds to the given
    /// point
    pub fn index_for_coordinate(&self, coord: (i32, i32)) -> usize {
        let row_start = ((coord.1 - self.top_left.1) * self.width) as usize;
        let distance_into_row = (coord.0 - self.top_left.0) as usize;

        row_start + distance_into_row
    }

    /// Get the coordinates in viewed space of the point at the given index in
    /// the image buffer
    pub fn coordinate_from_index(&self, i: usize) -> (i32, i32) {
        (
            i as i32 % self.width + self.top_left.0,
            i as i32 / self.width + self.top_left.1,
        )
    }

    /// Get the coordinates of the given buffer index in canvas space
    pub fn canvas_coord_from_index(&self, i: usize) -> (i32, i32) {
        (i as i32 % self.width, i as i32 / self.width)
    }

    /// Resize the canvas if needed to allow the new point to be displayed
    fn resize_if_needed(&mut self, new_points: &[&(i32, i32)]) {
        let x_bound = (self.top_left.0)..(self.top_left.0 + self.width);
        let y_bound = (self.top_left.1)..(self.top_left.1 + self.height);

        // No resize needed
        if new_points.iter().fold(true, |in_bounds, (x, y)| {
            in_bounds && x_bound.contains(x) && y_bound.contains(y)
        }) {
            return;
        }

        // Determine the new viewport
        let new_top_left = new_points.iter().fold(
            self.top_left,
            |(old_min_x, old_min_y), (x, y)| {
                (old_min_x.min(*x), old_min_y.min(*y))
            },
        );

        let top_left_shift = (
            new_top_left.0 - self.top_left.0,
            new_top_left.1 - self.top_left.1,
        );

        let new_width = new_points
            .iter()
            .fold(self.width - top_left_shift.0, |new_width, (x, _)| {
                new_width.max(x - new_top_left.0 + 1)
            });

        let new_height = new_points
            .iter()
            .fold(self.height - top_left_shift.1, |new_height, (_, y)| {
                new_height.max(y - new_top_left.1 + 1)
            });

        let mut new_canvas = Canvas {
            empty_char: self.empty_char,
            top_left: new_top_left,
            width: new_width,
            height: new_height,
            contents: vec![self.empty_char; (new_width * new_height) as usize],
            render_enabled: self.render_enabled,
            draw_enabled: self.draw_enabled,
        };

        // Add the old contents to the new contents
        self.contents.iter().enumerate().for_each(|(i, c)| {
            let coord = self.coordinate_from_index(i);
            let new_i = new_canvas.index_for_coordinate(coord);
            new_canvas.contents[new_i] = *c;
        });

        *self = new_canvas;
    }

    pub fn draw_point(&mut self, point: (i32, i32), v: char) {
        if !self.draw_enabled {
            return;
        }

        self.resize_if_needed(&[&point]);

        let index = self.index_for_coordinate(point);
        self.contents[index] = v;
    }

    pub fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), p: char) {
        if !self.draw_enabled {
            return;
        }

        self.resize_if_needed(&[&start, &end]);

        let (delta, times) = match (start.0 == end.0, start.1 == end.1) {
            (true, _) if start.1 < end.1 => (self.width, end.1 - start.1),
            (true, _) => (-self.width, start.1 - end.1),
            (_, true) if start.0 < end.0 => (1, end.0 - start.0),
            (_, true) => (-1, start.0 - end.0),
            _ => unimplemented!("Horizontal or vertical lines only for now"),
        };

        let index = self.index_for_coordinate(start) as i32;

        (0..=times).map(i32::from).for_each(|i| {
            self.contents[(index + delta * i) as usize] = p;
        });
    }

    pub fn render(&self) {
        if self.render_enabled {
            println!();
            self.contents
                .chunks(self.width as usize)
                .map(|chunk| chunk.iter().join(""))
                .for_each(|line| println!("{line}"));
        }
    }
}
