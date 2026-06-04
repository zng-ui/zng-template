fn main() {
    #[cfg(feature = "release")]
    release::build();
}

#[cfg(feature = "release")]
mod release {
    pub fn build() {
        #[cfg(windows)]
        windows::build();
    }

    #[cfg(windows)]
    mod windows {
        pub fn build() {
            // Set windows .exe metadata, icon.
            let mut res = winresource::WindowsResource::new();
            res.set_icon("../../res/icon/windows.ico");
            res.compile().unwrap();

            // Build CLI proxy
            zng_env::windows_subsystem::build_cli_com_proxy("t-app-t.exe", None).unwrap();
        }
    }
}
