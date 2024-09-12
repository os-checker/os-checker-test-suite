//!
//! 注意：rustc 能检测到常量中的溢出算数，但检测不到函数调用后的溢出；
//!       而 mirai 能检测到这两者。
//!
//! 来自 rustc：
//! error: this arithmetic operation will overflow
//!  --> mirai-checks-this/src/main.rs:3:13
//!   |
//! 3 |     let a = u8::MAX + 1;
//!   |             ^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
//!   |
//!   = note: `#[deny(arithmetic_overflow)]` on by default
//!
//! 来自 mirai：
//! warning: attempt to add with overflow
//!  --> mirai-checks-this/src/main.rs:3:13
//!   |
//! 3 |     let a = u8::MAX + 1;
//!   |             ^^^^^^^^^^^
//!
//! 可是我们无法区分诊断来自 rustc 还是 mirai...

pub fn main() {
    let a = u8::MAX;
    // rustc 对此无诊断，但 mirai 会报告溢出 :）
    dbg!(a + b());
}

// $ cargo +nightly-2023-12-30 mirai
// warning: attempt to add with overflow
//  --> mirai-checks-this/src/main.rs:3:10
//   |
// 3 |     dbg!(a + b());
//   |          ^^^^^^^
#[cfg(not(feature = "zero"))]
fn b() -> u8 {
    1
}

#[cfg(feature = "zero")]
fn b() -> u8 {
    0
}
