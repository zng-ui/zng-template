# Zng Template

This repository is the default Zng template, use [`cargo-zng`] to create a new project from it:

```command
cargo zng new "App Name!"
```

[`cargo-zng`]: https://crates.io/crates/cargo-zng

## Overview

This template provides a good foundation for a GUI app that is expected to grow large. There are many ways to setup
such a project, this template makes choices for you that you might want to review.

### Flat Workspace

All crates that are part of the final binary are placed in the `crates` directory. All crates that are only
used during development are placed in the `tools` directory. 

The root `Cargo.toml` only declares the workspace, dependencies used by multiple project crates and build profiles.

Only `crates/t-app-t/Cargo.toml` declares a version, all other crates are internal with version `0.0.0`.

As the project grows expect the number of crates to reach dozens at least. Prefixes can be used to define "categories",
for example, as the `gui` crate grows large hot-reload might slow down, you might then declare a `crates/gui-my-widget` 
for each large screen or widget.

The Zng project uses this pattern successfully, we learned it from the Rust-Analyzer project, read 
[Large Rust Workspaces](https://matklad.github.io/2021/08/22/large-rust-workspaces.html) for more details.

### Startup Config

This project implements "startup config" distinct from  "user config". Startup config is required to customize
the app per installation, just compiling for each target platform is not enough, eventually some weird requirement
will show up and ideally you can just tell an advanced user to run `t-app-t.exe --something`.

The startup config is setup in [crates/t-app-t/src/cli.rs].

### User Config

The normal user config uses the Zng `CONFIG` service and is setup in [crates/t-app-t/src/config.rs]. It uses
JSON by default, but you can easily change it by using [`zng`] Cargo feature flags.

### Crash Handler

The crash handler is setup in [crates/t-app-t/src/crash.rs]. You can plug your crash reporting logic there.

### Log

Logging is done using the [`tracing`] crate and is setup in [crates/t-app-t/src/log.rs]. Tracing is used
across all Zng crates because it provides structured spans that are easy to integrate with a profiler.

[`tracing`]: https://crates.io/crates/tracing

### Localization

### Licenses Bundling

### Prebuilt View-Process

This project uses the `"view_prebuilt"` Cargo feature. It downloads a prebuilt view-process library from the [zng repository]
on build and bundles it. If you prefer to build it yourself change the Cargo feature to `"view"`, install all build dependencies
described in the [zng repository]. Also override the `profile.dev.package.zng-view` crate `opt-level`, the renderer has a noticeable
lower framerate in debug builds.

[`zng`]: https://github.com/zng-ui/zng/crates/zng
[zng repository]: https://github.com/zng-ui/zng