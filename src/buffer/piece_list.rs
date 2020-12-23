use crate::buffer::Piece;
use std::iter::Peekable;

#[derive(Debug, Clone, Copy)]
struct NodeIndex(Option<std::num::NonZeroUsize>);
impl NodeIndex {
    fn with(val: usize) -> Self {
        Self(std::num::NonZeroUsize::new(val))
    }
    fn empty() -> Self {
        Self(None)
    }
    fn get(&self) -> Option<usize> {
        match self.0 {
            None => None,
            Some(val) => Some(val.get()-1)
        }
    }
}

struct Node {
    val: Piece,
    next: NodeIndex,
    prev: NodeIndex,
}
pub struct PieceList {
    memory: Vec<Node>,
    head: NodeIndex,
    tail: NodeIndex,
    free: NodeIndex,
}
impl PieceList {
    // PieceList is never empty and needs at least one value
    pub fn new(val: Piece) -> Self {
        let node = Node { val, next: NodeIndex::empty(), prev: NodeIndex::empty() };
        Self {
            memory: vec![ node ],
            head: NodeIndex::with(0),
            tail: NodeIndex::with(0),
            free: NodeIndex::empty(),
        }
    }
    pub fn next(&mut self) {
        todo!();
    }
    pub fn prev(&mut self) {
        todo!();
    }
    pub fn insert_after(&mut self, val: Piece) {
        todo!()
    }
    pub fn delete(&mut self) -> Piece {
        todo!();
    }
    pub fn iter(&self) -> PieceIterator {
        PieceIterator {
            idx: self.head,
            list: self,
        }
    }
}
pub struct PieceIterator<'list> {
    list: &'list PieceList,
    /// index of next element
    idx: NodeIndex,
}
impl<'list> Iterator for PieceIterator<'list> {
    type Item = &'list Piece;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx.get().map(|i| {
            let result: &Node = &self.list.memory[i];
            self.idx = result.next;
            &result.val
        })
    }
}

/*
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



    fn get_by_id(&self, pivot_id: usize) -> Option<&Piece> {
        self.nodes.get(pivot_id)
    }

    fn insert_before(&mut self, pivot_id: usize, start: usize, length: usize, source: PieceSource) -> usize {
        debug_assert!(pivot_id != INVALID_NODE_INDEX);

        let pivot = &self.nodes.get(pivot_id).unwrap();
        let prev_idx = pivot.prev;
        let next_idx = pivot.next;

        let piece = Piece {
            start, length, source,
            next: next_idx, prev: prev_idx,
        };

        let piece_idx;
        if self.free != INVALID_NODE_INDEX {
            piece_idx = self.free;
            self.free = self.nodes[piece_idx].next;
            self.nodes[piece_idx] = piece;
        } else {
            piece_idx = self.nodes.len();
            self.nodes.push(piece);
        }

        if prev_idx != INVALID_NODE_INDEX {
            let prev = &mut self.nodes[prev_idx];
            prev.next = piece_idx;
        } else {
            self.head = piece_idx;
        }

        if next_idx != INVALID_NODE_INDEX {
            let next = &mut self.nodes[next_idx];
            next.prev = piece_idx;
        } else {
            self.tail = piece_idx;
        }

        piece_idx
    }

    fn remove(&mut self, elem_id: usize) -> Piece {
        debug_assert!(elem_id != INVALID_NODE_INDEX);

        let prev_idx = self.nodes[elem_id].prev;
        let next_idx = self.nodes[elem_id].next;

        if prev_idx != INVALID_NODE_INDEX {
            let prev = &mut self.nodes[prev_idx];
            prev.next = next_idx;
        } else {
            self.head = next_idx;
        }

        if next_idx != INVALID_NODE_INDEX {
            let next = &mut self.nodes[next_idx];
            next.prev = prev_idx;
        } else {
            self.tail = prev_idx;
        }

        let result = self.nodes[elem_id].clone();

        self.nodes[elem_id].next = self.free;
        self.free = elem_id;

        result
    }

    fn cursor(&mut self) -> PieceCursor {
        PieceCursor {
            next_prev_idx: (self.head, INVALID_NODE_INDEX),
            list: self,
        }
    }
}

struct CharacterIterator<'buffer> {
    buffer: &'buffer Buffer,
    iter: Peekable<PieceIterator<'buffer>>,
    idx_in_piece: usize,
}
impl<'buffer> Iterator for CharacterIterator<'buffer> {
    type Item = &'buffer str;
    fn next(&mut self) -> Option<Self::Item> {
        let &piece = self.iter.peek()?;
        if self.idx_in_piece < piece.length {
            let from = match piece.source {
                ORIGINAL => &self.buffer.original,
                APPEND => &self.buffer.append,
            };
            // note: these next two calculations can never return None as
            // long as the string is valid utf-8.
            // improvement?: might have to be considered hard errors, editor
            // can't operate if the internal state is that fucked.
            let s = std::str::from_utf8(&from[self.idx_in_piece..]).ok()?;
            let r = UnicodeSegmentation::graphemes(s, true).next()?;
            self.idx_in_piece += r.len();
            Some(r)
        } else {
            self.idx_in_piece = 0;
            self.iter.next();
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
        let (_, prev_idx) = self.next_prev_idx;
        if prev_idx != INVALID_NODE_INDEX {
            let removed = self.list.remove(prev_idx);
            self.next_prev_idx = (removed.next, removed.prev);
        }
    }
    fn insert_before(&mut self,
                     start: usize, length: usize, source: PieceSource) {
        let (next_idx, _) = self.next_prev_idx;
        let piece_idx = self.list.insert_before(next_idx, start, length, source);
        self.next_prev_idx = (next_idx, piece_idx);
    }
}
*/