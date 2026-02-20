Compile a VPy project using the buildtools CLI.

Usage: /compile-vpy [path-to-project-or-vpy-file]

If $ARGUMENTS is provided, use it as the target. Otherwise, look for `.vpyproj` files in the `examples/` directory and ask which one to compile.

Run the compiler:
```bash
cargo run --bin vpy_cli -- build <target>
```

From the `buildtools/` directory. Report:
- Any compilation errors with file and line number
- Output binary size and location
- Whether it's a single-bank or multi-bank ROM

If compilation fails, investigate the relevant compiler phase based on the error message.
