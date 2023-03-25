#[cfg(feature = "RiLogosLineOpera")]
use leptos::*;
#[cfg(feature = "RiLogosLineOpera")]
///This icon requires the feature `RiLogosLineOpera` to be enabled.
#[component]
pub fn Opera(
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
        "M14.766 19.51a8.003 8.003 0 0 0 0-15.02C16.71 5.977 18 8.935 18 12s-1.289 6.024-3.234 7.51zM9.234 4.49a8.003 8.003 0 0 0 0 15.02C7.29 18.023 6 15.065 6 12s1.289-6.024 3.234-7.51zM12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10zm0-3.5c2 0 4-3.033 4-6.5s-2-6.5-4-6.5S8 8.533 8 12s2 6.5 4 6.5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
