use neon::handle::{Handle};
use neon::context::FunctionContext;
use neon::prelude::{
    Object,
    JsTypedArray,
    JsNumber
};

trait JsIntVec<T> {
    fn to_int_vec(&self, cx: FunctionContext) -> Vec<T>;
} 

trait JsInt<T> {
    fn to_int(&self, cx: FunctionContext) -> T;
}

/*
**  JS TypedArray to Rust Vector
*/

macro_rules! impl_js_array_type {
    ($t:tt) => {
        impl JsIntVec<$t> for JsTypedArray<$t> {
            fn to_int_vec(&self, mut cx: FunctionContext) -> Vec<$t> {
                let len: Handle<JsNumber> = self.get(&mut cx, "length").unwrap();
                let len = len.value(&mut cx) as usize;
                let mut arr: Vec<$t> = vec![];
                for i in 0..len {
                    let jsval: Handle<JsNumber> = self.get(&mut cx, i as u32).unwrap();
                    let jsval = jsval.value(&mut cx) as $t;
                    arr[i] = jsval;
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
            fn to_int(&self, mut cx: FunctionContext) -> $t {
                self.value(&mut cx) as $t
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