#![allow(unused_variables)]

fn main() {
    let mut l = 13;
    let alias = &mut l as *mut i32;
    let x = &mut l;
    *x = 42;
    unsafe {
        *alias = 7;
    }
    println!("The answer is {}", *x);
}
