use std::rc::Rc;

use crate::chunks::Chunk;

pub struct ChunkNode {
    pub chunk: Chunk,

    pub prev: Option<Rc<ChunkNode>>,
}

impl ChunkNode {
    pub fn new(chunk: Chunk, prev: Option<ChunkNode>) -> ChunkNode {
        ChunkNode { 
            chunk, 
            prev: if let Some(node) = prev {
                Some(Rc::new(node))
            } else {
                None
            },
        }
    }
}
