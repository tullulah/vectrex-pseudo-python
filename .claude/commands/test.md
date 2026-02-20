Run tests for the VPy buildtools Rust workspace.

Run `cargo test --all` from the `buildtools/` directory. If $ARGUMENTS is provided, run tests for that specific crate (e.g. `cargo test -p vpy_parser`).

After running, report:
1. Total tests: passed / failed / ignored
2. For failures: test name, which crate, and the failure reason
3. A summary of which compiler phases (1-9) have all tests green vs failing

If tests fail, investigate the relevant source files and offer to fix the issue.
