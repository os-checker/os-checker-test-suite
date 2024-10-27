fn main() {
    println!("Hello, world!");
}

#[test]
fn ex1_test() {
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
