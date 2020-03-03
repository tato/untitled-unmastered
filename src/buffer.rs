enum PieceSource {
    ORIGINAL,
    APPEND,
}
struct Piece {
    start: usize,
    length: usize,
    source: PieceSource,
}
pub struct Buffer {
    original: Vec<char>,
    append: Vec<char>,
    pieces: Vec<Piece>,
}

use PieceSource::*;
impl Buffer {
    pub fn new() -> Self {
        Self {
            original: Vec::new(),
            append: Vec::new(),
            pieces: Vec::new(),
        }
    }
    pub fn from(source: &str) -> Self {
        let first_piece = Piece{ 
            start: 0, 
            length: source.len(),
            source: ORIGINAL
        };
        Self {
            original: source.chars().collect(),
            append: Vec::new(),
            pieces: vec![ first_piece ],
        }
    }
    pub fn insert(&mut self, text: &str, _pos: usize) { // TODO(ptato) pos
        // TODO(ptato)
        let start = self.append.len();
        for c in text.chars() {
            self.append.push(c);
        }
        self.pieces.push(Piece{ start, length: text.len(), source: APPEND });
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for piece in &self.pieces {
            let from = match &piece.source {
                ORIGINAL => &self.original,
                APPEND => &self.append,
            };
            result += from[piece.start..piece.start+piece.length].iter().collect::<String>().as_str();
        }
        result
    }
}
