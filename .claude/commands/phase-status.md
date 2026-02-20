Show the current status of all 9 VPy compiler phases.

Read `buildtools/README.md` to get the latest status, then run `cargo test --all` in the `buildtools/` directory to get live test counts per crate.

Output a status table like:

| Phase | Crate              | Status      | Tests |
|-------|--------------------|-------------|-------|
| 1     | vpy_loader         | ✅ Complete | 5/5   |
| 2     | vpy_parser         | ✅ Complete | 52/52 |
| ...   | ...                | ...         | ...   |

Highlight any phases that are in-progress or have failing tests. Note what work remains for incomplete phases.
