#[cfg(feature = "IoCarSharp")]
use leptos::*;
#[cfg(feature = "IoCarSharp")]
///This icon requires the feature `IoCarSharp` to be enabled.
#[component]
pub fn CarSharp(
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
        "M447.68,220.78a16.44,16.44,0,0,0-1-3.1l-48-112A16,16,0,0,0,384,96H128a16,16,0,0,0-14.71,9.7l-48,112a16.44,16.44,0,0,0-1,3.1A16.15,16.15,0,0,0,64,224V408a8,8,0,0,0,8,8h32a8,8,0,0,0,8-8V384H400v24a8,8,0,0,0,8,8h32a8,8,0,0,0,8-8V224A16.15,16.15,0,0,0,447.68,220.78ZM144,320a32,32,0,1,1,32-32A32,32,0,0,1,144,320Zm224,0a32,32,0,1,1,32-32A32,32,0,0,1,368,320ZM104.26,208l34.29-80h234.9l34.29,80Z"
        /> < title > { title } < / title > < / svg >
    }
}
