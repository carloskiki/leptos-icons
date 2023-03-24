#[cfg(feature = "BiSolidFlagCheckered")]
use leptos::*;
#[cfg(feature = "BiSolidFlagCheckered")]
///This icon requires the feature `BiSolidFlagCheckered` to be enabled.
#[component]
pub fn FlagCheckered(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M4 2v18H3v2h4v-2H6v-5h13a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1H6V2H4zm4 3v2h2V5h2v2h2V5h2v2h2v2h-2v2h2v2h-2v-2h-2v2h-2v-2h-2v2H8v-2H6V9h2V7H6V5h2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 9h2v2H8zm4 0h2v2h-2zm-2-2h2v2h-2zm4 0h2v2h-2z" /> < title > { title } < /
        title > < / svg >
    }
}
