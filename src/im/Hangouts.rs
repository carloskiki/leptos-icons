#[cfg(feature = "ImHangouts")]
use leptos::*;
#[cfg(feature = "ImHangouts")]
///This icon requires the feature `ImHangouts` to be enabled.
#[component]
pub fn Hangouts(
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
        "M7.997 0c-3.816 0-6.909 3.094-6.909 6.909 0 3.616 3.294 6.547 6.909 6.547v2.544c4.197-2.128 6.916-5.556 6.916-9.091 0-3.816-3.1-6.909-6.916-6.909zM7 8c0 0.828-0.447 1.5-1 1.5v-1.5h-2v-3h3v3zM12 8c0 0.828-0.447 1.5-1 1.5v-1.5h-2v-3h3v3z"
        /> < title > { title } < / title > < / svg >
    }
}
