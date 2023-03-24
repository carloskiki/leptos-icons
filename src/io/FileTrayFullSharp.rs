#[cfg(feature = "IoFileTrayFullSharp")]
use leptos::*;
#[cfg(feature = "IoFileTrayFullSharp")]
///This icon requires the feature `IoFileTrayFullSharp` to be enabled.
#[component]
pub fn FileTrayFullSharp(
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
        "http://www.w3.org/2000/svg" x = "128" y = "128" width = "256" height = "38" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "112" y = "192" width = "288"
        height = "38" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M448,64H64L32,256V448H480V256ZM436,256H320a64,64,0,0,1-128,0H76L98,106H414Z" />
        < title > { title } < / title > < / svg >
    }
}
