mod world;
mod js;

use world::container::{
    get_world_grid,
    ChunkWorld,
};
use world::chunk::{
    get_chunk_map,
    WorldGrid,
    ChunkMap
};
use js::parser::{
    JsStructVec
};
use neon::prelude::{
    FunctionContext,
    ModuleContext,
    NeonResult,
    JsResult,
    JsUndefined,
    JsArray,
    Context
};

fn load_buffer(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let buf = cx.argument::<JsArray>(0)?;
    let world: ChunkWorld = JsArray::from_arr(&mut cx, &buf);
    let grid: WorldGrid = get_world_grid(&world);
    let map: ChunkMap = get_chunk_map(grid);
    println!("{:?}", map.get_chunk(&[0, 8, 0]));
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("loadBuffer", load_buffer)?;
    Ok(())
}
