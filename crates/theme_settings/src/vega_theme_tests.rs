//! VEGA: checks for the bundled default "VEGA Dark" theme.

use gpui::{Hsla, Rgba};
use theme::{Appearance, Theme};

use crate::{ThemeFamilyContent, refine_theme_family};

fn bundled_theme(json: &str, name: &str) -> Theme {
    let family: ThemeFamilyContent = serde_json::from_str(json).expect("theme JSON parses");
    refine_theme_family(family)
        .themes
        .into_iter()
        .find(|theme| theme.name == name)
        .unwrap_or_else(|| panic!("{name} is in its theme family"))
}

fn vega_dark() -> Theme {
    bundled_theme(
        include_str!("../../../assets/themes/vega/vega.json"),
        "VEGA Dark",
    )
}

/// WCAG 2 contrast ratio between two opaque colors.
fn contrast(a: Hsla, b: Hsla) -> f32 {
    fn luminance(color: Hsla) -> f32 {
        let Rgba { r, g, b, .. } = color.to_rgb();
        let linear = |c: f32| {
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
    }
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[test]
fn vega_dark_is_the_default_dark_theme() {
    let theme = vega_dark();
    assert_eq!(theme.name.as_ref(), settings::DEFAULT_DARK_THEME);
    assert_eq!(theme.appearance(), Appearance::Dark);
}

#[test]
fn vega_dark_styles_every_syntax_capture_one_dark_does() {
    let vega = vega_dark();
    let one = bundled_theme(
        include_str!("../../../assets/themes/one/one.json"),
        "One Dark",
    );
    let mut index: usize = 0;
    while let Some(name) = one.syntax().get_capture_name(index) {
        assert!(
            vega.syntax().style_for_name(name).is_some(),
            "VEGA Dark has no style for `{name}`"
        );
        index += 1;
    }
    assert!(index > 0);
}

#[test]
fn vega_dark_syntax_meets_wcag_aa() {
    let theme = vega_dark();
    let background = theme.colors().editor_background;
    let mut index: usize = 0;
    while let Some(name) = theme.syntax().get_capture_name(index) {
        index += 1;
        // Ghost text for edit predictions is deliberately dim.
        if name == "predictive" {
            continue;
        }
        let color = theme
            .syntax()
            .style_for_name(name)
            .and_then(|style| style.color)
            .unwrap_or_else(|| panic!("`{name}` has a color"));
        let ratio = contrast(color, background);
        assert!(
            ratio >= 4.5,
            "`{name}` contrast {ratio:.2} is below WCAG AA (4.5)"
        );
    }
    let text = contrast(theme.colors().editor_foreground, background);
    assert!(text >= 4.5, "editor text contrast {text:.2}");
}
