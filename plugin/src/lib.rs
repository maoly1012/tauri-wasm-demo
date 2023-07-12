#[allow(unused)]
use wasmedge_bindgen::*;
#[wasmedge_bindgen_macro::wasmedge_bindgen]
pub fn hello(){
    println!("hello");
}