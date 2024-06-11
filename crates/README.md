# Crates

{{app}} project crates.

## `t-app-t`

Main entry, builds the t-app-t executable binary. Code is mostly startup config, CLI. Also
builds resources that are embedded on the executable file metadata on Windows.

## `gui` 

GUI library, statically linked, but can build as cdylib for hot-reload.

The app "view" is implemented here.

## `shared`

Utils library, embedded resources.