use zng::{config::settings::*, prelude::*};

// called by t-app-t/main.rs
pub fn init() {
    SETTINGS.register_categories(|b| {
        lang::register_category(b);
    });

    SETTINGS.register(|b| {
        lang::register(b);
    });
}

// l10n-## Lang
mod lang {
    use super::*;
    use shared::config::lang::{CONFIG_KEY, SYSTEM_LANG};
    use widget::node::presenter;
    use zng::l10n::Lang;

    pub const CATEGORY_ID: &str = "lang";

    pub(super) fn register_category(b: &mut CategoriesBuilder) {
        b.entry(CATEGORY_ID, |b| {
            // l10n-# Label of the category lang
            b.name(l10n!("settings/category-lang", "Lang"))
        });
    }

    pub(super) fn register(b: &mut SettingsBuilder) {
        b.entry(CONFIG_KEY, CATEGORY_ID, |b| {
            // l10n-# Label of the field lang
            b.name(l10n!("settings/lang.name", "Lang"));
            b.description(l10n!("settings/lang.description", "App text language"));
            b.editor_fn(WidgetFn::new(editor));
            b.reset(shared::env::settings_resetter(), "settings.")
        });
    }

    /// Langs that have local translations.
    fn available_langs() -> impl Var<Vec<Lang>> {
        L10N.available_langs().map(|m| {
            let mut r = vec![];
            for (lang, files) in m.iter() {
                if cfg!(feature = "release")
                    && ["template", "pseudo"].contains(&lang.language.as_str())
                {
                    // exclude test langs from release builds.
                    continue;
                }
                if files.keys().any(|f| f.is_current_app()) {
                    // only include langs with app translations, the imported dependency localizations
                    // can support more languages, but those are not fully covered.
                    r.push(lang.clone());
                }
            }
            r.sort();
            r.push(SYSTEM_LANG);
            r
        })
    }

    fn editor(_: Setting) -> impl UiNode {
        let selected = CONFIG.get(CONFIG_KEY, SYSTEM_LANG);
        // combo box
        Toggle! {
            style_fn = toggle::ComboStyle!();
            child = lang_text(selected.clone());
            padding = 4;
            checked_popup = wgt_fn!(|_| {
                // drop down, presents available_langs dynamically, will probably not change with it open tho
                popup::Popup!(presenter(
                    available_langs(),
                    wgt_fn!(selected, |langs: Vec<Lang>| {
                        Stack! {
                            toggle::selector = toggle::Selector::single(selected.clone());

                            direction = StackDirection::top_to_bottom();
                            children = langs
                                .into_iter()
                                .map(|l| {
                                    // drop down item
                                    Toggle! {
                                        child = lang_text(l.clone());
                                        child_align = Align::START;
                                        value::<Lang> = l;
                                    }
                                })
                                .collect::<UiVec>()
                        }
                    })
                ))
            });
        }
    }

    fn lang_text(lang: impl IntoVar<Lang>) -> impl UiNode {
        let lang = lang.into_var();
        Text! {
            txt = lang.map_to_txt();
            when *#{lang} == SYSTEM_LANG {
                txt = l10n!("settings/system-lang", "<system language>");
                font_style = FontStyle::Italic;
            }
        }
    }
}
