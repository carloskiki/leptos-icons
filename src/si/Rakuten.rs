#[cfg(feature = "SiRakuten")]
use leptos::*;
#[cfg(feature = "SiRakuten")]
///This icon requires the feature `SiRakuten` to be enabled.
#[component]
pub fn Rakuten(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M23.277 21.3L3.939 24 .722 21.3h22.555zM7.6 19.276H3.939V0h6.052a6.653 6.653 0 0 1 6.65 6.646c0 2.234-1.108 4.204-2.799 5.418l5.418 7.211h-4.585l-4.486-5.979H7.6v5.98zm0-9.64h2.392a2.992 2.992 0 0 0 2.989-2.989 2.994 2.994 0 0 0-2.989-2.986H7.6v5.975z"
        /> < title > { title } < / title > < / svg >
    }
}
