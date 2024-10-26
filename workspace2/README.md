# RAP

```bash
export RAP_LOG=warn

# os-checker-test-suite/workspace2
# cargo rap -F -M -- -p # list Possible packages/workspace members
cargo rap -F -M -- -p ws-rap-checks-this1
cargo rap -F -M -- -p ws-rap-checks-this2
cargo rap -F -M -- --workspace

# check with feature selection
cargo rap -F -M -- -p ws-rap-checks-this_with-feature-gated -F rap

# check packages (build.rs is not detected by rap, but directly handled by rustc)
cargo rap -F -M -- -p ws-rap-checks-this_with-build-script

# check tests & examples
cargo rap -F -M -- -p ws-rap-checks-test-and-example --examples --tests
cd crates/ws-rap-checks-test-and-example 
cargo rap -F -M -- --example ex1
cargo rap -F -M -- --examples
cargo rap -F -M -- --tests
cargo rap -F -M -- --examples --tests
```
