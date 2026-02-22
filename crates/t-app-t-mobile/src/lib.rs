#[cfg(target_os = "android")]
mod android {
    use zng::view_process::default::*;

    // Android entry point.
    #[unsafe(no_mangle)]
    fn android_main(app: android::AndroidApp) {
        zng::env::init!();

        // writes to Logcat
        zng::app::print_tracing(tracing::Level::INFO);

        // set Android app instance and paths
        android::init_android_app(app.clone());
        // install assets packed by `.zr-apk` from the 'pack/apk/assets/res' content
        zng::env::android_install_res(|| app.asset_manager().open(c"res.tar"));

        run_same_process(super::app);
    }
}

#[cfg(target_os = "ios")]
mod ios {
    // iOS is not supported on this release of Zng, it will be in the future.
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn app() {
    use zng::prelude::*;

    // mobile has no CLI, init args with defaults.
    shared::env::init_args(shared::env::TtAppTtArgs {
        paths: vec![],
        log_dir: None,
        no_view_process: true,
        no_crash_handler: true,
        lang: Default::default(),
        lang_dir: zng::env::res("l10n"),
    });

    // start app scope, with default extensions.
    let app = APP.defaults();
    // add extensions here
    //let app = app.extend(MyExt::default());

    // run and open main window
    app.run_window("main", async {
        // register bundled licenses, used by the default `OPEN_LICENSES_CMD` screen.
        #[cfg(feature = "release")]
        zng::third_party::LICENSES.register(shared::res::licenses);

        // load Fluent localization files and set initial lang.
        shared::l10n::app_init();

        // load user config files.
        shared::config::app_init();
        // register settings metadata providers.
        gui::settings::init();

        gui::primary::window().await
    })
}
