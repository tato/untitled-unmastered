use std::rc::Rc;
#[derive(Copy, Clone)]
enum PieceSource {
    ORIGINAL,
    APPEND,
}
struct Piece {
    start: usize,
    length: usize,
    source: PieceSource,
    next: Option<Rc<Piece>>,
}
pub struct Buffer {
    original: Vec<char>,
    append: Vec<char>,
    first: Option<Rc<Piece>>,
}

use PieceSource::*;
impl Buffer {
    pub fn new() -> Self {
        Self {
            original: Vec::new(),
            append: Vec::new(),
            first: None,
        }
    }
    pub fn from(source: &str) -> Self {
        let first_piece = Piece{ 
            start: 0, 
            length: source.len(),
            source: ORIGINAL,
            next: None,
        };
        Self {
            original: source.chars().collect(),
            append: Vec::new(),
            first: Some(Rc::new(first_piece)),
        }
    }
    pub fn insert(&mut self, text: &str, insert_position: usize) {
        let start = self.append.len();
        for c in text.chars() {
            self.append.push(c);
        }

        let mut search_position = 0;
        let mut iter_piece = &self.first;
        while let Some(piece) = iter_piece {

            let piece_position_start = search_position;
            search_position += piece.length;
            let piece_position_end = search_position;

            if search_position >= insert_position {
                if insert_position == piece_position_start {
                } else {
                    let after = Piece{
                        start: piece.start + piece.length,
                        length: insert_position - piece_position_start,
                        source: piece.source,
                        next: piece.next.clone(),
                    };
                    let new = Piece{ 
                        start: start,
                        length: text.len(),
                        source: APPEND,
                        next: Some(Rc::new(after)),
                    };
                    let before = Piece {
                        start: piece.start,
                        length: piece_position_end - insert_position,
                        source: piece.source,
                        next: Some(Rc::new(new)),
                    };

                    if let Some(piece) = &self.first {
                        self.first = Some(Rc::new(before));
                    }
                }
                break;
            }
            iter_piece = &piece.next;
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut iter_piece = &self.first;
        while let Some(piece) = iter_piece {
            let from = match &piece.source {
                ORIGINAL => &self.original,
                APPEND => &self.append,
            };
            result += from[piece.start..piece.start+piece.length].iter().collect::<String>().as_str();

            iter_piece = &piece.next;
        }
        result
    }
}
