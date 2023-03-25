#[cfg(feature = "RiSystemLineToggle")]
use leptos::*;
#[cfg(feature = "RiSystemLineToggle")]
///This icon requires the feature `RiSystemLineToggle` to be enabled.
#[component]
pub fn Toggle(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M8 7a5 5 0 1 0 0 10h8a5 5 0 0 0 0-10H8zm0-2h8a7 7 0 0 1 0 14H8A7 7 0 0 1 8 5zm0 10a3 3 0 1 1 0-6 3 3 0 0 1 0 6z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
