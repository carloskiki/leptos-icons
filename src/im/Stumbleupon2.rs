#[cfg(feature = "ImStumbleupon2")]
use leptos::*;
#[cfg(feature = "ImStumbleupon2")]
///This icon requires the feature `ImStumbleupon2` to be enabled.
#[component]
pub fn Stumbleupon2(
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
        "M13.313 0h-10.625c-1.478 0-2.688 1.209-2.688 2.688v10.625c0 1.478 1.209 2.688 2.688 2.688h10.625c1.478 0 2.688-1.209 2.688-2.688v-10.625c0-1.478-1.209-2.688-2.688-2.688zM8 5c-0.551 0-1 0.449-1 1v4c0 1.654-1.346 3-3 3s-3-1.346-3-3v-2h2v2c0 0.551 0.449 1 1 1s1-0.449 1-1v-4c0-1.654 1.346-3 3-3s3 1.346 3 2.781v0.969l-1.281 0.375-0.719-0.375v-0.969c0-0.333-0.449-0.781-1-0.781zM15 10c0 1.654-1.346 3-3 3s-3-1.346-3-3.219v-1.938l0.719 0.375 1.281-0.375v1.938c0 0.77 0.449 1.219 1 1.219s1-0.449 1-1v-2h2v2z"
        /> < title > { title } < / title > < / svg >
    }
}
