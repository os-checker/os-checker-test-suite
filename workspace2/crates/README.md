
# ✅ rap can check members without runing cargo clean

```rust
os-checker-test-suite/workspace2 $ cargo rap -F -M
22:14:42|RAP|INFO|: Start cargo-rap
     Removed 27 files, 412.6KiB total
22:14:42|RAP|INFO|: Execute cargo clean.
22:14:42|RAP|INFO|: Search local targets for analysis.
22:14:42|RAP|INFO|: Find a new pakage: "ws-rap-checks-this1".
22:14:42|RAP|INFO|: Find a new pakage: "ws-rap-checks-this2".
22:14:42|RAP|INFO|: Running rap for target bin:ws-rap-checks-this1
    Checking ws-rap-checks-this1 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this1)
22:14:43|RAP|INFO|: Execute after_analysis() of compiler callbacks
22:14:43|RAP|WARN|: RCanary: Leak Function: Unsat DefId(0:3 ~ ws_rap_checks_this1[127a]::main) crates/ws-rap-checks-this1/src/main.rs:1:1: 4:2 (#0)
22:14:43|RAP|WARN|: RCanary: LeakItem Candidates: _2 = std::boxed::Box::<&str>::into_raw(move _3) -> [return: bb2, unwind: bb3], crates/ws-rap-checks-this1/src/main.rs:3:16
: 3:34 (#0)
22:14:43|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.00s
22:14:43|RAP|INFO|: Running rap for target bin:ws-rap-checks-this2
    Checking ws-rap-checks-this2 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this2)
22:14:43|RAP|INFO|: Execute after_analysis() of compiler callbacks
22:14:43|RAP|WARN|: Use after free detected in function "main"
22:14:43|RAP|WARN|: Location: crates/ws-rap-checks-this2/src/main.rs:19:21: 19:29 (#6)
22:14:43|RAP|WARN|: Location: crates/ws-rap-checks-this2/src/main.rs:19:15: 19:19 (#6)
22:14:43|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
```
