#[cfg(feature = "ImBlogger2")]
use leptos::*;
#[cfg(feature = "ImBlogger2")]
///This icon requires the feature `ImBlogger2` to be enabled.
#[component]
pub fn Blogger2(
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
        = "#000000" d =
        "M14.5 0h-13c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM14 10.125c0 2.141-1.741 3.875-3.897 3.875h-4.2c-2.156 0-3.903-1.734-3.903-3.875v-4.25c0-2.141 1.747-3.875 3.903-3.875h1.966c2.156 0 3.881 1.609 3.881 3.75 0.028 0.4 0.391 0.75 0.8 0.75h0.672c0.431 0 0.775 0.453 0.775 0.881v2.744z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11 10c0 0.55-0.45 1-1 1h-4c-0.55 0-1-0.45-1-1v0c0-0.55 0.45-1 1-1h4c0.55 0 1 0.45 1 1v0z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M9 6c0 0.55-0.45 1-1 1h-2c-0.55 0-1-0.45-1-1v0c0-0.55 0.45-1 1-1h2c0.55 0 1 0.45 1 1v0z"
        /> < title > { title } < / title > < / svg >
    }
}
