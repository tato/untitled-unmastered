#[derive(Copy, Clone)]
enum PieceSource {
    ORIGINAL,
    APPEND,
}

type PiecePointer = Option<usize>;

struct Piece {
    start: usize,
    length: usize,
    source: PieceSource,

    next: PiecePointer,
    prev: PiecePointer,
}
pub struct Buffer {
    original: Vec<char>,
    append: Vec<char>,
    pieces: Vec<Piece>,

    first: PiecePointer,
}

use PieceSource::*;
impl Buffer {
    pub fn new() -> Self {
        Self {
            original: Vec::new(),
            append: Vec::new(),
            pieces: Vec::new(),
            first: None,
        }
    }
    pub fn from(source: &str) -> Self {
        let first_piece = Piece{ 
            start: 0, 
            length: source.len(),
            source: ORIGINAL,
            next: None,
            prev: None,
        };
        Self {
            original: source.chars().collect(),
            append: Vec::new(),
            pieces: vec![ first_piece ],
            first: Some(0),
        }
    }
    pub fn insert(&mut self, text: &str, pos: usize) {
        let start = self.append.len();
        for c in text.chars() {
            self.append.push(c);
        }

        let mut search_pos = 0;
        let mut iter_piece = &self.first;
        loop {
            if let Some(piece_pointer) = *iter_piece {
                let piece = &self.pieces[piece_pointer];

                let piece_pos_start = search_pos;
                search_pos += piece.length;
                let piece_pos_end = search_pos;

                if search_pos >= pos {
                    if pos == piece_pos_start {
                        let first_pointer = Some(self.pieces.len());
                        let new_pointer = Some(self.pieces.len() + 1);

                        let piece_prev = piece.prev;

                        let first = Piece {
                            start: piece.start,
                            length: piece_pos_end - pos,
                            source: piece.source,
                            next: piece.next,
                            prev: new_pointer,
                        };
                        self.pieces.push(first);

                        let new = Piece{ 
                            start: start,
                            length: text.len(),
                            source: APPEND,
                            next: first_pointer,
                            prev: piece_prev,
                        };
                        self.pieces.push(new);

                        if piece_prev == None {
                            self.first = new_pointer;
                        }
                    } else {
                        let first_pointer = Some(self.pieces.len());
                        let second_pointer = Some(self.pieces.len() + 1);
                        let new_pointer = Some(self.pieces.len() + 2);

                        let first = Piece {
                            start: piece.start,
                            length: piece_pos_end - pos,
                            source: piece.source,
                            next: new_pointer,
                            prev: piece.prev,
                        };
                        let second = Piece{
                            start: piece.start + piece.length,
                            length: piece.length - first.length,
                            source: piece.source,
                            next: piece.next,
                            prev: new_pointer,
                        };
                        self.pieces.push(first);
                        self.pieces.push(second);

                        let new = Piece{ 
                            start: start,
                            length: text.len(),
                            source: APPEND,
                            next: second_pointer,
                            prev: first_pointer,
                        };
                        self.pieces.push(new);

                        if let Some(p) = self.first {
                            if p == piece_pointer {
                                self.first = first_pointer;
                            }
                        }
                    }
                    break;
                }
                iter_piece = &piece.next;
            } else {
                break;
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut iter_pointer = &self.first;
        loop {
            match iter_pointer {
                None => break,
                Some(pointer) => {
                    let piece = &self.pieces[*pointer];
                    let from = match &piece.source {
                        ORIGINAL => &self.original,
                        APPEND => &self.append,
                    };
                    result += from[piece.start..piece.start+piece.length].iter().collect::<String>().as_str();

                    iter_pointer = &piece.next;
                },
            }
        }
        result
    }
}
