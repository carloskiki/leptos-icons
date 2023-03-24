#[cfg(feature = "VsCombine")]
use leptos::*;
#[cfg(feature = "VsCombine")]
///This icon requires the feature `VsCombine` to be enabled.
#[component]
pub fn Combine(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M1.5 1l-.5.5v3l.5.5h3l.5-.5v-3L4.5 1h-3zM2 4V2h2v2H2zm-.5 2l-.5.5v3l.5.5h3l.5-.5v-3L4.5 6h-3zM2 9V7h2v2H2zm-1 2.5l.5-.5h3l.5.5v3l-.5.5h-3l-.5-.5v-3zm1 .5v2h2v-2H2zm10.5-7l-.5.5v6l.5.5h3l.5-.5v-6l-.5-.5h-3zM15 8h-2V6h2v2zm0 3h-2V9h2v2zM9.1 8H6v1h3.1l-1 1 .7.6 1.8-1.8v-.7L8.8 6.3l-.7.7 1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
