#[cfg(feature = "IoJournalSharp")]
use leptos::*;
#[cfg(feature = "IoJournalSharp")]
///This icon requires the feature `IoJournalSharp` to be enabled.
#[component]
pub fn JournalSharp(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M290,32H104A24,24,0,0,0,80,56V456a24,24,0,0,0,24,24H290Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M408,32H350V480h58a24,24,0,0,0,24-24V56A24,24,0,0,0,408,32Z" /> < title > {
        title } < / title > < / svg >
    }
}
