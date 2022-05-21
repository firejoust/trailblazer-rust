use std::collections::HashMap;

pub type BlockState = u16;
pub type BlockPos = [i32; 3];

#[derive(Debug)]
pub struct ChunkGrid {
    pub position: BlockPos,
    pub blocks: [[[BlockState; 16]; 16]; 16]
}

impl Into<ChunkMap> for Vec<ChunkGrid> {
    fn into(self) -> ChunkMap {
        let mut grid: HashMap<BlockPos, ChunkGrid> = HashMap::new();
        for chunk in self.into_iter() {
            grid.insert(chunk.position, chunk);
        }
        ChunkMap { grid }
    }
}

#[derive(Debug)]
pub struct ChunkMap {
    pub grid: HashMap<BlockPos, ChunkGrid>
}

impl ChunkMap {
    fn get_chunk(&self, pos: &BlockPos) -> Option<&ChunkGrid> {
        self.grid.get(pos)
    }

    fn get_block(&self, pos: &BlockPos) -> Option<BlockState> {
        let chunkpos = [pos[0] >> 4, pos[1] >> 4, pos[2] >> 4];
        let chunk = self.get_chunk(&chunkpos)?;
        // get the first 4 bits of the position (0-15 index)
        Some(
            chunk.blocks
            [(pos[0] - (chunkpos[0] << 4)) as usize]
            [(pos[1] - (chunkpos[1] << 4)) as usize]
            [(pos[2] - (chunkpos[2] << 4)) as usize]
        )
    }
}