

```rust
workspace2/crates/ws-rap-checks-this_with-build-script $ cargo rap -F -M
17:31:13|RAP|INFO|: Start cargo-rap
   Compiling ws-rap-checks-this_with-build-script v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this_with-build-script)
17:31:13|RAP|INFO|: Execute after_analysis() of compiler callbacks
17:31:13|RAP|INFO|: analysis done
error: linking with `cc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/self-contained:/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/self-contained:/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/self-contained:/run/user/0/fnm_multishells/101146_1729837306085/bin:/root/.local/share/fnm:/usr/local/bin/nvim-linux64/bin/:/run/user/0/fnm_multishells/5290_1729515234205/bin:/root/.local/share/fnm:/root/.cargo/bin:/usr/local/bin/nvim-linux64/bin/:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" VSLANG="1033" "cc" "-m64" "/tmp/rustcUrSjx0/symbols.o" "/rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.2rluy79vwiqugnslj6xnjxwgc.rcgu.o" "/rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.8htyibkdqqjv42zcdb1na1o6y.rcgu.o" "/rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.96vq9w9vi9ttrgufai5vbh0fd.rcgu.o" "/rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.a1kzgwz9v5g70678i5gy4gy6m.rcgu.o" "/rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.ad6tzvidr3ebt0aosayteyfma.rcgu.o" "/rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.apgr9soeas6ilwbkf9gyys3kd.rcgu.o" "/rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.581v2fx64zv8v7zopclxza1zi.rcgu.o" "-Wl,--as-needed" "-L" "/rust/my/os-checker-test-suite/workspace2/target/debug/deps" "-L" "/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "" "" "" "" "" "" "" "" "" "" "" "" "" "" "" "" "" "" "" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-B/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/gcc-ld" "-B/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/gcc-ld" "-B/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/gcc-ld" "-fuse-ld=lld" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/root/.rustup/toolchains/nightly-2024-06-30-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/self-contained" "-o" "/rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"
  = note: rust-lld: error: undefined symbol: rust_eh_personality
          >>> referenced by a1kzgwz9v5g70678i5gy4gy6m
          >>>               /rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.a1kzgwz9v5g70678i5gy4gy6m.rcgu.o:(DW.ref.rust_eh_personality)

          rust-lld: error: undefined symbol: std::rt::lang_start_internal::h2f4251ff225845a5
          >>> referenced by 96vq9w9vi9ttrgufai5vbh0fd
          >>>               /rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.96vq9w9vi9ttrgufai5vbh0fd.rcgu.o:(std::rt::lang_start::h4d43b1925293ebbe)

          rust-lld: error: undefined symbol: std::io::stdio::_print::h2ff37b8000868228
          >>> referenced by 2rluy79vwiqugnslj6xnjxwgc
          >>>               /rust/my/os-checker-test-suite/workspace2/target/debug/build/ws-rap-checks-this_with-build-script-70010feb86d751a0/build_script_build-70010feb86d751a0.2rluy79vwiqugnslj6xnjxwgc.rcgu.o:(build_script_build::main::h90c94d129785e7f6)
          collect2: error: ld returned 1 exit status


error: could not compile `ws-rap-checks-this_with-build-script` (build script) due to 1 previous error
17:31:14|RAP|ERROR|: Finished with non-zero exit code.

```

```rust
workspace2/crates/ws-rap-checks-this_with-build-script $ rm build.rs
workspace2/crates/ws-rap-checks-this_with-build-script $ cargo rap -F -M
17:33:11|RAP|INFO|: Start cargo-rap
    Checking ws-rap-checks-this_with-build-script v0.1.0 (/rust/my/os-checker-test-suite/workspace2/crates/ws-rap-checks-this_with-build-script)
17:33:11|RAP|INFO|: Execute after_analysis() of compiler callbacks
17:33:11|RAP|INFO|: analysis done
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
```
