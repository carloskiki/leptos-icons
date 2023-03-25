#[cfg(feature = "RiLogosLineMiniProgram")]
use leptos::*;
#[cfg(feature = "RiLogosLineMiniProgram")]
///This icon requires the feature `RiLogosLineMiniProgram` to be enabled.
#[component]
pub fn MiniProgram(
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
        "M12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10zm0-2a8 8 0 1 0 0-16 8 8 0 0 0 0 16zm1-6a3.5 3.5 0 1 1-4.977-3.174 1 1 0 1 1 .845 1.813A1.5 1.5 0 1 0 11 14v-4a3.5 3.5 0 1 1 4.977 3.174 1 1 0 0 1-.845-1.813A1.5 1.5 0 1 0 13 10v4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
