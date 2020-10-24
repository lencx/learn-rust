# Controlling how tests are run

```bash
# #[test]
# #[ignore]

# If you don’t want to run the tests in parallel
# or if you want more fine-grained control over the number of threads used
# We set the number of test threads to 1, telling the program not to use any parallelism.
cargo test -- --test-threads=1

# disable the output capture behavior
cargo test -- --nocapture

# Filtering to Run Multiple Tests
# note that the module in which a test appears becomes part of the test’s name,
# so we can run all the tests in a module by filtering on the module’s name.
cargo test <fn_part_name>

# run only the ignored tests
cargo test -- --ignored
```
