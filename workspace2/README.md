# RAP

```bash
# os-checker-test-suite/workspace2
# cargo rap -F -M -- -p # list Possible packages/workspace members
cargo rap -F -M -- -p ws-rap-checks-this1
cargo rap -F -M -- -p ws-rap-checks-this2
cargo rap -F -M -- --workspace

# check with feature selection
cargo rap -F -M -- -p ws-rap-checks-this_with-feature-gated -F rap

# check tests & examples
cd crates/ws-rap-checks-test-and-example 
cargo rap -F -M -- --example ex1
cargo rap -F -M -- --examples
cargo rap -F -M -- --tests
cargo rap -F -M -- --examples --tests
```
