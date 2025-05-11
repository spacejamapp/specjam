Archived, while PolkaJam provides its own testing framework, this repo is
no longer needed and will be embedded into the Spacejam repo.

# Specjam

jam-test-vectors rust wrapper.

For avoiding the memory overload, we now wraps the jam-test-vectors repo and
generates the test vectors on demand, all tests are in the same interface.

```rust
pub struct Test {
  input: String,
  output: String,
  scale: Option<Scale>,
  section: Section,
  name: String,
}
```

For using it in your project,

```rust
use specjam::Registry;

fn my_test() {
  for test in Registry("jamtestvectors").accumulate().iter() {
    println!("{}", test.name);
  }
}

```

# LICENSE

GPL-3.0
