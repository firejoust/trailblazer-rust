use std::collections::HashMap;

pub type BlockState = u16;
pub type BlockPos = [i32; 3];

pub type ColumnGrid = Vec<SectionGrid>;
pub type WorldGrid = Vec<ColumnGrid>;

#[derive(Debug)]
pub struct SectionGrid {
    pub position: BlockPos,
    pub blocks: [[[BlockState; 16]; 16]; 16]
}

#[derive(Debug)]
pub struct ChunkMap {
    pub grid: HashMap<BlockPos, SectionGrid>
}

impl ChunkMap {
    pub fn get_chunk(&self, pos: &BlockPos) -> Option<&SectionGrid> {
        self.grid.get(pos)
    }

    pub fn get_block(&self, pos: &BlockPos) -> Option<BlockState> {
        let chunkpos = [
            pos[0] >> 4,
            pos[1] >> 4,
            pos[2] >> 4
        ];
        let chunk = self.get_chunk(&chunkpos)?;
        // get the first 4 bits of the position (0-15 index)
        Some(
            chunk.blocks
            [(pos[0] - (chunkpos[0] << 4)) as usize] // x
            [(pos[1] - (chunkpos[1] << 4)) as usize] // y
            [(pos[2] - (chunkpos[2] << 4)) as usize] // z
        )
    }
}

pub fn get_chunk_map(world: WorldGrid) -> ChunkMap {
    let mut grid: HashMap<BlockPos, SectionGrid> = HashMap::new();
    for column in world {
        for chunk in column {
            // 
            grid.insert(chunk.position, chunk);
        }
    }
    ChunkMap { grid }
}