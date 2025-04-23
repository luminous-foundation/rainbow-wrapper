use std::rc::Rc;

use crate::chunks::Chunk;

#[derive(Clone)]
pub struct ChunkNode {
    pub chunk: Chunk,

    pub prev: Option<Rc<ChunkNode>>,
}

impl ChunkNode {
    pub fn new(chunk: Chunk, prev: Option<Rc<ChunkNode>>) -> ChunkNode {
        ChunkNode { 
            chunk, 
            prev,
        }
    }
}
