
use super::parser::{
    JsIntVec, JsInt, JsStruct, JsStructVec
};

use crate::world::chunk::{
    BitArray, ChunkSection, ChunkColumn
};

use neon::prelude::{
    Handle,
    FunctionContext,
    Object,
    JsObject, JsTypedArray, JsArray, JsNumber
};


/*
**  JS Object to Rust Struct (BitArray)
*/

impl JsStruct for BitArray {
    fn from_obj(cx: &mut FunctionContext, obj: &JsObject) -> Self {
        let data: Handle<JsTypedArray<u32>> = obj.get(cx, "data").unwrap();
        let capacity: Handle<JsNumber> = obj.get(cx, "capacity").unwrap();
        let bits_per_value: Handle<JsNumber> = obj.get(cx, "bits_per_value").unwrap();
        let value_mask: Handle<JsNumber> = obj.get(cx, "value_mask").unwrap();
        Self {
            data: data.to_int_vec(cx),
            capacity: capacity.to_int(cx),
            bits_per_value: bits_per_value.to_int(cx),
            value_mask: value_mask.to_int(cx)
        }
    }
}

/*
**  JS Object to Rust Struct (ChunkSection)
*/

impl JsStruct for ChunkSection {
    fn from_obj(cx: &mut FunctionContext, obj: &JsObject) -> Self {
        let palette: Handle<JsTypedArray<u32>> = obj.get(cx, "palette").unwrap();
        Self {
            data: BitArray::from_obj(cx, obj),
            palette: palette.to_int_vec(cx)
        }
    }
}

impl JsStructVec<ChunkSection> for JsArray {
    fn from_arr(cx: &mut FunctionContext, arr: &JsArray) -> Vec<ChunkSection> {
        let len: Handle<JsNumber> = arr.get(cx, "length").unwrap();
        let len = len.value(cx) as usize;
        let mut sections: Vec<ChunkSection> = vec![];
        for i in 0..len {
            let element: Handle<JsObject> = arr.get(cx, i as u32).unwrap();
            sections[i] = ChunkSection::from_obj(cx, &element);
        }
        sections
    }
}

/*
**  JS Object to Rust Struct (ChunkColumn)
*/

impl JsStruct for ChunkColumn {
    fn from_obj(cx: &mut FunctionContext, obj: &JsObject) -> Self {
        let x: Handle<JsNumber> = obj.get(cx, "x").unwrap();
        let z: Handle<JsNumber> = obj.get(cx, "z").unwrap();
        let sections: Handle<JsArray> = obj.get(cx, "sections").unwrap();
        Self {
            x: x.value(cx) as i32,
            z: z.value(cx) as i32,
            sections: JsArray::from_arr(cx, &sections) 
        }
    }
}