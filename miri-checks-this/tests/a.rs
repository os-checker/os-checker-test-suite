#![allow(invalid_reference_casting)]

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
