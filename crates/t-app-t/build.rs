fn main() {
    set_windows_metadata();
}

/// Set windows package (.exe) metadata, icon.
fn set_windows_metadata() {
    #[cfg(all(windows, feature = "release"))]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../res/icon/windows.ico");
        res.compile().unwrap();
    }
}
