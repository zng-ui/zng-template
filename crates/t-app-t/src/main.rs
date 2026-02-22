// Don't show console window in Windows release builds started from the Windows Explorer.
#![cfg_attr(feature = "release", windows_subsystem = "windows")]

mod cli;
mod crash;
mod log;

use zng::prelude::*;

use zng::view_process::prebuilt as view_process;

fn main() {
    // init `zng::env`, `shared::env`
    // if requested, run as other processes and exit (cli, view, workers)
    zng::env::init!();

    // run app.
    if shared::env::args().no_view_process {
        view_process::run_same_process(app_process);
    } else {
        app_process();
    }
}

fn app_process() {
    // start app scope, with default extensions.
    let app = APP.defaults();
    // add extensions here
    //let app = app.extend(MyExt::default());

    // run and open main window
    app.run_window("main", async {
        // if you use "single_instance" for the app, hook event here.
        // zng::app::APP_INSTANCE_EVENT.on_pre_event(..);

        // register bundled licenses, used by the default `OPEN_LICENSES_CMD` screen.
        #[cfg(feature = "release")]
        zng::third_party::LICENSES.register(shared::res::licenses);

        // load/watch Fluent localization files and set initial lang.
        shared::l10n::app_init();

        // load/watch user config files.
        shared::config::app_init();
        // register settings metadata providers.
        gui::settings::init();

        gui::primary::window().await
    })
}
