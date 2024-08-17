# Crates

{{app}} project crates.

## `t-app-t`

Main entry, builds the t-app-t executable binary for Linux, macOS and Windows. Code is mostly startup config and CLI. Also
builds resources that are embedded on the executable file metadata on Windows.

## `t-app-t-mobile`

Main entry for Android, builds the t-app-t binary for the app package.

## `gui` 

GUI library, statically linked, but can build as cdylib for hot-reload.

The app "view" is implemented here.

## `shared`

Utils library, embedded resources.