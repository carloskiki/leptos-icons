#[cfg(feature = "IoBrowsersSharp")]
use leptos::*;
#[cfg(feature = "IoBrowsersSharp")]
///This icon requires the feature `IoBrowsersSharp` to be enabled.
#[component]
pub fn BrowsersSharp(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M32,64V448a16,16,0,0,0,16,16H464a16,16,0,0,0,16-16V64a16,16,0,0,0-16-16H48A16,16,0,0,0,32,64ZM440,428H72a4,4,0,0,1-4-4V152a4,4,0,0,1,4-4H440a4,4,0,0,1,4,4V424A4,4,0,0,1,440,428Z"
        /> < title > { title } < / title > < / svg >
    }
}
