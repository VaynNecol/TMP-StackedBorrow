#![allow(unused_variables)]

static mut ALIAS: *mut i32 = core::ptr::null_mut();
fn main() {
    let mut l = 13;
    unsafe {
        ALIAS = &mut l as *mut i32;
    }
    let answer = test_unique(&mut l);
    println!("The answer is {}", answer);
}
fn unknown_function() {
    unsafe {
        *ALIAS = 7;
    }
}
fn test_unique(x: &mut i32) -> i32 {
    *x = 42;
    unknown_function();
    return *x;
}
