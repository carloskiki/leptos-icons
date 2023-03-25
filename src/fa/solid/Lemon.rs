#[cfg(feature = "FaSolidLemon")]
use leptos::*;
#[cfg(feature = "FaSolidLemon")]
///This icon requires the feature `FaSolidLemon` to be enabled.
#[component]
pub fn Lemon(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M448 96c0-35.3-28.7-64-64-64c-6.6 0-13 1-19 2.9c-22.5 7-48.1 14.9-71 9c-75.2-19.1-156.4 11-213.7 68.3S-7.2 250.8 11.9 326c5.8 22.9-2 48.4-9 71C1 403 0 409.4 0 416c0 35.3 28.7 64 64 64c6.6 0 13-1 19.1-2.9c22.5-7 48.1-14.9 71-9c75.2 19.1 156.4-11 213.7-68.3s87.5-138.5 68.3-213.7c-5.8-22.9 2-48.4 9-71c1.9-6 2.9-12.4 2.9-19.1zM212.5 127.4c-54.6 16-101.1 62.5-117.1 117.1C92.9 253 84 257.8 75.5 255.4S62.2 244 64.6 235.5c19.1-65.1 73.7-119.8 138.9-138.9c8.5-2.5 17.4 2.4 19.9 10.9s-2.4 17.4-10.9 19.9z"
        /> < title > { title } < / title > < / svg >
    }
}
