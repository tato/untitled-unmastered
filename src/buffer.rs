extern crate linked_list;
use linked_list::LinkedList;
use std::rc::Rc;
use crate::*;

#[derive(Debug)]
struct Piece {
    start: usize,
    length: usize,
    source: Rc<Vec<u8>>,
}
pub struct Buffer {
    original: Rc<Vec<u8>>,
    append: Rc<Vec<u8>>,
    pieces: LinkedList<Rc<Piece>>,

    cursor_piece: Rc<Piece>,

    reminder_cursor_x: usize,
    cursor_x: usize,
    cursor_y: usize,
}
impl Buffer {
    pub fn from(source: &str) -> Self {
        let original = source.to_string().into_bytes();
        let first_piece: Rc<Piece> = Piece {
            start: 0,
            length: source.len(),
            source: original.into(),
        }.into();
        let mut pieces = LinkedList::new();
        pieces.push_back(first_piece.clone());
        Self {
            pieces,
            original: Vec::new().into(),
            append: Vec::new().into(),
            cursor_piece: first_piece.clone(),
            reminder_cursor_x: 0,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn remove(&mut self, position: usize) {
        let mut start_in_render = 0;
        let mut cursor = self.pieces.cursor();
        while let Some(piece) = cursor.next() {
            let end_in_render = start_in_render + piece.length;
            let remove_offset = position - start_in_render;
            if position >= end_in_render {
                start_in_render = end_in_render;
                continue;
            }

            let piece_start = piece.start;
            let piece_length = piece.length;
            let piece_source = piece.source.clone();
            let from = &piece_source;

            cursor.prev();
            cursor.remove();

            let len = unsafe {
                let a = piece_start + remove_offset;
                let s = std::str::from_utf8_unchecked(&from[a..]);
                let m = "somehow attempted to remove a character that does not exist!";
                UnicodeSegmentation::
                    graphemes(s, true)
                    .next()
                    .map(|it| it.len())
                    .unwrap_or_else(|| panic_with_dialog(m))
            };

            if position != end_in_render - len {
                let after = Piece {
                    start: piece_start + remove_offset + len,
                    length: piece_length - remove_offset - len,
                    source: piece_source.clone(),
                };
                cursor.insert(after.into());
            }

            if position != start_in_render {
                let before = Piece {
                    start: piece_start,
                    length: remove_offset,
                    source: piece_source,
                };

                cursor.insert(before.into());
            }

            break;
        }
        self.recalculate_cursor_piece();
    }

    pub fn insert(&mut self, text: &str, insert_position: usize) {
        let start = self.append.len();
        let mut length = 0;
        for c in text.bytes() {
            Rc::get_mut(&mut self.append).unwrap().push(c);
            length += 1;
        }

        if self.pieces.is_empty() {
            self.pieces.push_back(Piece {
                start,
                length,
                source: self.append.clone(),
            }.into());
        }

        let mut search_position = 0;
        let mut cursor = self.pieces.cursor();
        while let Some(piece) = cursor.next() {
            let piece_position_start = search_position;
            search_position += piece.length;
            let piece_position_end = search_position;

            if search_position >= insert_position {
                let piece_start = piece.start;
                let piece_source = piece.source.clone();

                cursor.prev();
                cursor.remove();

                if insert_position != piece_position_end - 1 {
                    let after = Piece {
                        start: piece_start + insert_position - piece_position_start,
                        length: piece_position_end - insert_position,
                        source: piece_source.clone(),
                    };
                    if after.length > 0 {
                        cursor.insert(after.into());
                    }
                }

                let new = Piece {
                    start,
                    length,
                    source: self.append.clone(),
                };
                if new.length > 0 {
                    cursor.insert(new.into());
                }

                if insert_position != piece_position_start {
                    let before = Piece {
                        start: piece_start,
                        length: insert_position - piece_position_start,
                        source: piece_source.clone(),
                    };
                    if before.length > 0 {
                        cursor.insert(before.into());
                    }
                }
                break;
            }
        }
        self.recalculate_cursor_piece();
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for piece in &self.pieces {
            let from = &piece.source;
            let sl = &from[piece.start..piece.start + piece.length];
            result += std::str::from_utf8(sl).unwrap_or_else(panic_with_dialog);
        }
        result
    }

    fn recalculate_cursor_piece(&mut self) {
        unimplemented!();
    }

    pub fn cursor_offset_in_buffer(&self) -> usize {
        todo!();
    //     let mut result = 0;
    //     let mut x = 0;
    //     let mut y = 0;
    //     for piece in &self.table.pieces {
    //         let from = match &piece.source {
    //             ORIGINAL => &self.original,
    //             APPEND => &self.append,
    //         };
    //         for c in UnicodeSegmentation::graphemes(
    //         result += piece.length;
    //     }
    //     return result;
    }

    pub fn get_character_under_cursor(&self) -> &str {
        todo!();
    }

    // pub fn get(&mut self, idx: usize) -> Option<&str> {
    //     let mut current = 0;
    //     for piece in &self.pieces {
    //         let next = current + piece.length;
    //         if idx >= current && idx < next {
    //             let from = match &piece.source {
    //                 ORIGINAL => &self.original,
    //                 APPEND => &self.append,
    //             };
    //             let s = unsafe {
    //                 std::str::from_utf8_unchecked(&from[piece.start..piece.length])
    //             };
    //             return UnicodeSegmentation::graphemes(s, true)
    //                 .nth(idx - current + piece.start);
    //         }
    //         current = next;
    //     }
    //     None
    // }

    pub fn move_cursor_horizontal(&mut self, x: i64) {
        todo!();
        // let length_of_line = self
        //     .buffer
        //     .to_string()
        //     .split('\n')
        //     .nth(self.cursor_y)
        //     .map(|it| UnicodeSegmentation::graphemes(it, true).count())
        //     .unwrap_or(0);

        // let moving_beyond_first = x < 0 && self.global_cursor_x == 0;
        // let moving_beyond_last = x > 0 && self.global_cursor_x >= length_of_line;

        // if !moving_beyond_first && !moving_beyond_last {
        //     self.global_cursor_x = ((self.global_cursor_x as i64) + x) as usize;
        // }

        // self.actual_cursor_x = min(self.global_cursor_x, length_of_line);
    }

    pub fn move_cursor_vertical(&mut self, y: i64, render: &RenderContext) {
        todo!();

        // let buffer_string = self.buffer.to_string();
        // let buffer_lines: Vec<_> = buffer_string
        //     .split('\n')
        //     .map(|it| UnicodeSegmentation::graphemes(it, true).collect::<Vec<_>>())
        //     .collect();

        // let moving_beyond_first = y < 0 && self.cursor_y == 0;
        // let moving_beyond_last = y > 0 && self.cursor_y >= buffer_lines.len()-1;

        // if !moving_beyond_first && !moving_beyond_last {
        //     self.cursor_y = ((self.cursor_y as i64) + y) as usize;
        // }

        // let length_of_line = buffer_lines[self.cursor_y].len();
        // self.actual_cursor_x = min(self.global_cursor_x, length_of_line);

        // let ch = render.character_height;
        // let window_height_in_characters = (render.height() / ch) as usize;
        // if y > 0 &&
        //     self.cursor_y > self.y_render_offset + window_height_in_characters - 7
        // {
        //     self.y_render_offset += y as usize;
        // }
        // if y < 0 && self.cursor_y < self.y_render_offset + 5
        //     && (self.y_render_offset as i64) + y >= 0
        // {
        //     self.y_render_offset -= (-y) as usize;
        // }
        // self.y_render_offset = min(self.y_render_offset, buffer_lines.len());

        // self.cursor_animation_instant = Instant::now();
    }

}

