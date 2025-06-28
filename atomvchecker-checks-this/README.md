This test is checked at AtomVChecker's [bec82556b](https://github.com/os-checker/AtomicVChecker/commit/bec82556be9af937a6b3d30ddcc277f7a89f094f) commit.

```rust
$ rustup run nightly-2023-03-09 cargo atomvchecker
warning: atomvchecker-checks-this v0.1.0 (/home/gh-zjp-CN/os-checker-test-suite/atomvchecker-checks-this) ignoring invalid dependency `scope` 
which is missing a lib target
   Compiling memchr v2.5.0
   Compiling libc v0.2.139
   Compiling cc v1.0.78
   Compiling gimli v0.27.1
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
   Compiling cfg-if v1.0.0
   Compiling byteorder v1.4.3
   Compiling crossbeam v0.2.12
   Compiling miniz_oxide v0.6.2
   Compiling object v0.30.3
   Compiling backtrace v0.3.67
   Compiling addr2line v0.19.0
   Compiling error-chain v0.10.0
   Compiling mem v0.5.0
   Compiling atomvchecker-checks-this v0.1.0 (/home/gh-zjp-CN/os-checker-test-suite/atomvchecker-checks-this)
warning: use of deprecated function `std::mem::uninitialized`: use `mem::MaybeUninit` instead
  --> src/main.rs:66:50
   |
66 |             payload: Payload::Data(unsafe { mem::uninitialized() }),
   |                                                  ^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

[2025-06-28T00:20:13Z WARN  atomvchecker::callbacks] [
      {
        "AtomicCorrelationViolation": {
          "bug_kind": "AtomicCorrelationViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "atomic": "src/main.rs:298:41: 298:54"
          },
          "explanation": "Using an atomic operation with a weaker memory ordering than necessary can lead to an inconsistent memory state. Usi
ng Acquire is sufficient to ensure the program's correctness."
        }
      },
      {
        "AtomicCorrelationViolation": {
          "bug_kind": "AtomicCorrelationViolation",
          "possibility": "Possibly",
          "diagnosis": {
            "atomic": "src/main.rs:177:45: 177:65"
          },
          "explanation": "Using an atomic operation with a weaker memory ordering than necessary can lead to an inconsistent memory state. Usi
ng Release is sufficient to ensure the program's correctness."
        }
      }
    ]
[2025-06-28T00:20:13Z WARN  atomvchecker::callbacks] crate atomvchecker_checks_this contains atomic_correlation_violation: { possibly: 2 }
```
