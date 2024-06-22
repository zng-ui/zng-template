use zng::prelude::*;

pub async fn window() -> window::WindowRoot {
    // l10n-primary_win-### Primary Window

    Window! {
        id = "primary-window";
        title = zng::env::about().app.clone();
        icon = shared::res::ICON_SMALL;

        child_top = menu(), 0;
        child = content();
    }
}

// #[zng::hot_reload::hot_node]
fn menu() -> impl UiNode {
    Menu!(ui_vec![SubMenu!(
        l10n!("primary/menu-about", "About"),
        ui_vec![
            #[cfg(feature = "dev")]
            Button!(zng::window::cmd::INSPECT_CMD.scoped(WINDOW.id())),
            Button!(zng::third_party::OPEN_LICENSES_CMD),
        ],
    ),])
}

// #[zng::hot_reload::hot_node]
fn content() -> impl UiNode {
    Stack! {
        layout::align = Align::CENTER;
        direction = StackDirection::top_to_bottom();
        children = ui_vec![
            Image!(shared::res::ICON_MEDIUM),
            Text! {
                txt = l10n!("primary_win/greetings", "Hello {{app}}!");
                font_size = 2.em();
                txt_align = Align::CENTER;
            },
        ]
    }
}
