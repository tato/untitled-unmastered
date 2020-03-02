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
            source: PieceSource::ORIGINAL
        };
        Self {
            original: source.chars().collect(),
            append: Vec::new(),
            pieces: vec![ first_piece ],
        }
    }
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for piece in &self.pieces {
            let from = match &piece.source {
                ORIGINAL => &self.original,
                APPEND => &self.append,
            };
            result += from[piece.start..piece.start+piece.length].into_iter().collect::<String>().as_str();
        }
        result
    }
}
