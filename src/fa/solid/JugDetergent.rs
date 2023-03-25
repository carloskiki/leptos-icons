#[cfg(feature = "FaSolidJugDetergent")]
use leptos::*;
#[cfg(feature = "FaSolidJugDetergent")]
///This icon requires the feature `FaSolidJugDetergent` to be enabled.
#[component]
pub fn JugDetergent(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M96 24c0-13.3 10.7-24 24-24h80c13.3 0 24 10.7 24 24V48h8c13.3 0 24 10.7 24 24s-10.7 24-24 24H88C74.7 96 64 85.3 64 72s10.7-24 24-24h8V24zM0 256c0-70.7 57.3-128 128-128H256c70.7 0 128 57.3 128 128V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V256zm256 0v96c0 17.7 14.3 32 32 32s32-14.3 32-32V256c0-17.7-14.3-32-32-32s-32 14.3-32 32z"
        /> < title > { title } < / title > < / svg >
    }
}
