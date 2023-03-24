#[cfg(feature = "VsRepo")]
use leptos::*;
#[cfg(feature = "VsRepo")]
///This icon requires the feature `VsRepo` to be enabled.
#[component]
pub fn Repo(
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
        "M14 10V1.5l-.5-.5H3.74a1.9 1.9 0 0 0-.67.13 1.77 1.77 0 0 0-.94 1 1.7 1.7 0 0 0-.13.62v9.5a1.7 1.7 0 0 0 .13.67c.177.427.515.768.94.95a1.9 1.9 0 0 0 .67.13H4v-1h-.26a.72.72 0 0 1-.29-.06.74.74 0 0 1-.4-.4.93.93 0 0 1-.05-.29v-.5a.93.93 0 0 1 .05-.29.74.74 0 0 1 .4-.4.72.72 0 0 1 .286-.06H13v2H9v1h4.5l.5-.5V10zM4 10V2h9v8H4zm1-7h1v1H5V3zm0 2h1v1H5V5zm1 2H5v1h1V7zm.5 6.49L5.28 15H5v-3h3v3h-.28L6.5 13.49z"
        /> < title > { title } < / title > < / svg >
    }
}
