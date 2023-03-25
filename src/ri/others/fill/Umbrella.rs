#[cfg(feature = "RiOthersFillUmbrella")]
use leptos::*;
#[cfg(feature = "RiOthersFillUmbrella")]
///This icon requires the feature `RiOthersFillUmbrella` to be enabled.
#[component]
pub fn Umbrella(
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
        "M13 2.05c5.053.501 9 4.765 9 9.95v1h-9v6a2 2 0 1 0 4 0v-1h2v1a4 4 0 1 1-8 0v-6H2v-1c0-5.185 3.947-9.449 9-9.95V2a1 1 0 0 1 2 0v.05z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
