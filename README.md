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

The root [`Cargo.toml`](./Cargo.toml) only declares the workspace, shared dependencies and build profiles.

Only [`crates/t-app-t/Cargo.toml`](./crates/t-app-t/Cargo.toml) declares a version, all other crates are internal with version `0.0.0-local`.

As the project grows expect the number of crates to reach dozens at least. Prefixes can be used to define "categories",
for example, as the `gui` crate grows hot-reload might slow down, you can them declare a `crates/gui-my-widget` 
for each screen or widget.

The Zng project uses this pattern successfully, we learned it from the Rust-Analyzer project, read 
[Large Rust Workspaces](https://matklad.github.io/2021/08/22/large-rust-workspaces.html) for more details.

### Startup Config

This project implements "startup config" distinct from  "user config". Startup config is required to customize
the app per installation, just compiling for each target platform is not enough, eventually some weird requirement
will show up and ideally you can just tell an advanced user to run `t-app-t.exe --something`.

The startup config is setup in [`t-app-t/src/cli.rs`](./crates/t-app-t/src/cli.rs).

### User Config

The normal user config uses the Zng `CONFIG` service and is setup in [`t-app-t/src/config.rs`](./crates/t-app-t/src/config.rs). It uses
JSON by default, but you can easily change it by using enabled a config format [`zng`] Cargo feature.

### Crash Handler

The crash handler is setup in [`t-app-t/src/crash.rs`](./crates/t-app-t/src/crash.rs). You can plug your crash reporting logic there.

### Log

Logging is done using the [`tracing`] crate and is setup in [`t-app-t/src/log.rs`](./crates/t-app-t/src/log.rs). Tracing is used
across all Zng crates because it provides structured spans that are easy to integrate with a profiler.

[`tracing`]: https://crates.io/crates/tracing

### Localization

Localization is implemented using the [Project Fluent] format. Use the `zng::l10n::l10n!` macro to declare localizable text.
Use `cargo do l10n` to scrap a localization template and test locales.

The localization files are saved in [`res/l10n`](./res/l10n/). Localization is setup in [`t-app-t/src/l10n.rs`](./crates/t-app-t/src/l10n.rs),
the `L10N` service watches the files and updates text in real time, you can use this and the app's `--lang-dir` option to setup a translation
environment that provides real time feedback as the localization files are edited.

[Project Fluent]: https://projectfluent.org/

### Licenses Bundling

Release builds of this project collect all Cargo dependency licenses and bundles them on the app. The license scrapping is done
by [`cargo-about`](https://github.com/EmbarkStudios/cargo-about), it must be installed or release builds will fail. The scrapping
is initiated in [`shared/build.rs`](./crates/shared/build.rs), you can add more non-cargo dependency licenses here too.

This is a quick way to fulfill license requirements, but you must make sure that it meeds your company **legal standards**.

### Prebuilt View-Process

This project uses the `"view_prebuilt"` Cargo feature. It downloads a prebuilt view-process library from the [zng repository]
on first build and bundles it. 

If you prefer to build it yourself change the Cargo feature to `"view"` and install all build dependencies
described in the [zng repository]. Also override the `profile.dev.package.zng-view` crate `opt-level`, the renderer has a noticeable
lower framerate in debug builds.

[`zng`]: https://github.com/zng-ui/zng/crates/zng
[zng repository]: https://github.com/zng-ui/zng

### Resources

App resources are placed in [`res/`](./res/). Some of these resources are embedded in [`shared/src/res.rs`](./crates/shared/src/res.rs).
On Windows release builds the icon is embedded in [`t-app-t/build.rs`](./crates/t-app-t/build.rs).

Other resources are bundled on the distribution packages.

### Distribution Packages

Distribution packages are implemented in [`pack/`](./pack/). Each subdirectory is structured in a format that `cargo zng res --pack` can
execute to bundle and build a package. Use the `cargo do pack [PACKAGE]` command to build release and pack.