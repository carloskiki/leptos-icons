#[cfg(feature = "IoBowlingBallOutline")]
use leptos::*;
#[cfg(feature = "IoBowlingBallOutline")]
///This icon requires the feature `IoBowlingBallOutline` to be enabled.
#[component]
pub fn BowlingBallOutline(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width = {
        size.clone() } height = { size } > < circle xmlns = "http://www.w3.org/2000/svg"
        cx = "256" cy = "256" r = "208" fill = "none" stroke = "#000" stroke - miterlimit
        = "10" stroke - width = "32" />< circle xmlns = "http://www.w3.org/2000/svg" cx =
        "288" cy = "200" r = "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx =
        "296" cy = "128" r = "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx =
        "360" cy = "168" r = "24" /> < title > { title } < / title > < / svg >
    }
}
