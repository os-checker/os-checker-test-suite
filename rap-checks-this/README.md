
```rust
os-checker-test-suite/rap-checks-this $ RAP_LOG=warn cargo +nightly-2024-06-30 rap -F
     Removed 26 files, 165.8KiB total
    Checking rap-checks-this v0.1.0 (/rust/my/os-checker-test-suite/rap-checks-this)
warning: function `main` is never used
 --> rap-checks-this/src/lib.rs:3:4
  |
3 | fn main() {
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

10:59|RAP-FRONT|WARN|: Use after free detected in function "main"
10:59|RAP-FRONT|WARN|: Location: rap-checks-this/src/lib.rs:10:15: 10:19 (#5)
10:59|RAP-FRONT|WARN|: Location: rap-checks-this/src/lib.rs:11:22: 11:23 (#7)
10:59|RAP-FRONT|WARN|: Location: rap-checks-this/src/lib.rs:11:15: 11:19 (#7)
10:59|RAP-FRONT|WARN|: Location: rap-checks-this/src/lib.rs:10:22: 10:26 (#5)
warning: `rap-checks-this` (lib) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
warning: function `main` is never used
 --> rap-checks-this/src/lib.rs:3:4
  |
3 | fn main() {
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

10:59|RAP-FRONT|WARN|: Use after free detected in function "main"
10:59|RAP-FRONT|WARN|: Location: rap-checks-this/src/lib.rs:10:15: 10:19 (#5)
10:59|RAP-FRONT|WARN|: Location: rap-checks-this/src/lib.rs:11:22: 11:23 (#7)
10:59|RAP-FRONT|WARN|: Location: rap-checks-this/src/lib.rs:11:15: 11:19 (#7)
10:59|RAP-FRONT|WARN|: Location: rap-checks-this/src/lib.rs:10:22: 10:26 (#5)
warning: `rap-checks-this` (lib) generated 1 warning
    Checking rap-checks-this v0.1.0 (/rust/my/os-checker-test-suite/rap-checks-this)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
```
