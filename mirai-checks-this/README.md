# MIRAI 的最小示例

rustc 能检测到常量中的溢出算数，但检测不到函数调用后的溢出；而 mirai 能检测到这两者。

```rust
// 来自 rustc：
 error: this arithmetic operation will overflow
  --> mirai-checks-this/src/main.rs:3:13
   |
 3 |     let a = u8::MAX + 1;
   |             ^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
   |
   = note: `#[deny(arithmetic_overflow)]` on by default

 // 来自 mirai：
 warning: attempt to add with overflow
  --> mirai-checks-this/src/main.rs:3:13
   |
 3 |     let a = u8::MAX + 1;
   |             ^^^^^^^^^^^
```

```rust
fn b() -> u8 { 1 }

let a = u8::MAX;
// rustc 对此无诊断，但 mirai 会报告溢出 :）
dbg!(a + b());


$ cargo +nightly-2023-12-30 mirai
warning: attempt to add with overflow
 --> mirai-checks-this/src/main.rs:3:10
  |
3 |     dbg!(a + b());
  |          ^^^^^^^
```

# `--statistics` 的问题

<details>

<summary>这部分不重要，结论是不使用 --statistics 参数</summary>

* `--diag` 与 `--statistics` 一起使用时，mirai 只会考虑 --statistics
* 直接第二次运行，会因为增量构建而导致无诊断

```rust
$ MIRAI_FLAGS="--diag=paranoid --statistics" cargo +nightly-2023-12-30 mirai
    Checking mirai-checks-this v0.1.0 (/rust/tmp/os-checker/os-checker-test-suite/mirai-checks-this)
mirai-checks-this/src/main.rs, analyzed, 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s

$ MIRAI_FLAGS="--diag=paranoid --statistics" cargo +nightly-2023-12-30 mirai
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s

$ MIRAI_FLAGS="--diag=paranoid" cargo +nightly-2023-12-30 mirai
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
```

注意：`--diag` 会随 Cargo 的缓存而在后续发出相同的诊断，不过一旦加入 `--statistics` 又开始无诊断了。

```rust
$ MIRAI_FLAGS="--diag=paranoid" cargo +nightly-2023-12-30 mirai
    Checking mirai-checks-this v0.1.0 (/rust/tmp/os-checker/os-checker-test-suite/mirai-checks-this)
warning: attempt to add with overflow
  --> mirai-checks-this/src/main.rs:26:10
   |
26 |     dbg!(a + b());
   |          ^^^^^^^

warning: `mirai-checks-this` (bin "mirai-checks-this") generated 1 warning

$ MIRAI_FLAGS="--diag=paranoid" cargo +nightly-2023-12-30 mirai
warning: attempt to add with overflow
  --> mirai-checks-this/src/main.rs:26:10
   |
26 |     dbg!(a + b());
   |          ^^^^^^^

warning: `mirai-checks-this` (bin "mirai-checks-this") generated 1 warning

$ MIRAI_FLAGS="--diag=paranoid" cargo +nightly-2023-12-30 mirai
warning: attempt to add with overflow
  --> mirai-checks-this/src/main.rs:26:10
   |
26 |     dbg!(a + b());
   |          ^^^^^^^
warning: `mirai-checks-this` (bin "mirai-checks-this") generated 1 warning

$ MIRAI_FLAGS="--statistics --diag=paranoid" cargo +nightly-2023-12-30 mirai
    Checking mirai-checks-this v0.1.0 (/rust/tmp/os-checker/os-checker-test-suite/mirai-checks-this)
mirai-checks-this/src/main.rs, analyzed, 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s

$ MIRAI_FLAGS="--diag=paranoid" cargo +nightly-2023-12-30 mirai
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
```

因此 os-checker 无法依赖 `--statistics` 报告的数量（因为重新构建需要更多的时间）。

</details>

# 好消息：获取 MIRAI 的诊断输出并不难

幸好它具有类似 cargo 的诊断输出，而且具有 cargo 的 JSON 格式：

```rust
$ MIRAI_FLAGS="--diag=paranoid" cargo +nightly-2023-12-30 mirai --message-format=json
{"reason":"compiler-message","package_id":"mirai-checks-this 0.1.0 (path+file:///rust/tmp/os-checker/os-checker-test-suite/mirai-checks-this)","
manifest_path":"/rust/tmp/os-checker/os-checker-test-suite/mirai-checks-this/Cargo.toml","target":{"kind":["bin"],"crate_types":["bin"],"name":"
mirai-checks-this","src_path":"/rust/tmp/os-checker/os-checker-test-suite/mirai-checks-this/src/main.rs","edition":"2021","doc":true,"doctest":f
alse,"test":true},"message":{"rendered":"warning: attempt to add with overflow\n  --> mirai-checks-this/src/main.rs:26:10\n   |\n26 |     dbg!(a
 + b());\n   |          ^^^^^^^\n\n","$message_type":"diagnostic","children":[],"code":null,"level":"warning","message":"attempt to add with ove
rflow","spans":[{"byte_end":833,"byte_start":826,"column_end":17,"column_start":10,"expansion":null,"file_name":"mirai-checks-this/src/main.rs",
"is_primary":true,"label":null,"line_end":26,"line_start":26,"suggested_replacement":null,"suggestion_applicability":null,"text":[{"highlight_en
d":17,"highlight_start":10,"text":"    dbg!(a + b());"}]}]}}
```

这意味着我们可以复用 os-checker 获取 clippy 输出的代码，并统计到数量。
（这么说，利用 MIR 的静态检查工具都可以具有 clippy 格式的输出样式了？那么这就是标准化呀！）

坏消息：由于输出格式相同，我们无法区分诊断来自 MIRAI 还是 rustc。

# 与编译条件交互

* 根据 [#1235](https://github.com/facebookexperimental/MIRAI/issues/1235)，传递 
  RUSTC_FLAGS 的方式为 `MIRAI_FLAGS="-- --cfg=my_cfg" cargo mirai`。
* 【已测试】可以与 features 交互：`MIRAI_FLAGS="--diag=paranoid" cargo +nightly-2023-12-30 mirai -Fzero`
* 【待测试】与 targets 交互

# `--diag` 控制检查的严格程度

对于 `kern-crates/CSpace` 仓库（随便选的；此仓库已经废弃），[CI](https://github.com/os-checker/dockerfiles/actions/runs/10827220493/job/30039886641)
在不同 `--diag` 参数下的报告数量（在清除缓存的情况下由 `--statistics` 统计）：

| --diag   | 统计数量 | 参数解释                       | 误报率 |
|----------|:--------:|--------------------------------|:------:|
| paranoid |    95    | 发出所有检查到的、可能的错误   |   高   |
| library  |    95    | 需要写明前置条件（需要标注？） |    -   |
| verify   |    81    | 有可能发出为误报的检查结果     |   中   |
| default  |    75    | 不发出可能为误报的检查结果     |   低   |
