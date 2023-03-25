#[cfg(feature = "ImArrowDown2")]
use leptos::*;
#[cfg(feature = "ImArrowDown2")]
///This icon requires the feature `ImArrowDown2` to be enabled.
#[component]
pub fn ArrowDown2(
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
        "M13.707 9.707l-5 5c-0.39 0.391-1.024 0.391-1.414 0l-5-5c-0.391-0.391-0.391-1.024 0-1.414s1.024-0.391 1.414 0l3.293 3.293v-9.586c0-0.552 0.448-1 1-1s1 0.448 1 1v9.586l3.293-3.293c0.195-0.195 0.451-0.293 0.707-0.293s0.512 0.098 0.707 0.293c0.391 0.391 0.391 1.024 0 1.414z"
        /> < title > { title } < / title > < / svg >
    }
}
