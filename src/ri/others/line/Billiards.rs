#[cfg(feature = "RiOthersLineBilliards")]
use leptos::*;
#[cfg(feature = "RiOthersLineBilliards")]
///This icon requires the feature `RiOthersLineBilliards` to be enabled.
#[component]
pub fn Billiards(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2zm0 2a8 8 0 1 0 0 16 8 8 0 0 0 0-16zm0 2a6 6 0 1 1 0 12 6 6 0 0 1 0-12zm0 1.75a2.5 2.5 0 0 0-1.88 4.147c-.565.457-.92 1.118-.92 1.853 0 1.38 1.254 2.5 2.8 2.5 1.546 0 2.8-1.12 2.8-2.5 0-.735-.355-1.396-.92-1.852A2.5 2.5 0 0 0 12 7.75zm0 5c.753 0 1.3.488 1.3 1s-.547 1-1.3 1-1.3-.488-1.3-1 .547-1 1.3-1zm0-3.5a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
