#![allow(unused_variables)]

fn main() {
    let mut l = 13;
    let a = &mut l; 
    let b = &mut *a;
    *b = 3;
    *a = 4;
    *b = 4;
}
