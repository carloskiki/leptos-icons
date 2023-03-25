#[cfg(feature = "ImGlass")]
use leptos::*;
#[cfg(feature = "ImGlass")]
///This icon requires the feature `ImGlass` to be enabled.
#[component]
pub fn Glass(
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
        "M12.153 0.263c-0.087-0.162-0.256-0.263-0.44-0.263h-7.425c-0.184 0-0.353 0.101-0.44 0.263-0.554 1.032-0.847 2.237-0.847 3.487 0 1.647 0.506 3.2 1.424 4.374 0.71 0.907 1.601 1.508 2.576 1.753v5.123h-1.5c-0.276 0-0.5 0.224-0.5 0.5s0.224 0.5 0.5 0.5h5c0.276 0 0.5-0.224 0.5-0.5s-0.224-0.5-0.5-0.5h-1.5v-5.123c0.975-0.244 1.866-0.846 2.576-1.753 0.918-1.174 1.424-2.727 1.424-4.374 0-1.249-0.293-2.455-0.847-3.487zM4.595 1h6.809c0.39 0.827 0.595 1.771 0.595 2.75 0 0.084-0.002 0.167-0.005 0.25h-7.991c-0.003-0.083-0.005-0.166-0.005-0.25-0-0.979 0.205-1.923 0.595-2.75z"
        /> < title > { title } < / title > < / svg >
    }
}
