# Distribution Packages

Call `cargo do pack [PACKAGE]` to compile packages.

Call `cargo zng res --tools` to get a list of `.zr-*` tools. 
Call `cargo zng res --tool <tool>` to get help for a tool.

## Packages

**common**

Not a full package, just groups bin and res in an easy place for other packages to reference.

**portable**

Portable app folder, app should be executable from a flash drive (on the target OS). Used by `cargo do run-release`.

**portable-tar**

Compress the `portable` package to a .tar.gz file.

**deb**

Debian package, compatible with many Unix operating systems.

**macos**

Builds a macOS .app folder and a .dmg image file from it.

**windows**

Builds an InnoSetup installer.

**android**

Builds an Android APK. 

Note that `apk/res/` is for Android specific resources, Zng resources are placed in `assets/res/`.