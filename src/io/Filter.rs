#[cfg(feature = "IoFilter")]
use leptos::*;
#[cfg(feature = "IoFilter")]
///This icon requires the feature `IoFilter` to be enabled.
#[component]
pub fn Filter(
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
        "M472,168H40a24,24,0,0,1,0-48H472a24,24,0,0,1,0,48Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M392,280H120a24,24,0,0,1,0-48H392a24,24,0,0,1,0,48Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M296,392H216a24,24,0,0,1,0-48h80a24,24,0,0,1,0,48Z" /> < title > { title } < /
        title > < / svg >
    }
}
