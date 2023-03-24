#[cfg(feature = "IoAlbums")]
use leptos::*;
#[cfg(feature = "IoAlbums")]
///This icon requires the feature `IoAlbums` to be enabled.
#[component]
pub fn Albums(
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
        "M368,96H144a16,16,0,0,1,0-32H368a16,16,0,0,1,0,32Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M400,144H112a16,16,0,0,1,0-32H400a16,16,0,0,1,0,32Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M419.13,448H92.87A44.92,44.92,0,0,1,48,403.13V204.87A44.92,44.92,0,0,1,92.87,160H419.13A44.92,44.92,0,0,1,464,204.87V403.13A44.92,44.92,0,0,1,419.13,448Z"
        /> < title > { title } < / title > < / svg >
    }
}
