NOTE: this result based on exclusion and removal of buildrs-failure package.

# `export RAP_RECURSIVE=deep RAP_LOG=info`

```rust
os-checker-test-suite $ cargo +nightly-2024-06-30 rap -F -M
11:15:58|RAP|INFO|: Start cargo-rap
11:15:59|RAP|INFO|: cargo clean in workspace root /rust/my/os-checker-test-suite
11:15:59|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/lockbud-checks-this
    Checking lockbud-checks-this v0.1.0 (/rust/my/os-checker-test-suite/lockbud-checks-this)
11:16:00|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:16:00|RAP|WARN|: Use after free detected in function "use_after_free"
11:16:00|RAP|WARN|: Location: lockbud-checks-this/src/lib.rs:8:23: 8:25 (#5)
11:16:00|RAP|WARN|: Location: lockbud-checks-this/src/lib.rs:7:13: 7:26 (#0)
11:16:00|RAP|WARN|: Location: lockbud-checks-this/src/lib.rs:8:28: 8:32 (#5)
11:16:00|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.25s
11:16:00|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/mirai-checks-this
    Checking mirai-checks-this v0.1.0 (/rust/my/os-checker-test-suite/mirai-checks-this)
11:16:01|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:16:01|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
11:16:01|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite
    Checking os-checker-test-suite v0.1.0 (/rust/my/os-checker-test-suite)
warning: unused variable: `a`
 --> src/main.rs:2:9
  |
2 |     let a = 3.14;
  |         ^ help: if this is intentional, prefix it with an underscore: `_a`
  |
  = note: `#[warn(unused_variables)]` on by default

11:16:01|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:16:01|RAP|INFO|: analysis done
warning: `os-checker-test-suite` (bin "os-checker-test-suite") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
11:16:01|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/rap-checks-this
    Checking rap-checks-this v0.1.0 (/rust/my/os-checker-test-suite/rap-checks-this)
warning: function `main` is never used
 --> rap-checks-this/src/lib.rs:4:4
  |
4 | fn main() {
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

11:16:02|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:16:02|RAP|WARN|: Use after free detected in function "main"
11:16:02|RAP|WARN|: Location: rap-checks-this/src/lib.rs:11:15: 11:19 (#5)
11:16:02|RAP|WARN|: Location: rap-checks-this/src/lib.rs:12:22: 12:23 (#7)
11:16:02|RAP|WARN|: Location: rap-checks-this/src/lib.rs:12:15: 12:19 (#7)
11:16:02|RAP|WARN|: Location: rap-checks-this/src/lib.rs:11:22: 11:26 (#5)
11:16:02|RAP|INFO|: analysis done
warning: `rap-checks-this` (lib) generated 1 warning
11:16:02|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:16:02|RAP|WARN|: RCanary: Leak Function: Unsat DefId(0:3 ~ rap_checks_this[8fe3]::main) rap-checks-this/src/main.rs:1:1: 4:2 (#0)
11:16:02|RAP|WARN|: RCanary: LeakItem Candidates: _2 = std::boxed::Box::<&str>::into_raw(move _3) -> [return: bb2, unwind: bb3], rap-checks-this/src/main.rs:3:16: 3:34 (#0)
11:16:02|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
11:16:02|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/rudra-checks-this
    Checking rudra-checks-this v0.1.0 (/rust/my/os-checker-test-suite/rudra-checks-this)
warning: struct `Leak` is never constructed
 --> rudra-checks-this/src/fp1.rs:8:12
  |
8 | pub struct Leak<'a> {
  |            ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: function `test_order_unsafe` is never used
  --> rudra-checks-this/src/order_unsafe.rs:10:4
   |
10 | fn test_order_unsafe<I: Iterator<Item = impl Debug>>(mut iter: I) {
   |    ^^^^^^^^^^^^^^^^^

warning: struct `Atom` is never constructed
 --> rudra-checks-this/src/wild_send.rs:8:8
  |
8 | struct Atom<P>(P);
  |        ^^^^

warning: struct `MyVec` is never constructed
 --> rudra-checks-this/src/vec_push_all.rs:8:12
  |
8 | pub struct MyVec<T>(Vec<T>);
  |            ^^^^^

warning: method `push_all` is never used
  --> rudra-checks-this/src/vec_push_all.rs:12:8
   |
10 | impl<T: Clone> MyVec<T> {
   | ----------------------- method in this implementation
11 |     // Example from: https://doc.rust-lang.org/nomicon/exception-safety.html#vecpush_all
12 |     fn push_all(&mut self, to_push: &[T]) {
   |        ^^^^^^^^

11:16:03|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:16:03|RAP|WARN|: Double free detected in function test_order_unsafe
11:16:03|RAP|WARN|: Location: rudra-checks-this/src/order_unsafe.rs:12:52: 12:53 (#0)
11:16:03|RAP|INFO|: analysis done
warning: `rudra-checks-this` (lib) generated 5 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
11:16:03|RAP|INFO|: cargo clean in workspace root /rust/my/os-checker-test-suite/workspace2
11:16:04|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/workspace2/crates/ws-lockbud-checks-this1
   Compiling proc-macro2 v1.0.89
   Compiling unicode-ident v1.0.13
   Compiling libc v0.2.161
    Checking cfg-if v1.0.0
    Checking byteorder v1.5.0
    Checking getrandom v0.2.15
    Checking rand_core v0.6.4
   Compiling quote v1.0.37
   Compiling syn v2.0.85
   Compiling zerocopy-derive v0.7.35
    Checking zerocopy v0.7.35
    Checking ppv-lite86 v0.2.20
    Checking rand_chacha v0.3.1
    Checking rand v0.8.5
    Checking ws-lockbud-checks-this1 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-lockbud-checks-this1)
warning: creating a shared reference to mutable static is discouraged
   --> crates/ws-lockbud-checks-this1/src/main.rs:101:9
    |
101 |         &HOST_ENTRY as *const hostent
    |         ^^^^^^^^^^^ shared reference to mutable static
    |
    = note: for more information, see issue #114447 <https://github.com/rust-lang/rust/issues/114447>
    = note: this will be a hard error in the 2024 edition
    = note: this shared reference has lifetime `'static`, but if the static ever gets mutated, or a mutable reference is created, then any further use of this shared refere
nce is Undefined Behavior
    = note: `#[warn(static_mut_refs)]` on by default
help: use `addr_of!` instead to create a raw pointer
    |
101 |         addr_of!(HOST_ENTRY) as *const hostent
    |         ~~~~~~~~~~~~~~~~~~~~

11:16:45|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:16:46|RAP|WARN|: Dangling pointer detected in function "fmt_time"!!! 
                      Location: crates/ws-lockbud-checks-this1/src/main.rs:113:1: 131:2 (#0)
11:16:46|RAP|INFO|: analysis done
warning: `ws-lockbud-checks-this1` (bin "ws-lockbud-checks-this1") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 42.63s
11:16:47|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/workspace2/crates/ws-lockbud-checks-this2
   Compiling libc v0.2.161
    Checking getrandom v0.2.15
    Checking rand_core v0.6.4
    Checking rand_chacha v0.3.1
    Checking rand v0.8.5
    Checking ws-lockbud-checks-this2 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-lockbud-checks-this2)
11:16:59|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:16:59|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 12.51s
11:16:59|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-test-and-example
    Checking ws-rap-checks-test-and-example v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-test-and-example)
11:17:00|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:17:00|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
11:17:00|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this1
    Checking ws-rap-checks-this1 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this1)
11:17:00|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:17:00|RAP|WARN|: RCanary: Leak Function: Unsat DefId(0:3 ~ ws_rap_checks_this1[5aa2]::main) crates/ws-rap-checks-this1/src/main.rs:1:1: 4:2 (#0)
11:17:00|RAP|WARN|: RCanary: LeakItem Candidates: _2 = std::boxed::Box::<&str>::into_raw(move _3) -> [return: bb2, unwind: bb3], crates/ws-rap-checks-this1/src/main.rs:3:16
: 3:34 (#0)
11:17:00|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
11:17:00|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this2
    Checking ws-rap-checks-this2 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this2)
11:17:00|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:17:01|RAP|WARN|: Use after free detected in function "main"
11:17:01|RAP|WARN|: Location: crates/ws-rap-checks-this2/src/main.rs:19:21: 19:29 (#6)
11:17:01|RAP|WARN|: Location: crates/ws-rap-checks-this2/src/main.rs:19:15: 19:19 (#6)
11:17:01|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s
11:17:01|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this_with-build-script
   Compiling ws-rap-checks-this_with-build-script v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this_with-build-script)
11:17:02|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:17:02|RAP|WARN|: Use after free detected in function "main"
11:17:02|RAP|WARN|: Location: crates/ws-rap-checks-this_with-build-script/src/main.rs:8:23: 8:25 (#5)
11:17:02|RAP|WARN|: Location: crates/ws-rap-checks-this_with-build-script/src/main.rs:8:28: 8:32 (#5)
11:17:02|RAP|WARN|: Location: crates/ws-rap-checks-this_with-build-script/src/main.rs:7:13: 7:26 (#0)
11:17:02|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
11:17:02|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this_with-feature-gated
    Checking ws-rap-checks-this_with-feature-gated v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this_with-feature-gated)
warning: struct `MyRef` is never constructed
 --> crates/ws-rap-checks-this_with-feature-gated/src/main.rs:1:8
  |
1 | struct MyRef<'a> {
  |        ^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: method `print` is never used
 --> crates/ws-rap-checks-this_with-feature-gated/src/main.rs:6:8
  |
5 | impl<'a> MyRef<'a> {
  | ------------------ method in this implementation
6 |     fn print(&self) {
  |        ^^^^^

warning: function `f` is never used
  --> crates/ws-rap-checks-this_with-feature-gated/src/main.rs:11:11
   |
11 | unsafe fn f<'a>(myref: MyRef<'a>) -> MyRef<'static> {
   |           ^

11:17:02|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:17:02|RAP|INFO|: analysis done
warning: `ws-rap-checks-this_with-feature-gated` (bin "ws-rap-checks-this_with-feature-gated") generated 3 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
```


# `export RAP_RECURSIVE=shallow RAP_LOG=info`

```rust
os-checker-test-suite $ cargo +nightly-2024-06-30 rap -F -M
11:36:38|RAP|INFO|: Start cargo-rap
11:36:46|RAP|INFO|: cargo clean in workspace root /rust/my/os-checker-test-suite
11:36:48|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/lockbud-checks-this
    Checking lockbud-checks-this v0.1.0 (/rust/my/os-checker-test-suite/lockbud-checks-this)
11:36:50|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:36:50|RAP|WARN|: Use after free detected in function "use_after_free"
11:36:50|RAP|WARN|: Location: lockbud-checks-this/src/lib.rs:8:23: 8:25 (#5)
11:36:50|RAP|WARN|: Location: lockbud-checks-this/src/lib.rs:7:13: 7:26 (#0)
11:36:50|RAP|WARN|: Location: lockbud-checks-this/src/lib.rs:8:28: 8:32 (#5)
11:36:50|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.18s
11:36:50|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/mirai-checks-this
    Checking mirai-checks-this v0.1.0 (/rust/my/os-checker-test-suite/mirai-checks-this)
11:36:51|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:36:51|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.89s
11:36:51|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite
    Checking os-checker-test-suite v0.1.0 (/rust/my/os-checker-test-suite)
warning: unused variable: `a`
 --> src/main.rs:2:9
  |
2 |     let a = 3.14;
  |         ^ help: if this is intentional, prefix it with an underscore: `_a`
  |
  = note: `#[warn(unused_variables)]` on by default

11:36:52|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:36:52|RAP|INFO|: analysis done
warning: `os-checker-test-suite` (bin "os-checker-test-suite") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
11:36:52|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/rap-checks-this
    Checking rap-checks-this v0.1.0 (/rust/my/os-checker-test-suite/rap-checks-this)
warning: function `main` is never used
 --> rap-checks-this/src/lib.rs:4:4
  |
4 | fn main() {
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

11:36:53|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:36:53|RAP|WARN|: Use after free detected in function "main"
11:36:53|RAP|WARN|: Location: rap-checks-this/src/lib.rs:11:15: 11:19 (#5)
11:36:53|RAP|WARN|: Location: rap-checks-this/src/lib.rs:12:22: 12:23 (#7)
11:36:53|RAP|WARN|: Location: rap-checks-this/src/lib.rs:12:15: 12:19 (#7)
11:36:53|RAP|WARN|: Location: rap-checks-this/src/lib.rs:11:22: 11:26 (#5)
11:36:53|RAP|INFO|: analysis done
warning: `rap-checks-this` (lib) generated 1 warning
11:36:53|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:36:53|RAP|WARN|: RCanary: Leak Function: Unsat DefId(0:3 ~ rap_checks_this[8fe3]::main) rap-checks-this/src/main.rs:1:1: 4:2 (#0)
11:36:53|RAP|WARN|: RCanary: LeakItem Candidates: _2 = std::boxed::Box::<&str>::into_raw(move _3) -> [return: bb2, unwind: bb3], rap-checks-this/src/main.rs:3:16: 3:34 (#0)
11:36:53|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.03s
11:36:53|RAP|INFO|: cargo check in package folder /rust/my/os-checker-test-suite/rudra-checks-this
    Checking rudra-checks-this v0.1.0 (/rust/my/os-checker-test-suite/rudra-checks-this)
warning: struct `Leak` is never constructed
 --> rudra-checks-this/src/fp1.rs:8:12
  |
8 | pub struct Leak<'a> {
  |            ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: function `test_order_unsafe` is never used
  --> rudra-checks-this/src/order_unsafe.rs:10:4
   |
10 | fn test_order_unsafe<I: Iterator<Item = impl Debug>>(mut iter: I) {
   |    ^^^^^^^^^^^^^^^^^

warning: struct `MyVec` is never constructed
 --> rudra-checks-this/src/vec_push_all.rs:8:12
  |
8 | pub struct MyVec<T>(Vec<T>);
  |            ^^^^^

warning: method `push_all` is never used
  --> rudra-checks-this/src/vec_push_all.rs:12:8
   |
10 | impl<T: Clone> MyVec<T> {
   | ----------------------- method in this implementation
11 |     // Example from: https://doc.rust-lang.org/nomicon/exception-safety.html#vecpush_all
12 |     fn push_all(&mut self, to_push: &[T]) {
   |        ^^^^^^^^

warning: struct `Atom` is never constructed
 --> rudra-checks-this/src/wild_send.rs:8:8
  |
8 | struct Atom<P>(P);
  |        ^^^^

11:36:54|RAP|INFO|: Execute after_analysis() of compiler callbacks
11:36:54|RAP|WARN|: Double free detected in function test_order_unsafe
11:36:54|RAP|WARN|: Location: rudra-checks-this/src/order_unsafe.rs:12:52: 12:53 (#0)
11:36:54|RAP|INFO|: analysis done
warning: `rudra-checks-this` (lib) generated 5 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
```
