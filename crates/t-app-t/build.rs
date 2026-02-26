fn main() {
    #[cfg(all(windows, feature = "release"))]
    {
        // Set windows .exe metadata, icon.
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../res/icon/windows.ico");
        res.compile().unwrap();

        // Build CLI proxy
        zng_env::windows_subsystem::build_cli_com_proxy("t-app-t.exe", None).unwrap();
    }
}
