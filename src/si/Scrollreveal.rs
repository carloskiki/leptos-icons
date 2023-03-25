#[cfg(feature = "SiScrollreveal")]
use leptos::*;
#[cfg(feature = "SiScrollreveal")]
///This icon requires the feature `SiScrollreveal` to be enabled.
#[component]
pub fn Scrollreveal(
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
        "M1.714 2.857A1.71 1.71 0 0 0 0 4.571v6.858c0 .95.765 1.714 1.714 1.714a1.71 1.71 0 0 0 1.715-1.714V4.57a1.71 1.71 0 0 0-1.715-1.714zm6.857 0a1.71 1.71 0 0 0-1.714 1.714v1.143c0 .95.765 1.715 1.714 1.715a1.71 1.71 0 0 0 1.715-1.715V4.571A1.71 1.71 0 0 0 8.57 2.857zm6.858 0a1.71 1.71 0 0 0-1.715 1.714V19.43c0 .95.765 1.714 1.715 1.714a1.71 1.71 0 0 0 1.714-1.714V4.57a1.71 1.71 0 0 0-1.714-1.714zm6.857 0a1.71 1.71 0 0 0-1.715 1.714v6.858c0 .95.765 1.714 1.715 1.714A1.71 1.71 0 0 0 24 11.429V4.57a1.71 1.71 0 0 0-1.714-1.714zm-13.715 8a1.71 1.71 0 0 0-1.714 1.714v6.858c0 .95.765 1.714 1.714 1.714a1.71 1.71 0 0 0 1.715-1.714V12.57a1.71 1.71 0 0 0-1.715-1.714zm-6.857 5.714A1.71 1.71 0 0 0 0 18.286v1.143c0 .95.765 1.714 1.714 1.714a1.71 1.71 0 0 0 1.715-1.714v-1.143a1.71 1.71 0 0 0-1.715-1.715zm20.572 0a1.71 1.71 0 0 0-1.715 1.715v1.143c0 .95.765 1.714 1.715 1.714A1.71 1.71 0 0 0 24 19.429v-1.143a1.71 1.71 0 0 0-1.714-1.715Z"
        /> < title > { title } < / title > < / svg >
    }
}
