use super::chunk::{
    BlockState,
    SectionGrid,
    ColumnGrid,
    WorldGrid
};

const SECTION_BITS: usize = 16;
const SECTION_CAPACITY: usize = 4096;

// state lists (unordered, ordered)
type UStates = [BlockState; 4096];
type OStates = [[[BlockState; 16]; 16]; 16];

// world represented in chunks
pub type ChunkWorld = Vec<ChunkColumn>;

// array of blockstates stored as partial longs
#[derive(Debug)]
pub struct BitArray {
    pub data: Vec<u32>,
    pub capacity: u32,
    pub bits_per_value: u32,
    pub spanned: bool
}

#[derive(Debug)]
pub struct ChunkSection {
    pub data: BitArray,
    pub palette: Vec<BlockState>,
}

#[derive(Debug)]
pub struct ChunkColumn {
    pub x: i32,
    pub z: i32,
    pub sections: Vec<ChunkSection>
}

fn get_unordered_states(bits: &BitArray) -> UStates {
    if bits.capacity == 1 {
        // will panic if data is too large
        return [bits.data[0] as BlockState; SECTION_CAPACITY]
    }

    assert!(bits.capacity as usize == SECTION_CAPACITY);
    assert!(bits.bits_per_value as usize <= SECTION_BITS);

    // record buffer length
    let bl = bits.data.len();
    let dl = SECTION_CAPACITY;

    // initialise half-long bit capacity 64 = 32 + (32 - mod)
    let mut lh = false;
    let mut ln = 32; // current half bit capacity
    let ls = 32 - (64 % bits.bits_per_value); //  second-half bit capacity

    // initialise references to old/new buffers
    let mut bb = bits.data.clone();
    let mut db = [0; SECTION_CAPACITY];

    // initialise buffer position index
    let mut b = 0;
    let mut d = 0;

    // initialise counters for current long & block state bits
    let mut l = 0;
    let mut n = 0;

    // extract blockstates into new buffer from compacted data format
    while d < dl && b < bl {
        // shift to get the foremost insignificant bit
        let long = bb[b];
        let part = long >> 1;

        // non 0 bit, append to new buffer
        if part << 1 < long {
            db[d] += BlockState::pow(2, n);
        }
        bb[b] = part;

        l += 1;
        n += 1;

        if l >= ln {
            l = 0;
            b += 1;
            // determine length of the next half-long (non-spanned)
            ln = if bits.spanned { ln } else {
                lh = !lh;
                if lh { ls } else { 32 }
            };
        }

        if n >= bits.bits_per_value {
            n = 0;
            d += 1;
        }
    }
    db
}

fn get_ordered_states(section: &ChunkSection) -> OStates {
    let mut db = get_unordered_states(&section.data);
    let mut cb = [[[0; 16]; 16]; 16];

    // indirect palette conversion
    if section.palette.len() > 0 {
        for i in 0..SECTION_CAPACITY {
            db[i] = section.palette[db[i] as usize];
        }
    }

    // order blockstates by axis
    let mut c = 0;
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                cb[x][y][z] = db[c];
                c += 1;
            }
        }
    }
    cb
}

fn get_column_grid(column: &ChunkColumn) -> ColumnGrid {
    let mut arr = vec![];
    for y in 0..column.sections.len() {
        let section = &column.sections[y];
        arr.push(
            SectionGrid {
                position: [column.x, y as i32, column.z],
                blocks: get_ordered_states(&section)
            }
        );
    }
    arr
}

pub fn get_world_grid(world: &ChunkWorld) -> WorldGrid {
    let mut arr = vec![];
    for column in world {
        arr.push(get_column_grid(column));
    }
    arr
}