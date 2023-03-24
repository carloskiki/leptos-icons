#[cfg(feature = "RiBusinessFillHonour")]
use leptos::*;
#[cfg(feature = "RiBusinessFillHonour")]
///This icon requires the feature `RiBusinessFillHonour` to be enabled.
#[component]
pub fn Honour(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M21 4v14.721a.5.5 0 0 1-.298.458L12 23.03 3.298 19.18A.5.5 0 0 1 3 18.72V4H1V2h22v2h-2zM8 12v2h8v-2H8zm0-4v2h8V8H8z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
