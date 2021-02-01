use crate::{min, UnicodeSegmentation};
#[derive(Debug)]
pub enum LineSeparatorFormat {
    UNIX,
    DOS,
}
impl LineSeparatorFormat {
    pub fn separator(&self) -> &'static str {
        match self {
            LineSeparatorFormat::UNIX => "\n",
            LineSeparatorFormat::DOS => "\r\n",
        }
    }
}
impl std::fmt::Display for LineSeparatorFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            LineSeparatorFormat::UNIX => "UNIX",
            LineSeparatorFormat::DOS => "DOS",
        };
        write!(f, "{}", t)
    }
}
pub struct Buffer {
    lines: Vec<String>,

    cursor_x: usize,
    cursor_y: usize,
    reminder_cursor_x: usize,

    pub line_separator_format: LineSeparatorFormat,
}
impl Buffer {
    pub fn from(s: &str) -> Self {
        let line_separator_format = {
            let count = s.split('\n').count();
            if s.split('\n').take(count - 1).all(|l| l.ends_with('\r')) {
                LineSeparatorFormat::DOS
            } else {
                LineSeparatorFormat::UNIX
            }
        };
        let lines = s
            .split(line_separator_format.separator())
            .map(str::to_string)
            .collect::<Vec<_>>();
        let cursor_x = 0;
        let cursor_y = 0;
        let reminder_cursor_x = 0;
        Buffer {
            lines,
            cursor_x,
            cursor_y,
            reminder_cursor_x,
            line_separator_format,
        }
    }
    pub fn as_string(&self) -> String {
        self.lines.join("\n")
    }
    pub fn cursor(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }
    pub fn move_cursor_horizontal(&mut self, x: i64) {
        let length_of_line = self
            .lines
            .get(self.cursor_y)
            .map(|it| UnicodeSegmentation::graphemes(it.as_str(), true).count())
            .unwrap_or(0);

        let moving_beyond_first = x < 0 && self.cursor_x == 0;
        let moving_beyond_last = x > 0 && self.cursor_x + 1 >= length_of_line;

        if !moving_beyond_first && !moving_beyond_last {
            self.cursor_x = ((self.cursor_x as i64) + x) as usize;
            self.reminder_cursor_x = self.cursor_x;
        }
    }
    pub fn move_cursor_vertical(&mut self, y: i64) {
        let moving_beyond_first = y < 0 && self.cursor_y == 0;
        let moving_beyond_last = y > 0 && self.cursor_y + 1 >= self.lines.len();

        if !moving_beyond_first && !moving_beyond_last {
            self.cursor_y = ((self.cursor_y as i64) + y) as usize;
        }

        let length_of_line =
            UnicodeSegmentation::graphemes(self.lines[self.cursor_y].as_str(), true).count();
        if length_of_line == 0 {
            self.cursor_x = 0;
        } else {
            self.cursor_x = min(self.reminder_cursor_x, length_of_line - 1);
        }
    }
    pub fn get_under_cursor(&self) -> &str {
        debug_assert!(self.cursor_y < self.lines.len());
        UnicodeSegmentation::graphemes(self.lines[self.cursor_y].as_str(), true)
            .nth(self.cursor_x)
            .unwrap()
    }
    pub fn insert_under_cursor(&mut self, s: &str) {
        debug_assert!(self.cursor_y < self.lines.len());
        let line_graphemes =
            UnicodeSegmentation::graphemes(self.lines[self.cursor_y].as_str(), true)
                .collect::<Vec<_>>();
        if s == "\n" {
            let mut extend_lines: Vec<String> = vec![];
            let extra = line_graphemes[self.cursor_x..].join("");
            self.lines[self.cursor_y] = line_graphemes[..self.cursor_x].join("");
            extend_lines.extend(self.lines[..self.cursor_y + 1].iter().cloned());
            extend_lines.push(extra);
            extend_lines.extend(self.lines[self.cursor_y + 1..].iter().cloned());
            self.lines = extend_lines;
            self.move_cursor_vertical(1);
            self.cursor_x = 0;
            self.reminder_cursor_x = 0;
        } else {
            self.lines[self.cursor_y] = line_graphemes[..self.cursor_x].join("")
                + s
                + &line_graphemes[self.cursor_x..].join("");
            self.move_cursor_horizontal(1);
        }
    }
    pub fn delete_under_cursor(&mut self) {
        debug_assert!(self.cursor_y < self.lines.len());
        if self.cursor_x > 0 {
            let line_graphemes =
                UnicodeSegmentation::graphemes(self.lines[self.cursor_y].as_str(), true)
                    .collect::<Vec<_>>();
            let mut new_line = vec![];
            new_line.extend(line_graphemes[..self.cursor_x - 1].iter().cloned());
            new_line.extend(line_graphemes[self.cursor_x..].iter().cloned());
            self.lines[self.cursor_y] = new_line.join("");
            self.move_cursor_horizontal(-1);
        } else if self.cursor_y > 0 {
            let previous_line_len = UnicodeSegmentation::grapheme_indices(
                self.lines[self.cursor_y - 1].as_str(),
                true,
            )
            .count();
            self.lines[self.cursor_y] =
                self.lines[self.cursor_y - 1].clone() + &self.lines[self.cursor_y];
            self.lines.remove(self.cursor_y - 1);
            self.cursor_y -= 1;
            self.cursor_x = previous_line_len;
        }
    }
}

mod test {
    #[test]
    fn should_insert_at_start_of_line() {
        let mut buffer = crate::buffer::Buffer::from("");
        buffer.insert_under_cursor("a");
        buffer.insert_under_cursor("b");
        buffer.insert_under_cursor("c");
        assert_eq!("abc", buffer.to_string());
    }
}
