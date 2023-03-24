#[cfg(feature = "ImSpellCheck")]
use leptos::*;
#[cfg(feature = "ImSpellCheck")]
///This icon requires the feature `ImSpellCheck` to be enabled.
#[component]
pub fn SpellCheck(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M2 4h2v3h1v-6c0-0.55-0.45-1-1-1h-2c-0.55 0-1 0.45-1 1v6h1v-3zM2 1h2v2h-2v-2zM15 1v-1h-3c-0.55 0-1 0.45-1 1v5c0 0.55 0.45 1 1 1h3v-1h-3v-5h3zM10 2.5v-1.5c0-0.55-0.45-1-1-1h-3v7h3c0.55 0 1-0.45 1-1v-1.5c0-0.55-0.137-1-0.688-1 0.55 0 0.688-0.45 0.688-1zM9 6h-2v-2h2v2zM9 3h-2v-2h2v2zM13 9l-6.5 7-3.5-4.5 1.281-1.094 2.219 2.313 5.5-4.719z"
        /> < title > { title } < / title > < / svg >
    }
}
