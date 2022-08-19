use super::primative::{
    JsIntVec,
    JsInt,
    JsStruct,
    JsStructVec
};
use super::world::container::{
    BitArray,
    ChunkSection,
    ChunkColumn
};
use neon::prelude::{
    FunctionContext,
    JsTypedArray,
    JsArray,
    JsBoolean,
    JsNumber,
    JsObject,
    Object,
    Handle
};

//
//  Convert abstracted JS objects to their structural equivalent 
//  with Neon type bindings
//

/*
**  BitArray
*/

impl JsStruct for BitArray {
    fn from_obj(cx: &mut FunctionContext, obj: &JsObject) -> Self {
        let data: Handle<JsTypedArray<u32>> = obj.get(cx, "data").unwrap();
        let capacity: Handle<JsNumber> = obj.get(cx, "capacity").unwrap();
        let bits_per_value: Handle<JsNumber> = obj.get(cx, "bitsPerValue").unwrap();
        let spanned: Handle<JsBoolean> = obj.get(cx, "spanned").unwrap();
        Self {
            data: data.to_int_vec(cx),
            capacity: capacity.to_int(cx),
            bits_per_value: bits_per_value.to_int(cx),
            spanned: spanned.value(cx)
        }
    }
}

/*
**  ChunkSection
*/

impl JsStruct for ChunkSection {
    fn from_obj(cx: &mut FunctionContext, obj: &JsObject) -> Self {
        let data: Handle<JsObject> = obj.get(cx, "data").unwrap();
        let palette: Handle<JsTypedArray<u16>> = obj.get(cx, "palette").unwrap();
        Self {
            data: BitArray::from_obj(cx, &data),
            palette: palette.to_int_vec(cx)
        }
    }
}

/*
**  ChunkColumn
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

/*
**  Arrays of Objects (ChunkSections, ChunkColumns)
*/

macro_rules! impl_js_obj_arr {
    ($t:ident) => {
        impl JsStructVec<$t> for JsArray {
            fn from_arr(cx: &mut FunctionContext, arr: &JsArray) -> Vec<$t> {
                let len: Handle<JsNumber> = arr.get(cx, "length").unwrap();
                let len = len.value(cx) as usize;
                let mut list: Vec<$t> = vec![];
                for i in 0..len {
                    let element: Handle<JsObject> = arr.get(cx, i as u32).unwrap();
                    list.push($t::from_obj(cx, &element));
                }
                list
            }
        }
    }
}

impl_js_obj_arr!(ChunkSection);
impl_js_obj_arr!(ChunkColumn);