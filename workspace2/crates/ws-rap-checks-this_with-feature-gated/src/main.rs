struct MyRef<'a> {
    a: &'a str,
}

impl<'a> MyRef<'a> {
    fn print(&self) {
        println!("{}", self.a);
    }
}

unsafe fn f<'a>(myref: MyRef<'a>) -> MyRef<'static> {
    unsafe { std::mem::transmute(myref) }
}

#[cfg(feature = "rap")]
fn main() {
    let string = "Hello World!".to_string();
    unsafe {
        let my_ref = f(MyRef { a: &string });
        drop(string);
        my_ref.print(); // Expected to fail but executes without detection of use-after-free
    }
}

#[cfg(not(feature = "rap"))]
fn main() {}
