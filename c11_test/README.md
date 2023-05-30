
```bash

#Run all test functions in module tests (src/lib.rs) but test functions in main.rs won't be run.
cargo test

# List all tests and benchmarks
cargo test -- --list

# Run tests with one thread (one test at a time, rather than in parallel) and 
#   show stdout of sucessful tests
cargo test -- --test-threads 1  --show-output

# Run this test only
cargo test greater_than_100

# Run all tests whose names match "two"
cargo test two

# Run all tests, including ignored ones
cargo test -- --include-ignored

# Run ignored-only tests
cargo test -- --ignored

# Run a test function in main.rs
cargo test two_plus_two_in_main

# Run all tests in the main binary
cargo test --bin c11_test

# Run all tests in binaries
cargo test --bins

# Run a test function in tests/integration_test.rs
cargo test integration_adds_two

# Run all tests in tests/integration_test.rs
cargo test --test integration_test
```