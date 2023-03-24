#[cfg(feature = "FaSolidRug")]
use leptos::*;
#[cfg(feature = "FaSolidRug")]
///This icon requires the feature `FaSolidRug` to be enabled.
#[component]
pub fn Rug(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M24 64H56 80V88v88 80 80 88 24H56 24c-13.3 0-24-10.7-24-24s10.7-24 24-24h8V360H24c-13.3 0-24-10.7-24-24s10.7-24 24-24h8V280H24c-13.3 0-24-10.7-24-24s10.7-24 24-24h8V200H24c-13.3 0-24-10.7-24-24s10.7-24 24-24h8V112H24C10.7 112 0 101.3 0 88S10.7 64 24 64zm88 0H528V448H112V64zM640 88c0 13.3-10.7 24-24 24h-8v40h8c13.3 0 24 10.7 24 24s-10.7 24-24 24h-8v32h8c13.3 0 24 10.7 24 24s-10.7 24-24 24h-8v32h8c13.3 0 24 10.7 24 24s-10.7 24-24 24h-8v40h8c13.3 0 24 10.7 24 24s-10.7 24-24 24H584 560V424 336 256 176 88 64h24 32c13.3 0 24 10.7 24 24z"
        /> < title > { title } < / title > < / svg >
    }
}
