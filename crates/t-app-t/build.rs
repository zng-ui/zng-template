fn main() {
    // Set windows .exe metadata, icon.
    #[cfg(all(windows, feature = "release"))]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../res/icon/windows.ico");
        res.compile().unwrap();
    }
}
