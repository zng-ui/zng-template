use tools_util::*;

/// Get release langs
pub fn release_langs() -> Vec<String> {
    cmd("cargo", &["zng", "l10n", "--release-langs", "res/l10n"])
        .output()
        .success_or_die("cargo zng l10n --release-langs error")
        .split(',')
        .map(|s| s.to_owned())
        .collect()
}
