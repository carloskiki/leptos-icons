#[cfg(feature = "ImStatsBars2")]
use leptos::*;
#[cfg(feature = "ImStatsBars2")]
///This icon requires the feature `ImStatsBars2` to be enabled.
#[component]
pub fn StatsBars2(
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
        "M4.5 6h-3c-0.275 0-0.5 0.225-0.5 0.5v9c0 0.275 0.225 0.5 0.5 0.5h3c0.275 0 0.5-0.225 0.5-0.5v-9c0-0.275-0.225-0.5-0.5-0.5zM4.5 15h-3v-4h3v4zM9.5 4h-3c-0.275 0-0.5 0.225-0.5 0.5v11c0 0.275 0.225 0.5 0.5 0.5h3c0.275 0 0.5-0.225 0.5-0.5v-11c0-0.275-0.225-0.5-0.5-0.5zM9.5 15h-3v-5h3v5zM14.5 2h-3c-0.275 0-0.5 0.225-0.5 0.5v13c0 0.275 0.225 0.5 0.5 0.5h3c0.275 0 0.5-0.225 0.5-0.5v-13c0-0.275-0.225-0.5-0.5-0.5zM14.5 15h-3v-6h3v6z"
        /> < title > { title } < / title > < / svg >
    }
}
