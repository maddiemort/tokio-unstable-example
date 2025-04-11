# `tokio_unstable` Example

The [`tokio`][tokio] crate uses a combination of a Cargo feature flag (`testing`) and a bare `--cfg`
flag (`tokio_unstable`) to conditionally compile some code. Users of the crate must enable both
flags in order to use (for example) `tokio::task::Builder` - but bare `--cfg` flags, unlike Cargo
feature flags, can only be enabled by the end user of the dependency tree (the person running the
`cargo build` command), not by intermediate dependencies. As far as I understand, this is the reason
that a bare `--cfg` flag was chosen for this use-case.

The end user therefore has to set either the `RUSTFLAGS` env var or the `build.rustflags` Cargo
configuration option to include `--cfg tokio_unstable`.

Unfortunately, Cargo does not pass `RUSTFLAGS`/`build.rustflags` to build scripts or proc macros
when it is passed the `--target` flag (this is [intended behaviour][rustflags]). That means that, if
a crate that uses features from `tokio` that are gated behind `tokio_unstable` appears in the
`[build-dependencies]` of any crate, or the `[dependencies]` of any proc macro crate, it will be
impossible to build it.

The only option that appears to be available to work around this is to point the
`build.rustc-wrapper` option at a script that will pass the `--cfg tokio_unstable` flag to all
invocations of `rustc`. Unfortunately, `build.rustc-wrapper` has no per-platform override options,
so if your crate is intended to be compiled from both \*nix platforms and Windows, whatever script
or binary you point that option at will have to be cross-platform. That turns out in practice to be
nearly impossible, or at least such a headache that it's not worth the effort.

## Reproducing the Issue

This repository contains a workspace with two crates in it - `main-crate` and `task-spawner`.
`task_spawner::spawn_task()` uses `tokio::task::Builder` to spawn a task, and that function is
called both from `main-crate/src/main.rs` and from `main-crate/build.rs`. Since `task-spawner` uses
`tokio::task::Builder`, it needs `tokio` to be compiled with `--cfg tokio_unstable` - in this repo,
that's achieved by setting `build.rustflags` in `.cargo/config.toml`.

Normal `cargo build` and `cargo run` commands work perfectly fine, because that `build.rustflags`
setting is propagated to all dependencies (normal and build deps). But, if you instead run `cargo
build --target <target-triple>`, where `<target-triple>` can be any supported target triple
(including the target triple of the build host), `task-spawner` will fail to compile. This is
because the copy of `task-spawner` that's a _build dependency_ of `main-crate` will now be compiled
without `--cfg tokio_unstable`.

[tokio]: https://github.com/tokio-rs/tokio
[rustflags]: https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags
