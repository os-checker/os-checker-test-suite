# SARIF support

See [sarif.yaml](https://github.com/os-checker/os-checker-test-suite/blob/5b61f76fdf7ca5c874bc3a2620ebe87bc0f2f9d9/.github/workflows/sarif.yaml)
and the [result](https://github.com/os-checker/os-checker-test-suite/security/code-scanning).

# Rudra Output

Logs are printed to stdout, while analysis results are to stderr.

```rust
os-checker-test-suite/rudra-checks-this $ cargo +nightly-2021-10-21 rudra
2024-11-01 20:01:53.531200 |INFO | [rudra-progress] Running cargo rudra
2024-11-01 20:01:54.058017 |INFO | [rudra-progress] Running rudra for target lib:rudra-checks-this
warning: function is never used: `test_order_unsafe`
  --> rudra-checks-this/src/order_unsafe.rs:10:4
   |
10 | fn test_order_unsafe<I: Iterator<Item = impl Debug>>(mut iter: I) {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: struct is never constructed: `MyVec`
 --> rudra-checks-this/src/vec_push_all.rs:8:12
  |
8 | pub struct MyVec<T>(Vec<T>);
  |            ^^^^^

warning: associated function is never used: `push_all`
  --> rudra-checks-this/src/vec_push_all.rs:12:8
   |
12 |     fn push_all(&mut self, to_push: &[T]) {
   |        ^^^^^^^^

2024-11-01 20:01:54.923184 |INFO | [rudra-progress] Rudra started
2024-11-01 20:01:54.923337 |INFO | [rudra-progress] SendSyncVariance analysis started
2024-11-01 20:01:54.923434 |INFO | [rudra-progress] SendSyncVariance analysis finished
2024-11-01 20:01:54.923440 |INFO | [rudra-progress] UnsafeDataflow analysis started
2024-11-01 20:01:54.943918 |INFO | [rudra-progress] UnsafeDataflow analysis finished
2024-11-01 20:01:54.943954 |INFO | [rudra-progress] Rudra finished
Error (SendSyncVariance:/PhantomSendForSend/NaiveSendForSend/RelaxSend): Suspicious impl of `Send` found
-> rudra-checks-this/src/wild_send.rs:9:1: 9:40
unsafe impl<P: Ord> Send for Atom<P> {}
Warning (UnsafeDataflow:/ReadFlow): Potential unsafe dataflow issue in `order_unsafe::test_order_unsafe`
-> rudra-checks-this/src/order_unsafe.rs:10:1: 15:2
fn test_order_unsafe<I: Iterator<Item = impl Debug>>(mut iter: I) {
    unsafe {
        std::ptr::read(&Box::new(1234) as *const _);
    }
    println!("{:?}", iter.next());
}

Error (UnsafeDataflow:/WriteFlow/VecSetLen): Potential unsafe dataflow issue in `vec_push_all::MyVec::<T>::push_all`
-> rudra-checks-this/src/vec_push_all.rs:12:5: 23:6
fn push_all(&mut self, to_push: &[T]) {
        self.0.reserve(to_push.len());
        unsafe {
            // can't overflow because we just reserved this
            self.0.set_len(self.0.len() + to_push.len());

            for (i, x) in to_push.iter().enumerate() {
                // Clone might panic
                self.0.as_mut_ptr().offset(i as isize).write(x.clone());
            }
        }
    }

2024-11-01 20:01:55.043410 |INFO | [rudra-progress] cargo rudra finished
```
