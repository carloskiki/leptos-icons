#[cfg(feature = "IoBusinessSharp")]
use leptos::*;
#[cfg(feature = "IoBusinessSharp")]
///This icon requires the feature `IoBusinessSharp` to be enabled.
#[component]
pub fn BusinessSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,176V16H32V496H160V400h32v96H480V176ZM112,432H80V400h32Zm0-80H80V320h32Zm0-80H80V240h32Zm0-80H80V160h32Zm0-80H80V80h32ZM240,80h32v32H240ZM192,352H160V320h32Zm0-80H160V240h32Zm0-80H160V160h32Zm0-80H160V80h32Zm80,320H240V400h32Zm0-80H240V320h32Zm0-80H240V240h32Zm0-80H240V160h32v32ZM448,464H320V432h32V400H320V352h32V320H320V272h32V240H320V208H448Z"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "384" y = "400" width = "32"
        height = "32" />< rect xmlns = "http://www.w3.org/2000/svg" x = "384" y = "320"
        width = "32" height = "32" />< rect xmlns = "http://www.w3.org/2000/svg" x =
        "384" y = "240" width = "32" height = "32" /> < title > { title } < / title > < /
        svg >
    }
}
