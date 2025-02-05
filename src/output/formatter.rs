use std::fmt::{self, Write};

#[derive(Debug)]
pub struct Formatter<'a> {
    /// Write destination.
    pub out: &'a mut String,

    /// Number of spaces per indentation level.
    pub indent_size: usize,

    /// Current indentation level.
    pub indent_level: usize,
}

impl<'a> Formatter<'a> {
    pub fn new(out: &'a mut String, indent_size: usize) -> Self {
        Self {
            out,
            indent_size,
            indent_level: 0,
        }
    }

    pub fn block<F>(&mut self, heading: &str, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        write!(self, "{} {{\n", heading)?;

        self.indent(f)?;

        write!(self, "}}\n")?;

        Ok(())
    }

    pub fn indent<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.indent_level += 1;

        let ret = f(self);

        self.indent_level -= 1;

        ret
    }

    #[inline]
    #[rustfmt::skip]
    fn push_indentation(&mut self) {
        if self.indent_level > 0 {
            self.out.push_str(&" ".repeat(self.indent_level * self.indent_size));
        }
    }
}

impl<'a> Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut lines = s.lines().peekable();

        while let Some(line) = lines.next() {
            // Add indentation before the line if necessary.
            if self.out.ends_with('\n') && !line.is_empty() {
                self.push_indentation();
            }

            self.out.push_str(line);

            if lines.peek().is_some() || s.ends_with('\n') {
                self.out.push('\n');
            }
        }

        Ok(())
    }
}
