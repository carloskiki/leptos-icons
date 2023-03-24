#[cfg(feature = "ImRss")]
use leptos::*;
#[cfg(feature = "ImRss")]
///This icon requires the feature `ImRss` to be enabled.
#[component]
pub fn Rss(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M2.13 11.733c-1.175 0-2.13 0.958-2.13 2.126 0 1.174 0.955 2.122 2.13 2.122 1.179 0 2.133-0.948 2.133-2.122-0-1.168-0.954-2.126-2.133-2.126zM0.002 5.436v3.067c1.997 0 3.874 0.781 5.288 2.196 1.412 1.411 2.192 3.297 2.192 5.302h3.080c-0-5.825-4.739-10.564-10.56-10.564zM0.006 0v3.068c7.122 0 12.918 5.802 12.918 12.932h3.076c0-8.82-7.176-16-15.994-16z"
        /> < title > { title } < / title > < / svg >
    }
}
