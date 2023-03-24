#[cfg(feature = "ImSortAmountAsc")]
use leptos::*;
#[cfg(feature = "ImSortAmountAsc")]
///This icon requires the feature `ImSortAmountAsc` to be enabled.
#[component]
pub fn SortAmountAsc(
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
        = "#000000" d = "M5 12v-12h-2v12h-2.5l3.5 3.5 3.5-3.5h-2.5z" />< path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d = "M7 9h9v2h-9v-2z" />< path xmlns = "http://www.w3.org/2000/svg"
        xmlns : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M7 6h7v2h-7v-2z" />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M7 3h5v2h-5v-2z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M7 0h3v2h-3v-2z" /> < title
        > { title } < / title > < / svg >
    }
}
