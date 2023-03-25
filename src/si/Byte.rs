#[cfg(feature = "SiByte")]
use leptos::*;
#[cfg(feature = "SiByte")]
///This icon requires the feature `SiByte` to be enabled.
#[component]
pub fn Byte(
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
        "M0 0v16.114h16.14V9.838c-.025-.633-.579-1.082-1.317-1.082-.739 0-1.294.449-1.32 1.108v3.614c-1.712-.002-3.435.003-5.142-.002a6.536 6.536 0 0 1 6.435-5.248c3.64.027 6.567 2.955 6.567 6.568a6.552 6.552 0 0 1-12.369 3.032l-.053-.104c-.396-.818-.739-1.188-1.583-1.24-.844-.027-1.503.447-1.292 1.133A9.175 9.175 0 0 0 14.796 24 9.195 9.195 0 0 0 24 14.796c0-4.537-3.428-8.466-7.886-9.1V0zm2.638 2.638h10.84v3.059a9.175 9.175 0 0 0-7.781 7.78c-1.013.002-2.04 0-3.06 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
