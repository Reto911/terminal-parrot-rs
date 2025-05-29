use std::io;

use console::Term;

use crate::{Orientation, animation::Animation, colors::COLORS};

pub struct Draw<'a> {
    term: Term,
    animation: &'a Animation,
    orientation: Orientation,
    frame_idx: usize,
    color_idx: usize,
}

impl<'a> Draw<'a> {
    pub fn new(term: Term, animation: &'a Animation, orientation: Orientation) -> Self {
        Self {
            term,
            animation,
            orientation,
            frame_idx: 0,
            color_idx: 0,
        }
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.term.clear_screen()?;
        let mut lines = self.animation.frames[self.frame_idx].lines();

        let inner = |line| {
            self.term
                .write_line(COLORS[self.color_idx].apply_to(line).to_string().as_str())
        };

        if self.orientation == Orientation::Aussie {
            lines.rev().try_for_each(inner)
        } else {
            lines.try_for_each(inner)
        }?;

        self.term.flush()?;

        self.frame_idx += 1;
        self.color_idx += 1;
        self.frame_idx %= self.animation.frames.len();
        self.color_idx %= COLORS.len();
        Ok(())
    }
}
