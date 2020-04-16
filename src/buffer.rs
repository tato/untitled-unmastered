extern crate linked_list;
use linked_list::LinkedList;
use crate::*;

#[derive(Copy, Clone, Debug)]
enum PieceSource {
    ORIGINAL,
    APPEND,
}
#[derive(Debug)]
struct Piece {
    start: usize,
    length: usize,
    source: PieceSource,
}
pub struct Buffer {
    original: Vec<u8>,
    append: Vec<u8>,
    pieces: LinkedList<Piece>,
}

use PieceSource::*;
impl Buffer {
    pub fn new() -> Self {
        Self {
            original: Vec::new(),
            append: Vec::new(),
            pieces: LinkedList::new(),
        }
    }
    pub fn from(source: &str) -> Self {
        let first_piece = Piece {
            start: 0,
            length: source.len(),
            source: ORIGINAL,
        };
        let mut pieces = LinkedList::new();
        pieces.push_back(first_piece);
        Self {
            original: source.to_string().into_bytes(),
            append: Vec::new(),
            pieces,
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
            let piece_source = piece.source;
            let from = match &piece.source {
                ORIGINAL => &self.original,
                APPEND => &self.append,
            };

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
                    source: piece_source,
                };
                cursor.insert(after);
            }

            if position != start_in_render {
                let before = Piece {
                    start: piece_start,
                    length: remove_offset,
                    source: piece_source,
                };

                cursor.insert(before);
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

        if self.pieces.is_empty() {
            self.pieces.push_back(Piece {
                start,
                length,
                source: APPEND,
            });
        }

        let mut search_position = 0;
        let mut cursor = self.pieces.cursor();
        while let Some(piece) = cursor.next() {
            let piece_position_start = search_position;
            search_position += piece.length;
            let piece_position_end = search_position;

            if search_position >= insert_position {
                let piece_start = piece.start;
                let piece_source = piece.source;

                cursor.prev();
                cursor.remove();

                if insert_position != piece_position_end - 1 {
                    let after = Piece {
                        start: piece_start + insert_position - piece_position_start,
                        length: piece_position_end - insert_position,
                        source: piece_source,
                    };
                    if after.length > 0 {
                        cursor.insert(after);
                    }
                }

                let new = Piece {
                    start,
                    length,
                    source: APPEND,
                };
                if new.length > 0 {
                    cursor.insert(new);
                }

                if insert_position != piece_position_start {
                    let before = Piece {
                        start: piece_start,
                        length: insert_position - piece_position_start,
                        source: piece_source,
                    };
                    if before.length > 0 {
                        cursor.insert(before);
                    }
                }
                break;
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for piece in &self.pieces {
            let from = match &piece.source {
                ORIGINAL => &self.original,
                APPEND => &self.append,
            };
            let sl = &from[piece.start..piece.start + piece.length];
            result += std::str::from_utf8(sl).unwrap_or_else(panic_with_dialog);
        }
        result
    }

    pub fn get(&mut self, idx: usize) -> Option<&str> {
        let mut current = 0;
        for piece in &self.pieces {
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
        }
        None
    }
}
