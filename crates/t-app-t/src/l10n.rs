use zng::l10n::*;

/// Initialize localization in the app context.
pub fn app_init() {
    L10N.load_dir(&shared::env::args().lang_dir);
}
