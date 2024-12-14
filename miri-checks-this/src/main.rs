#![allow(invalid_reference_casting)]

//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows
// src: https://github.com/rust-lang/miri/blob/21d531b37b51d80a997ef03be1fc09dfe40556b2/tests/fail/both_borrows/aliasing_mut1.rs
fn main() {
    use std::mem;

    pub fn safe(x: &mut i32, y: &mut i32) {
        //~[stack]^ ERROR: protect
        *x = 1; //~[tree] ERROR: /write access through .* is forbidden/
        *y = 2;
    }

    let mut x = 0;
    let xraw: *mut i32 = unsafe { mem::transmute(&mut x) };
    // We need to apply some tricky to be able to call `safe` with two mutable references
    // with the same tag: We transmute both the fn ptr (to take raw ptrs) and the argument
    // (to be raw, but still have the unique tag).
    let safe_raw: fn(x: *mut i32, y: *mut i32) =
        unsafe { mem::transmute::<fn(&mut i32, &mut i32), _>(safe) };

    safe_raw(xraw, xraw);
}

//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows
// src: https://github.com/rust-lang/miri/blob/21d531b37b51d80a997ef03be1fc09dfe40556b2/tests/fail/both_borrows/illegal_write1.rs
#[test]
fn illegal_write1() {
    let target = Box::new(42); // has an implicit raw
    let xref = &*target;
    {
        let x: *mut u32 = xref as *const _ as *mut _;
        unsafe { *x = 42 };
        //~[stack]^ ERROR: /write access .* tag only grants SharedReadOnly permission/
        //~[tree]| ERROR: /write access through .* is forbidden/
    }
    let _x = *xref;
}
