```
src            # crate

src/bar        # self  = bar
               # crate::bar

src/bar/baz.rs # super = bar
               # crate::bar::baz

src/bar/mod.rs # self  = bar
               # crate::bar

src/foo        # super = crate
               # crate::foo

src/foo/one.rs # super = foo
               # crate::foo::one

src/foo/two.rs # crate::foo::two

src/foo.rs     # self  = foo
               # crate::foo

src/main.rs    # crate (also entry point)
```

`crate::` is where `main.rs` or `lib.rs` is located (configured via `Cargo.toml`)

`foo` is a module, with Rust 2018 having foo.rs next to a directory with the same name treats it as a module

`bar` is a module, with Rust 2015 directories require a mod.rs in them to denote this
