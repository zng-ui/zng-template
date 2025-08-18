use zng::prelude::*;

pub async fn window(args: zng::app::crash_handler::CrashArgs) -> window::WindowRoot {
    // l10n-## Crash Dialog

    Window! {
        title = l10n!(
            "crash_dialog/title",
            "{$app} - Error",
            app = zng::env::about().app.clone()
        );
        icon = shared::res::ICON_SMALL;
        child = Container! {
            // error icon to the left (or right if RTL)
            child_start =
                Wgt! {
                    widget::background = ICONS.get("error");
                    layout::size = 40;
                    layout::align = Align::TOP;
                    layout::margin = 10;
                },
                0,
            ;
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
                            WINDOWS.open(async move { zng::app::crash_handler::debug_dialog(args) });
                            WINDOW.close();
                        });
                    }
                ];
            };
        };
        // control buttons, [Restart] [Exit]
        child_bottom = {
            spacing: 5,
            node: Wrap! {
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
            },
        };

        // bring to foreground
        on_load = hn_once!(|_| {
            let _ = WINDOWS.focus(WINDOW.id());
        });
    }
}
