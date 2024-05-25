# Crates

{{app}} project crates.

## `t-app-t`

Main entry, builds the t-app-t executable binary. Code is mostly config and running app processes.

Also has a `build.rs` that compiles bundled resources.

## `gui` 

Graphics user interface library, statically linked in normal builds and dynamically linked in hot-reload builds.

The app "view" is implemented here.

##  `cli`

Command line interface library, statically linked.

The {{app}} CLI provides advanced startup switches and .env, not a GUI alternative.

## `shared`

Utils library, statically linked.

All other t-app-t crates depend on this crate, it is a good place to put any items that are useful across
the domains of the other crates.