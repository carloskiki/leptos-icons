#[cfg(feature = "CgCornerLeftDown")]
use leptos::*;
#[cfg(feature = "CgCornerLeftDown")]
///This icon requires the feature `CgCornerLeftDown` to be enabled.
#[component]
pub fn CornerLeftDown(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.6013 6.84996C10.6023 5.74539 11.4986 4.85079 12.6032 4.85181L20.6032 4.8592L20.605 2.8592L12.605 2.85181C10.3959 2.84977 8.60335 4.63897 8.60131 6.84811L8.59179 17.1538L4.81054 13.3656L3.39502 14.7785L9.7531 21.1483L16.1229 14.7902L14.71 13.3747L10.5915 17.4856L10.6013 6.84996Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
