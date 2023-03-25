#[cfg(feature = "ImVideoCamera")]
use leptos::*;
#[cfg(feature = "ImVideoCamera")]
///This icon requires the feature `ImVideoCamera` to be enabled.
#[component]
pub fn VideoCamera(
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
        "M6 4.5c0-1.381 1.119-2.5 2.5-2.5s2.5 1.119 2.5 2.5c0 1.381-1.119 2.5-2.5 2.5s-2.5-1.119-2.5-2.5zM0 4.5c0-1.381 1.119-2.5 2.5-2.5s2.5 1.119 2.5 2.5c0 1.381-1.119 2.5-2.5 2.5s-2.5-1.119-2.5-2.5zM12 9.5v-1.5c0-0.55-0.45-1-1-1h-10c-0.55 0-1 0.45-1 1v5c0 0.55 0.45 1 1 1h10c0.55 0 1-0.45 1-1v-1.5l4 2.5v-7l-4 2.5zM10 12h-8v-3h8v3z"
        /> < title > { title } < / title > < / svg >
    }
}
