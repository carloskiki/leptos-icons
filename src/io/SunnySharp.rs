#[cfg(feature = "IoSunnySharp")]
use leptos::*;
#[cfg(feature = "IoSunnySharp")]
///This icon requires the feature `IoSunnySharp` to be enabled.
#[component]
pub fn SunnySharp(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "234" y = "26" width = "44" height = "92" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "234" y = "394" width = "44" height
        = "92" />< rect xmlns = "http://www.w3.org/2000/svg" x = "340.11" y = "103.89"
        width = "92" height = "44" transform = "translate(24.07 309.89) rotate(-45)" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "79.89" y = "364.11" width = "92"
        height = "44" transform = "translate(-236.14 202.1) rotate(-45)" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "394" y = "234" width = "92" height = "44" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "26" y = "234" width = "92" height
        = "44" />< rect xmlns = "http://www.w3.org/2000/svg" x = "364.11" y = "340.11"
        width = "44" height = "92" transform = "translate(-159.93 386.11) rotate(-45)"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "103.89" y = "79.89" width =
        "44" height = "92" transform = "translate(-52.15 125.89) rotate(-45)" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M256,358A102,102,0,1,1,358,256,102.12,102.12,0,0,1,256,358Z" /> < title > {
        title } < / title > < / svg >
    }
}
