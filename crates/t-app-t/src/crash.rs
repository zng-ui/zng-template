use zng::{app::crash_handler, prelude::*};

zng::app::crash_handler::crash_handler_config!(|cfg| {
    if shared::env::args().no_crash_handler {
        cfg.no_crash_handler();
    } else {
        cfg.dialog(crash_dialog_process);
    }
});

fn crash_dialog_process(args: crash_handler::CrashArgs) {
    if args.dialog_crash.is_some() {
        // crash dialog crashed, show a native dialog.
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_description(args.latest().message())
            .show();
    } else {
        // start app to show a custom crash dialog.
        APP.defaults().run_window(async {
            // you can start packing a crash report here
            // if let Some(_logs) = &shared::env::args().log_dir { }

            gui::crash::window(args).await
        });
    }
}
