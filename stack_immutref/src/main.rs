fn main() {
    let mut local = 42;
    let x = &mut local;
    let shared1 = &*x;
    let shared2 = &*x;
    let val = *x;
    let val = *shared1;
    let val = *shared2;
    *x += 17;
    let val = *shared1;
}
