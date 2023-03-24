#[cfg(feature = "FaSolidHouseFlag")]
use leptos::*;
#[cfg(feature = "FaSolidHouseFlag")]
///This icon requires the feature `FaSolidHouseFlag` to be enabled.
#[component]
pub fn HouseFlag(
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
        "M480 0c-17.7 0-32 14.3-32 32V192 512h64V192H624c8.8 0 16-7.2 16-16V48c0-8.8-7.2-16-16-16H512c0-17.7-14.3-32-32-32zM416 159L276.8 39.7c-12-10.3-29.7-10.3-41.7 0l-224 192C1 240.4-2.7 254.5 2 267.1S18.6 288 32 288H64V480c0 17.7 14.3 32 32 32h64c17.7 0 32-14.3 32-32V384c0-17.7 14.3-32 32-32h64c17.7 0 32 14.3 32 32v96c0 17.7 14.3 32 32 32h64.7l.2 0h-1V159z"
        /> < title > { title } < / title > < / svg >
    }
}
