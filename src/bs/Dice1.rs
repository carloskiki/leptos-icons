#[cfg(feature = "BsDice1")]
use leptos::*;
#[cfg(feature = "BsDice1")]
///This icon requires the feature `BsDice1` to be enabled.
#[component]
pub fn Dice1(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-dice-1" viewBox = "0 0 16 16" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "8" cy = "8" r = "1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13 1a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h10zM3 0a3 3 0 0 0-3 3v10a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3V3a3 3 0 0 0-3-3H3z"
        /> < title > { title } < / title > < / svg >
    }
}
