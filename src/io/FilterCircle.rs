#[cfg(feature = "IoFilterCircle")]
use leptos::*;
#[cfg(feature = "IoFilterCircle")]
///This icon requires the feature `IoFilterCircle` to be enabled.
#[component]
pub fn FilterCircle(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm32,304H224a16,16,0,0,1,0-32h64a16,16,0,0,1,0,32Zm48-64H176a16,16,0,0,1,0-32H336a16,16,0,0,1,0,32Zm32-64H144a16,16,0,0,1,0-32H368a16,16,0,0,1,0,32Z"
        /> < title > { title } < / title > < / svg >
    }
}
