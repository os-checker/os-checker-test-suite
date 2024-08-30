pub fn use_after_free() {
    let ptr = {
        let v = Vec::<u8>::new();
        v.as_ptr()
    };
    unsafe {
        if !ptr.is_null() {
            println!("{}", *ptr);
        }
    }
}
