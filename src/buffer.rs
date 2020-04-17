extern crate linked_list;
use linked_list::LinkedList;
use crate::*;
use std::ptr;

#[derive(Copy, Clone, Debug)]
enum PieceSource {
    ORIGINAL,
    APPEND,
}
use PieceSource::*;

#[derive(Debug)]
struct Piece {
    start: usize,
    length: usize,
    source: PieceSource,

    next: Option<Box<Piece>>,
    prev: *const Piece,
}
pub struct Buffer {
    original: Vec<u8>,
    append: Vec<u8>,

    pieces_count: usize,
    head: Option<Box<Piece>>,
    tail: *const Piece,
}

impl Buffer {

    pub fn from(source: &str) -> Self {
        let first_piece = Piece {
            start: 0,
            length: source.len(),
            source: ORIGINAL,

            next: None,
            prev: ptr::null(),
        };

        let mut result = Self {
            original: source.to_string().into_bytes(),
            append: Vec::new(),

            pieces_count: 1,
            head: Some(first_piece.into()),
            tail: ptr::null(),
        };

        result.tail = result.head.as_ref().unwrap().as_ref() as *const Piece;

        result
    }

    pub fn remove(&mut self, position: usize) {
        let mut start_in_render = 0;
        let mut iter_pieces = self.head;
        while let Some(piece) = iter_pieces {
            let end_in_render = start_in_render + piece.length;
            let remove_offset = position - start_in_render;
            if position >= end_in_render {
                start_in_render = end_in_render;
                iter_pieces = piece.next;
                continue;
            }

            let piece_start = piece.start;
            let piece_length = piece.length;
            let piece_source = piece.source;
            let from = match &piece.source {
                ORIGINAL => &self.original,
                APPEND => &self.append,
            };

            todo!();
            // cursor.prev();
            // cursor.remove();

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
                todo!();
                // let after = Piece {
                //     start: piece_start + remove_offset + len,
                //     length: piece_length - remove_offset - len,
                //     source: piece_source,
                // };
                //cursor.insert(after);
            }

            if position != start_in_render {
                todo!();
                // let before = Piece {
                //     start: piece_start,
                //     length: remove_offset,
                //     source: piece_source,
                // };
                // cursor.insert(before);
            }

            break;
        }
    }

    pub fn insert(&mut self, text: &str, insert_position: usize) {
        let start = self.append.len();
        let mut length = 0;
        for c in text.bytes() {
            self.append.push(c);
            length += 1;
        }

        todo!();
        // if self.pieces.is_empty() {
        //     self.pieces.push_back(Piece {
        //         start,
        //         length,
        //         source: APPEND,
        //     });
        // }

        let mut search_position = 0;
        let mut iter_pieces = self.head;
        while let Some(piece) = iter_pieces {
            let piece_position_start = search_position;
            search_position += piece.length;
            let piece_position_end = search_position;

            if search_position >= insert_position {
                let piece_start = piece.start;
                let piece_source = piece.source;

                todo!();
                // cursor.prev();
                // cursor.remove();

                if insert_position != piece_position_end - 1 {
                    todo!();
                    // let after = Piece {
                    //     start: piece_start + insert_position - piece_position_start,
                    //     length: piece_position_end - insert_position,
                    //     source: piece_source,
                    // };
                    // if after.length > 0 {
                    //     cursor.insert(after);
                    // }
                }

                todo!();
                // let new = Piece {
                //     start,
                //     length,
                //     source: APPEND,
                // };
                // if new.length > 0 {
                //     cursor.insert(new);
                // }

                if insert_position != piece_position_start {
                    todo!();
                    // let before = Piece {
                    //     start: piece_start,
                    //     length: insert_position - piece_position_start,
                    //     source: piece_source,
                    // };
                    // if before.length > 0 {
                    //     cursor.insert(before);
                    // }
                }
                break;
            }

            iter_pieces = piece.next;
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut iter_pieces = self.head;
        while let Some(piece) = iter_pieces {
            let from = match &piece.source {
                ORIGINAL => &self.original,
                APPEND => &self.append,
            };
            let sl = &from[piece.start..piece.start + piece.length];
            result += std::str::from_utf8(sl).unwrap_or_else(panic_with_dialog);
            iter_pieces = piece.next;
        }
        result
    }

    pub fn get(&mut self, idx: usize) -> Option<&str> {
        let mut current = 0;
        let mut iter_pieces = self.head;
        while let Some(piece) = iter_pieces {
            let next = current + piece.length;
            if idx >= current && idx < next {
                let from = match &piece.source {
                    ORIGINAL => &self.original,
                    APPEND => &self.append,
                };
                let s = unsafe {
                    std::str::from_utf8_unchecked(&from[piece.start..piece.length])
                };
                return UnicodeSegmentation::graphemes(s, true)
                    .nth(idx - current + piece.start);
            }
            current = next;
            iter_pieces = piece.next;
        }
        None
    }
}
