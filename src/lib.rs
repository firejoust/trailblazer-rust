mod world;
mod js;

use world::container::{ ChunkColumn };
use world::chunk::{ ChunkGrid };
use js::parser::JsStructVec;
use neon::prelude::*;

fn load_buffer(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let buf = cx.argument::<JsArray>(0)?;
    let buf: Vec<ChunkColumn> = JsArray::from_arr(&mut cx, &buf);
    let _: Vec<ChunkGrid> = (&buf[0]).into();
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("loadBuffer", load_buffer)?;
    Ok(())
}
