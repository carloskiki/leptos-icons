#[cfg(feature = "RiMediaFillImageEdit")]
use leptos::*;
#[cfg(feature = "RiMediaFillImageEdit")]
///This icon requires the feature `RiMediaFillImageEdit` to be enabled.
#[component]
pub fn ImageEdit(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M20 3c.552 0 1 .448 1 1v1.757l-2 2V5H5v8.1l4-4 4.328 4.329-1.327 1.327-.006 4.239 4.246.006 1.33-1.33L18.899 19H19v-2.758l2-2V20c0 .552-.448 1-1 1H4c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h16zm1.778 4.808l1.414 1.414L15.414 17l-1.416-.002.002-1.412 7.778-7.778zM15.5 7c.828 0 1.5.672 1.5 1.5s-.672 1.5-1.5 1.5S14 9.328 14 8.5 14.672 7 15.5 7z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
