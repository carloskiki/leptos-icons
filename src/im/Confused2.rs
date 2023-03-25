#[cfg(feature = "ImConfused2")]
use leptos::*;
#[cfg(feature = "ImConfused2")]
///This icon requires the feature `ImConfused2` to be enabled.
#[component]
pub fn Confused2(
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
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8c4.418 0 8-3.582 8-8s-3.582-8-8-8zM11 4c0.552 0 1 0.448 1 1s-0.448 1-1 1-1-0.448-1-1c0-0.552 0.448-1 1-1zM5 4c0.552 0 1 0.448 1 1s-0.448 1-1 1-1-0.448-1-1c0-0.552 0.448-1 1-1zM10.735 12.665c-1.295 0.472-2.733-0.199-3.204-1.494-0.283-0.777-1.145-1.179-1.923-0.896-0.712 0.259-1.109 1.005-0.953 1.725h-1.013c-0.144-1.133 0.507-2.258 1.624-2.665 1.295-0.472 2.733 0.199 3.204 1.494 0.283 0.777 1.145 1.179 1.923 0.896 0.712-0.259 1.109-1.005 0.953-1.725h1.014c0.144 1.133-0.507 2.258-1.624 2.665z"
        /> < title > { title } < / title > < / svg >
    }
}
