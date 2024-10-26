
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

# ✅ rap can handle cargo-check's --target and --features

E.g. for `ws-rap-checks-this_with-feature-gated`, when no feature is enabled, the result is:

```rust
workspace2/crates/ws-rap-checks-this_with-feature-gated $ cargo rap -F -M
14:42:42|RAP|INFO|: Start cargo-rap
14:42:42|RAP|INFO|: Search local targets for analysis.
14:42:42|RAP|INFO|: Find a new pakage: "ws-rap-checks-this_with-feature-gated".
14:42:42|RAP|INFO|: Running rap for target bin:ws-rap-checks-this_with-feature-gated
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

14:40:02|RAP|INFO|: Execute after_analysis() of compiler callbacks
14:40:02|RAP|INFO|: analysis done
warning: `ws-rap-checks-this_with-feature-gated` (bin "ws-rap-checks-this_with-feature-gated") generated 3 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
```

if `rap` feature enabled, there's a checking result:

```rust
workspace2/crates/ws-rap-checks-this_with-feature-gated $ cargo rap -F -M -- -F rap
14:42:30|RAP|INFO|: Start cargo-rap
14:42:30|RAP|INFO|: Search local targets for analysis.
14:42:30|RAP|INFO|: Find a new pakage: "ws-rap-checks-this_with-feature-gated".
14:42:30|RAP|INFO|: Running rap for target bin:ws-rap-checks-this_with-feature-gated
14:40:10|RAP|INFO|: Execute after_analysis() of compiler callbacks
14:40:11|RAP|WARN|: Use after free detected in function "main"
14:40:11|RAP|WARN|: Location: crates/ws-rap-checks-this_with-feature-gated/src/main.rs:21:9: 21:15 (#0)
14:40:11|RAP|WARN|: Location: crates/ws-rap-checks-this_with-feature-gated/src/main.rs:21:9: 21:23 (#0)
14:40:11|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
```


# ✅ lockbud supports no cargo clean too, also supports cargo args 

```rust
os-checker-test-suite/workspace2/crates $ cargo +nightly-2024-05-21 lockbud -k all -- --workspace
   Compiling proc-macro2 v1.0.89
   Compiling unicode-ident v1.0.13
   Compiling libc v0.2.161
   Compiling cfg-if v1.0.0
   Compiling byteorder v1.5.0
   Compiling ws-rap-checks-this1 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this1)
   Compiling ws-rap-checks-this2 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this2)
   Compiling getrandom v0.2.15
[2024-10-25T15:11:30Z WARN  lockbud::callbacks] [
      {
        "AtomicityViolation": {
          "bug_kind": "AtomicityViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "fn_name": "lazy::LazyUsize::unsync_init",
            "atomic_reader": "/root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/getrandom-0.2.15/src/lazy.rs:36:23: 36:43 (#0)",
            "atomic_writer": "/root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/getrandom-0.2.15/src/lazy.rs:39:13: 39:39 (#0)",
            "dep_kind": "Both"
          },
          "explanation": "atomic::store is data/control dependent on atomic::load"
        }
      }
    ]
[2024-10-25T15:11:30Z WARN  lockbud::callbacks] crate getrandom contains bugs: { probably: 0, possibly: 0 }, conflictlock: { probably: 0, possibly: 0 }, condvar_deadlock: {
 probably: 0, possibly: 0 }, atomicity_violation: { possibly: 1 }, invalid_free: { possibly: 0 }, use_after_free: { possibly: 0 }
   Compiling rand_core v0.6.4
   Compiling quote v1.0.37
   Compiling syn v2.0.85
   Compiling zerocopy-derive v0.7.35
   Compiling zerocopy v0.7.35
   Compiling ppv-lite86 v0.2.20
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling ws-lockbud-checks-this1 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-lockbud-checks-this1)
   Compiling ws-lockbud-checks-this2 v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-lockbud-checks-this2)
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

[2024-10-25T15:11:36Z WARN  lockbud::callbacks] [
      {
        "AtomicityViolation": {
          "bug_kind": "AtomicityViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "fn_name": "buggy_data_dep_i32",
            "atomic_reader": "crates/ws-lockbud-checks-this2/src/main.rs:33:13: 33:38 (#0)",
            "atomic_writer": "crates/ws-lockbud-checks-this2/src/main.rs:36:5: 36:35 (#0)",
            "dep_kind": "Data"
          },
          "explanation": "atomic::store is data/control dependent on atomic::load"
        }
      },
      {
        "AtomicityViolation": {
          "bug_kind": "AtomicityViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "fn_name": "buggy_control_dep_i32",
            "atomic_reader": "crates/ws-lockbud-checks-this2/src/main.rs:22:13: 22:38 (#0)",
            "atomic_writer": "crates/ws-lockbud-checks-this2/src/main.rs:26:9: 26:39 (#0)",
            "dep_kind": "Control"
          },
          "explanation": "atomic::store is data/control dependent on atomic::load"
        }
      },
      {
        "AtomicityViolation": {
          "bug_kind": "AtomicityViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "fn_name": "buggy_both_dep_i32",
            "atomic_reader": "crates/ws-lockbud-checks-this2/src/main.rs:42:13: 42:38 (#0)",
            "atomic_writer": "crates/ws-lockbud-checks-this2/src/main.rs:46:9: 46:39 (#0)",
            "dep_kind": "Both"
          },
          "explanation": "atomic::store is data/control dependent on atomic::load"
        }
      },
      {
        "AtomicityViolation": {
          "bug_kind": "AtomicityViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "fn_name": "buggy_control_dep_bool",
            "atomic_reader": "crates/ws-lockbud-checks-this2/src/main.rs:14:8: 14:33 (#0)",
            "atomic_writer": "crates/ws-lockbud-checks-this2/src/main.rs:15:9: 15:42 (#0)",
            "dep_kind": "Control"
          },
          "explanation": "atomic::store is data/control dependent on atomic::load"
        }
      }
    ]
[2024-10-25T15:11:36Z WARN  lockbud::callbacks] crate ws_lockbud_checks_this2 contains bugs: { probably: 0, possibly: 0 }, conflictlock: { probably: 0, possibly: 0 }, condv
ar_deadlock: { probably: 0, possibly: 0 }, atomicity_violation: { possibly: 4 }, invalid_free: { possibly: 0 }, use_after_free: { possibly: 0 }
[2024-10-25T15:11:36Z WARN  lockbud::callbacks] [
      {
        "UseAfterFree": {
          "bug_kind": "UseAfterFree",
          "possibility": "Possibly",
          "diagnosis": "Escape to Param/Return: Raw ptr _5 at crates/ws-lockbud-checks-this1/src/main.rs:46:13: 46:21 (#0) escapes to [(_1.0: std::sync::atomic::AtomicPtr<T
>)] but pointee is dropped at crates/ws-lockbud-checks-this1/src/main.rs:47:9: 47:10 (#0)",
          "explanation": "Raw ptr is used or escapes the current function after the pointed value is dropped"
        }
      },
      {
        "UseAfterFree": {
          "bug_kind": "UseAfterFree",
          "possibility": "Possibly",
          "diagnosis": "Raw ptr is used at crates/ws-lockbud-checks-this1/src/main.rs:131:2: 131:2 (#0) after dropped at crates/ws-lockbud-checks-this1/src/main.rs:131:1: 1
31:2 (#0)",
          "explanation": "Raw ptr is used or escapes the current function after the pointed value is dropped"
        }
      },
      {
        "UseAfterFree": {
          "bug_kind": "UseAfterFree",
          "possibility": "Possibly",
          "diagnosis": "Escape to Param/Return: Raw ptr _0 at crates/ws-lockbud-checks-this1/src/main.rs:113:36: 113:49 (#0) escapes to [_0] but pointee is dropped at crate
s/ws-lockbud-checks-this1/src/main.rs:131:1: 131:2 (#0)",
          "explanation": "Raw ptr is used or escapes the current function after the pointed value is dropped"
        }
      },
      {
        "UseAfterFree": {
          "bug_kind": "UseAfterFree",
          "possibility": "Possibly",
          "diagnosis": "Raw ptr is used at crates/ws-lockbud-checks-this1/src/main.rs:14:13: 14:26 (#0) after dropped at crates/ws-lockbud-checks-this1/src/main.rs:10:37: 1
0:38 (#0)",
          "explanation": "Raw ptr is used or escapes the current function after the pointed value is dropped"
        }
      },
      {
        "UseAfterFree": {
          "bug_kind": "UseAfterFree",
          "possibility": "Possibly",
          "diagnosis": "Raw ptr is used at crates/ws-lockbud-checks-this1/src/main.rs:15:28: 15:32 (#37) after dropped at crates/ws-lockbud-checks-this1/src/main.rs:10:37: 
10:38 (#0)",
          "explanation": "Raw ptr is used or escapes the current function after the pointed value is dropped"
        }
      },
      {
        "UseAfterFree": {
          "bug_kind": "UseAfterFree",
          "possibility": "Possibly",
          "diagnosis": "Raw ptr is used at crates/ws-lockbud-checks-this1/src/main.rs:14:13: 14:16 (#0) after dropped at crates/ws-lockbud-checks-this1/src/main.rs:10:37: 1
0:38 (#0)",
          "explanation": "Raw ptr is used or escapes the current function after the pointed value is dropped"
        }
      },
      {
        "UseAfterFree": {
          "bug_kind": "UseAfterFree",
          "possibility": "Possibly",
          "diagnosis": "Escape to Global: Raw ptr Place(_51) at crates/ws-lockbud-checks-this1/src/main.rs:96:24: 96:64 (#0) escapes to ConstantDeref(Val(Scalar(alloc1), *m
ut escape_to_global::hostent)) but pointee is dropped at crates/ws-lockbud-checks-this1/src/main.rs:102:5: 102:6 (#0)",
          "explanation": "Raw ptr is used or escapes the current function after the pointed value is dropped"
        }
      }
    ]
[2024-10-25T15:11:36Z WARN  lockbud::callbacks] crate ws_lockbud_checks_this1 contains bugs: { probably: 0, possibly: 0 }, conflictlock: { probably: 0, possibly: 0 }, condv
ar_deadlock: { probably: 0, possibly: 0 }, atomicity_violation: { possibly: 0 }, invalid_free: { possibly: 0 }, use_after_free: { possibly: 7 }
warning: `ws-lockbud-checks-this1` (bin "ws-lockbud-checks-this1") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.57s
```

