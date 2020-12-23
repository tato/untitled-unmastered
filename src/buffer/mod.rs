use crate::{panic_with_dialog,min,UnicodeSegmentation};
mod piece_list;
use piece_list::PieceList;

#[derive(Copy, Clone, Debug)]
enum PieceSource {
    ORIGINAL,
    APPEND,
}
use PieceSource::*;

#[derive(Clone, Debug)]
pub struct Piece {
    start: usize,
    length: usize,
    source: PieceSource,
}

pub struct Buffer {
    original: Vec<u8>,
    append: Vec<u8>,
    pieces: PieceList,

    reminder_cursor_x: usize,
    cursor_x: usize,
    cursor_y: usize,

    piece_under_cursor: usize,
}

impl Buffer {

    pub fn from(source: &str) -> Self {
        let first_piece = Piece {
            start: 0,
            length: source.len(),
            source: ORIGINAL,
        };

        Self {
            original: source.to_string().into_bytes(),
            append: Vec::new(),
            pieces: PieceList::new(first_piece),

            reminder_cursor_x: 0,
            cursor_x: 0,
            cursor_y: 0,

            piece_under_cursor: 0,
        }
    }

    pub fn insert_under_cursor(&mut self, text: &str) {
        todo!();
        // let start = self.append.len();
        // let bytes = text.as_bytes();
        // let length = bytes.len();
        // self.append.extend_from_slice(bytes);

        // debug_assert!(self.pieces.len() != 0);
        // debug_assert!(length != 0);

        // let piece = match self.pieces.get_by_id(self.piece_under_cursor) {
        //     None => panic_with_dialog("unexpected error 😳"),
        //     Some(piece) => piece,
        // };

        // let piece_start = piece.start;
        // let piece_source = piece.source;

        // // todo!("insert_position? piece_position_start?");
        // let insert_position = self.cursor_position_in_buffer();
        // let piece_position_start = self.pieces
        //     .iter()
        //     .take_while(|&p| p as *const _ != piece as *const _)
        //     .map(|p| p.length)
        //     .sum();
        // let piece_position_end = piece_position_start + piece.length;

        // if insert_position != piece_position_start {
        //     let start = piece_start;
        //     let length = insert_position - piece_position_start;
        //     if length > 0 {
        //         self.pieces.insert_before(self.piece_under_cursor,
        //                                   start, length, piece_source);
        //     }
        // }

        // let new_piece_id = self.pieces.insert_before(self.piece_under_cursor,
        //                                              start, length, APPEND);

        // if insert_position != piece_position_end - 1 {
        //     let start = piece_start + insert_position - piece_position_start;
        //     let length = piece_position_end - insert_position;
        //     if length > 0 {
        //         self.pieces.insert_before(self.piece_under_cursor,
        //                                   start, length, piece_source);
        //     }
        // }

        // self.pieces.remove(self.piece_under_cursor);
        // self.piece_under_cursor = new_piece_id;

        // self.move_cursor_horizontal(text.len() as i64);
    }

    pub fn remove(&mut self, position: usize) {
        todo!("self.piece_under_cursor");
        // let mut start_in_render = 0;
        // let mut cursor = self.pieces.cursor();
        // while let Some(piece) = cursor.next() {
        //     let end_in_render = start_in_render + piece.length;
        //     let remove_offset = position - start_in_render;
        //     if position >= end_in_render {
        //         start_in_render = end_in_render;
        //         continue;
        //     }

        //     let piece_start = piece.start;
        //     let piece_length = piece.length;
        //     let piece_source = piece.source;
        //     let from = match &piece.source {
        //         ORIGINAL => &self.original,
        //         APPEND => &self.append,
        //     };

        //     cursor.remove_prev();

        //     let len = unsafe {
        //         let a = piece_start + remove_offset;
        //         let s = std::str::from_utf8_unchecked(&from[a..]);
        //         let m = "somehow attempted to remove a character that does not exist!";
        //         UnicodeSegmentation::
        //             graphemes(s, true)
        //             .next()
        //             .map(|it| it.len())
        //             .unwrap_or_else(|| panic_with_dialog(m))
        //     };

        //     if position != start_in_render {
        //         cursor.insert_before(piece_start, remove_offset, piece_source);
        //     }

        //     if position != end_in_render - len {
        //         let start = piece_start + remove_offset + len;
        //         let length = piece_length - remove_offset - len;
        //         cursor.insert_before(start, length, piece_source);
        //     }

        //     break;
        // }
    }

    // todo! this function should never be necessary
    // but it's easier to implement than the alternatives for now
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for piece in self.pieces.iter() {
            let from = match &piece.source {
                ORIGINAL => &self.original,
                APPEND => &self.append,
            };
            let sl = &from[piece.start..piece.start + piece.length];
            result += std::str::from_utf8(sl).unwrap_or_else(panic_with_dialog);
        }
        result
    }

    pub fn move_cursor_horizontal(&mut self, x: i64) {
        // todo!: don't use to_string
        let length_of_line = self
            .to_string()
            .split('\n')
            .nth(self.cursor_y)
            .map(|it| UnicodeSegmentation::graphemes(it, true).count())
            .unwrap_or(0);

        let moving_beyond_first = x < 0 && self.cursor_x == 0;
        let moving_beyond_last = x > 0 && self.cursor_x+1 >= length_of_line;

        if !moving_beyond_first && !moving_beyond_last {
            self.cursor_x = ((self.cursor_x as i64) + x) as usize;
            self.reminder_cursor_x = self.cursor_x;
        }

        todo!("piece_under_cursor");
    }

    pub fn move_cursor_vertical(&mut self, y: i64) {
        // todo!: don't use to_string
        let buffer_string = self.to_string();
        let buffer_lines: Vec<_> = buffer_string
            .split('\n')
            .collect();

        let moving_beyond_first = y < 0 && self.cursor_y == 0;
        let moving_beyond_last = y > 0 && self.cursor_y+1 >= buffer_lines.len();

        if !moving_beyond_first && !moving_beyond_last {
            self.cursor_y = ((self.cursor_y as i64) + y) as usize;
        }

        let mut length_of_line = UnicodeSegmentation::graphemes(buffer_lines[self.cursor_y], true).count();
        if length_of_line == 0 {
            self.cursor_x = 0;
        } else {
            self.cursor_x = min(self.reminder_cursor_x, length_of_line - 1);
        }

        todo!("piece_under_cursor");
    }

    pub fn cursor(&self) -> (usize,usize) {
        (self.cursor_x,self.cursor_y)
    }

    // todo! this should not be necessary
    // we should be able to .insert_after_cursor() or whatever
    pub fn cursor_position_in_buffer(&self) -> usize {
        let buffer_string = self.to_string();
        let buffer_lines: Vec<&str> = buffer_string
            .split('\n')
            .take(self.cursor_y + 1)
            .collect();
        let length_before_line = buffer_lines[0..self.cursor_y]
            .iter()
            .map(|t| t.len() + 1)
            .sum::<usize>();
        let length_inside_line = UnicodeSegmentation::
            graphemes(buffer_lines[self.cursor_y], true)
            .take(self.cursor_x)
            .map(|gc| gc.len())
            .sum::<usize>();
        length_before_line + length_inside_line
    }

    pub fn get(&mut self, idx: usize) -> Option<&str> {
        todo!();
    //     let mut current = 0;
    //     let mut iter_pieces = self.head;
    //     while iter_pieces.is_null() {
    //         let piece = unsafe { &*iter_pieces };
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
    //         iter_pieces = piece.next;
    //     }
    //     None
    }
}




