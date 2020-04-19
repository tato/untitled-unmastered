use crate::*;

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

    next: usize,
    prev: usize,
}

struct PieceList {
    nodes_count: usize,
    nodes: Vec<Piece>,
    head: usize,
    tail: usize,
    free: usize,
}

const INVALID_NODE_INDEX: usize = std::usize::MAX;

pub struct Buffer {
    original: Vec<u8>,
    append: Vec<u8>,
    pieces: PieceList,

    reminder_cursor_x: usize,
    cursor_x: usize,
    cursor_y: usize,
}

impl Buffer {

    pub fn from(source: &str) -> Self {
        let first_piece = Piece {
            start: 0,
            length: source.len(),
            source: ORIGINAL,
            next: INVALID_NODE_INDEX,
            prev: INVALID_NODE_INDEX,
        };

        Self {
            original: source.to_string().into_bytes(),
            append: Vec::new(),
            pieces: PieceList::new(first_piece),

            reminder_cursor_x: 0,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn insert(&mut self, text: &str, insert_position: usize) {
        let start = self.append.len();
        let mut length = 0;
        for c in text.bytes() {
            self.append.push(c);
            length += 1;
        }

        debug_assert!(self.pieces.len() != 0);

        let mut search_position = 0;
        let mut cursor = self.pieces.cursor();
        while let Some(piece) = cursor.next() {
            let piece_position_start = search_position;
            search_position += piece.length;
            let piece_position_end = search_position;

            if search_position >= insert_position {
                let piece_start = piece.start;
                let piece_source = piece.source;

                cursor.remove_prev();

                if insert_position != piece_position_start {
                    let start = piece_start;
                    let length = insert_position - piece_position_start;
                    if length > 0 {
                        cursor.insert_before(start, length, piece_source);
                    }
                }

                if length > 0 {
                    cursor.insert_before(start, length, APPEND);
                }

                if insert_position != piece_position_end - 1 {
                    let start = piece_start + insert_position - piece_position_start;
                    let length = piece_position_end - insert_position;
                    if length > 0 {
                        cursor.insert_before(start, length, piece_source);
                    }
                }

                break;
            }
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

            cursor.remove_prev();

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

            if position != start_in_render {
                cursor.insert_before(piece_start, remove_offset, piece_source);
            }

            if position != end_in_render - len {
                let start = piece_start + remove_offset + len;
                let length = piece_length - remove_offset - len;
                cursor.insert_before(start, length, piece_source);
            }

            break;
        }
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


impl PieceList {
    fn new(first_piece: Piece) -> Self {
        let mut nodes = Vec::new();
        nodes.push(first_piece);

        Self {
            nodes_count: 1,
            nodes,
            head: 0,
            tail: 0,
            free: INVALID_NODE_INDEX,
        }
    }

    fn len(&self) -> usize {
        self.nodes_count
    }

    fn iter(&self) -> PieceIterator {
        PieceIterator {
            idx: self.head,
            list: self,
        }
    }

    fn cursor(&mut self) -> PieceCursor {
        PieceCursor {
            next_prev_idx: (self.head, INVALID_NODE_INDEX),
            list: self,
        }
    }
}

struct PieceIterator<'b> {
    list: &'b PieceList,
    /// index of next element
    idx: usize,
}
impl<'b> Iterator for PieceIterator<'b> {
    type Item = &'b Piece;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx != INVALID_NODE_INDEX {
            debug_assert!(self.idx < self.list.nodes.len());
            let result = &self.list.nodes[self.idx];
            self.idx = result.next;
            Some(result)
        } else {
            None
        }
    }
}


struct PieceCursor<'b> {
    list: &'b mut PieceList,
    next_prev_idx: (usize, usize),
}
impl<'b> PieceCursor<'b> {
    fn next(&mut self) -> Option<&mut Piece> {
        let (next_idx, _) = self.next_prev_idx;
        if next_idx != INVALID_NODE_INDEX {
            let result = &mut self.list.nodes[next_idx];
            self.next_prev_idx = (result.next, next_idx);
            Some(result)
        } else {
            None
        }
    }
    fn prev(&mut self) -> Option<&mut Piece> {
        let (_, prev_idx) = self.next_prev_idx;
        if prev_idx != INVALID_NODE_INDEX {
            let result = &mut self.list.nodes[prev_idx];
            self.next_prev_idx = (prev_idx, result.prev);
            Some(result)
        } else {
            None
        }
    }
    fn remove_prev(&mut self) {
        let (next_idx, prev_idx) = self.next_prev_idx;
        if prev_idx != INVALID_NODE_INDEX {
            let prev_of_prev_idx = self.list.nodes[prev_idx].prev;

            if prev_of_prev_idx != INVALID_NODE_INDEX {
                let prev_of_prev = &mut self.list.nodes[prev_of_prev_idx];
                prev_of_prev.next = next_idx;
            } else {
                self.list.head = next_idx;
            }

            if next_idx != INVALID_NODE_INDEX {
                let next = &mut self.list.nodes[next_idx];
                next.prev = prev_of_prev_idx;
            } else {
                self.list.tail = prev_of_prev_idx;
            }

            self.next_prev_idx = (next_idx, prev_of_prev_idx);

            self.list.nodes[prev_idx].next = self.list.free;
            self.list.free = prev_idx;
        }
    }
    fn insert_before(&mut self,
                     start: usize, length: usize, source: PieceSource) {
        let (next_idx, prev_idx) = self.next_prev_idx;
        let piece = Piece {
            start, length, source,
            next: next_idx, prev: prev_idx,
        };

        let piece_idx;
        if self.list.free != INVALID_NODE_INDEX {
            piece_idx = self.list.free;
            self.list.free = self.list.nodes[piece_idx].next;
            self.list.nodes[piece_idx] = piece;
        } else {
            piece_idx = self.list.nodes.len();
            self.list.nodes.push(piece);
        }

        if prev_idx != INVALID_NODE_INDEX {
            let prev = &mut self.list.nodes[prev_idx];
            prev.next = piece_idx;
        } else {
            self.list.head = piece_idx;
        }

        if next_idx != INVALID_NODE_INDEX {
            let next = &mut self.list.nodes[next_idx];
            next.prev = piece_idx;
        } else {
            self.list.tail = piece_idx;
        }

        self.next_prev_idx = (next_idx, piece_idx);
    }
}

