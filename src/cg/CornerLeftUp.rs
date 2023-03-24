#[cfg(feature = "CgCornerLeftUp")]
use leptos::*;
#[cfg(feature = "CgCornerLeftUp")]
///This icon requires the feature `CgCornerLeftUp` to be enabled.
#[component]
pub fn CornerLeftUp(
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
        "M14.71 10.6254L16.1229 9.20989L9.7531 2.85181L3.39502 9.22164L4.81054 10.6346L8.59179 6.84631L8.60131 17.152C8.60335 19.3611 10.3959 21.1503 12.605 21.1483L20.605 21.1409L20.6032 19.1409L12.6032 19.1483C11.4986 19.1493 10.6023 18.2547 10.6013 17.1501L10.5915 6.51446L14.71 10.6254Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
