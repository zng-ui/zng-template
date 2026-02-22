use zng::{prelude::*, window::WindowButton};

pub async fn window(args: zng::app::crash_handler::CrashArgs) -> window::WindowRoot {
    // l10n-## Crash Dialog

    Window! {
        title = l10n!(
            "crash_dialog/title",
            "{$app} - Error",
            app = zng::env::about().app.clone()
        );
        icon = shared::res::ICON_SMALL;
        save_state = false;
        padding = 10;
        enabled_buttons = WindowButton::CLOSE | WindowButton::MINIMIZE;
        size = (400, 150);
        child = Container! {
            // error icon to the left (or right if RTL)
            child_start = Wgt! {
                widget::background = ICONS.get("error");
                layout::size = 40;
                layout::align = Align::TOP;
                layout::margin = 10;
            };
            // error message and link to show debug dialog
            child = Stack! {
                direction = StackDirection::top_to_bottom();
                spacing = 5;
                children = ui_vec![
                    Text!(args.latest().message()),
                    #[cfg(feature = "dev")]
                    Button! {
                        style_fn = zng::button::LinkStyle!();
                        child = Text!(l10n!(
                            "crash_dialog/debug-dialog",
                            "Show debug crash dialog"
                        ));
                        on_click = hn_once!(args, |_| {
                            WINDOWS.open("crash-dialog-dbg", async move {
                                zng::app::crash_handler::debug_dialog(args)
                            });
                            WINDOW.close();
                        });
                    }
                ];
            };
        };
        // control buttons, [Restart] [Exit]
        child_spacing = 5;
        child_bottom = Wrap! {
            layout::align = Align::END;
            children = ui_vec![
                Button! {
                    child = Text!(l10n!("crash_dialog/restart", "Restart"));
                    on_click = hn_once!(args, |_| {
                        args.restart();
                    });
                },
                Button! {
                    child = Text!(l10n!("crash_dialog/exit", "Exit"));
                    on_click = hn_once!(args, |_| {
                        args.exit(0);
                    });
                }
            ];
        };
    }
}
