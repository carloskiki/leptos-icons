#[cfg(feature = "SiVault")]
use leptos::*;
#[cfg(feature = "SiVault")]
///This icon requires the feature `SiVault` to be enabled.
#[component]
pub fn Vault(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 0l11.955 24L24 0zm13.366 4.827h1.393v1.38h-1.393zm-2.77 5.569H9.22V8.993h1.389zm0-2.087H9.22V6.906h1.389zm0-2.086H9.22V4.819h1.389zm2.087 6.263h-1.377V11.08h1.388zm0-2.09h-1.377V8.993h1.388zm0-2.087h-1.377V6.906h1.388zm0-2.086h-1.377V4.819h1.388zm.683.683h1.393v1.389h-1.393zm0 3.475V8.993h1.389v1.388Z"
        /> < title > { title } < / title > < / svg >
    }
}
