/**
 * Ownership Rules:
 *
 * 1) Each value has a variable which is its owner
 * 2) There can only be one owner at any given time
 * 3) When the owner goes out of scope, the value will be dropped
 *
 * Borrowing Rules:
 *
 * 1) Allowed infinite borrows for read-only access and borrowing will last until the end of scope
 * 2) Read-only borrows make the original data immutable for their duration
 * 3) Only allowed to pass one borrow at a time for write access/mutability
 */

fn main() {
    // For primitive type -> copy
    // bool, character, numbers,
    let a = 10;
    let b = a; // copy
    let c = a; // copy

    println!("a: {} b: {} c: {}", a, b, c);

    // Dynamic heap type
    let s = String::from("String");
    let y = s;

    // -> this is impossible since value of s moved to y
    // ownership in Rust means only one reference can own a piece of data at a time
    // println!("{}", s);
    println!("{}", y);

    // on the other hand, next code is possible since it passes reference
    // so ownership hasn't changed
    let z = &y;
    println!("{}", y);
    println!("{}", z);
}
