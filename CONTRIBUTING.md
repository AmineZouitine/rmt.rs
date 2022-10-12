# Contribution guidelines


## Fork/clone/pull

The typical workflow for contributing to `rmt.rs` is:

1. Fork the `main` branch from the [GitHub repository](https://github.com/AmineZouitine/rmt.rs).
2. Clone your fork locally.
3. Commit changes.
4. Push the changes to your fork.
5. Send a pull request from your fork back to the original `main` branch.
## Discord
https://discord.gg/mJR59KNX
## Run test
We are testing on the same database so we should run a test on a single thread.
```sh
cargo test -- --test-threads=1
```

