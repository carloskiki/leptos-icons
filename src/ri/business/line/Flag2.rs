#[cfg(feature = "RiBusinessLineFlag2")]
use leptos::*;
#[cfg(feature = "RiBusinessLineFlag2")]
///This icon requires the feature `RiBusinessLineFlag2` to be enabled.
#[component]
pub fn Flag2(
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
        "M4 17v5H2V3h19.138a.5.5 0 0 1 .435.748L18 10l3.573 6.252a.5.5 0 0 1-.435.748H4zM4 5v10h14.554l-2.858-5 2.858-5H4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
