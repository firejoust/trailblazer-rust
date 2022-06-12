use neon::prelude::{
    FunctionContext,
    JsTypedArray,
    JsArray,
    JsNumber,
    JsObject,
    Object,
    Handle
};

pub trait JsInt<T> {
    fn to_int(&self, cx: &mut FunctionContext) -> T;
}

pub trait JsIntVec<T> {
    fn to_int_vec(&self, cx: &mut FunctionContext) -> Vec<T>;
} 

pub trait JsStruct {
    fn from_obj(cx: &mut FunctionContext, obj: &JsObject) -> Self;
}

pub trait JsStructVec<T: JsStruct> {
    fn from_arr(cx: &mut FunctionContext, arr: &JsArray) -> Vec<T>;
}

/*
//  JS TypedArray to Rust Vector
*/

macro_rules! impl_js_array_type {
    ($t:tt) => {
        impl JsIntVec<$t> for JsTypedArray<$t> {
            fn to_int_vec(&self, cx: &mut FunctionContext) -> Vec<$t> {
                let len: Handle<JsNumber> = self.get(cx, "length").unwrap();
                let len = len.value(cx) as usize;
                let mut arr: Vec<$t> = vec![];
                for i in 0..len {
                    let jsval: Handle<JsNumber> = self.get(cx, i as u32).unwrap();
                    let jsval = jsval.value(cx) as $t;
                    arr.push(jsval);
                }
                arr
            }
        }
    }
}

impl_js_array_type!(u64);
impl_js_array_type!(u32);
impl_js_array_type!(u16);
impl_js_array_type!(u8);

impl_js_array_type!(i64);
impl_js_array_type!(i32);
impl_js_array_type!(i16);
impl_js_array_type!(i8);

/*
//  JS Number to Rust Integer
*/

macro_rules! impl_js_number {
    ($t:tt) => {
        impl JsInt<$t> for JsNumber {
            fn to_int(&self, cx: &mut FunctionContext) -> $t {
                self.value(cx) as $t
            }
        }
    }
}

impl_js_number!(u64);
impl_js_number!(u32);
impl_js_number!(u16);
impl_js_number!(u8);

impl_js_number!(i64);
impl_js_number!(i32);
impl_js_number!(i16);
impl_js_number!(i8);