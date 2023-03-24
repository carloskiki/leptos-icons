#[cfg(feature = "IoPlayCircle")]
use leptos::*;
#[cfg(feature = "IoPlayCircle")]
///This icon requires the feature `IoPlayCircle` to be enabled.
#[component]
pub fn PlayCircle(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm74.77,217.3L216.32,334.44A10.78,10.78,0,0,1,200,325.13V186.87a10.78,10.78,0,0,1,16.32-9.31L330.77,246.7A10.89,10.89,0,0,1,330.77,265.3Z"
        /> < title > { title } < / title > < / svg >
    }
}
