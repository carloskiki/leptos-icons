#[cfg(feature = "RiOthersFillKnife")]
use leptos::*;
#[cfg(feature = "RiOthersFillKnife")]
///This icon requires the feature `RiOthersFillKnife` to be enabled.
#[component]
pub fn Knife(
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
        "M22.373 19.44a1.5 1.5 0 0 1-2.121 2.12l-4.596-4.596L12.12 20.5l-7.778-7.778a8 8 0 0 1-.174-11.135l.174-.179L22.373 19.44z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
