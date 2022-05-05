use crate::world::block::Block;

struct BitArray {
    data: Vec<u32>,
    capacity: u32,
    bits_per_value: u32,
    value_mask: u32
}

struct ChunkSection {
    data: BitArray,
    palette: Vec<u32>,
}

struct ChunkColumn {
    x: i32,
    z: i32,
    sections: Vec<ChunkSection>
}

struct ChunkGrid {
    position: [i32; 3],
    blocks: [[[Block; 16]; 16]; 16]
}
