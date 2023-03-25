#[cfg(feature = "ImTumblr2")]
use leptos::*;
#[cfg(feature = "ImTumblr2")]
///This icon requires the feature `ImTumblr2` to be enabled.
#[component]
pub fn Tumblr2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M14.5 0h-13c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM11.434 12.884c-0.472 0.222-0.9 0.378-1.281 0.469-0.381 0.088-0.797 0.134-1.241 0.134-0.506 0-0.803-0.063-1.191-0.191s-0.719-0.309-0.994-0.544c-0.275-0.238-0.463-0.488-0.569-0.753s-0.159-0.65-0.159-1.156v-3.872h-1.5v-1.563c0.434-0.141 0.938-0.344 1.244-0.606 0.309-0.263 0.559-0.578 0.744-0.947 0.188-0.369 0.316-0.837 0.388-1.406h1.569v2.55h2.556v1.972h-2.553v2.831c0 0.641-0.009 1.009 0.059 1.191s0.238 0.369 0.422 0.475c0.244 0.147 0.525 0.219 0.838 0.219 0.559 0 1.116-0.181 1.669-0.544v1.741z"
        /> < title > { title } < / title > < / svg >
    }
}
