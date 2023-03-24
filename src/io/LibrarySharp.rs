#[cfg(feature = "IoLibrarySharp")]
use leptos::*;
#[cfg(feature = "IoLibrarySharp")]
///This icon requires the feature `IoLibrarySharp` to be enabled.
#[component]
pub fn LibrarySharp(
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
        "M84,480H28a12,12,0,0,1-12-12V92A12,12,0,0,1,28,80H84A12,12,0,0,1,96,92V468A12,12,0,0,1,84,480Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,208V156a12,12,0,0,0-12-12H124a12,12,0,0,0-12,12v52Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M112,416v52a12,12,0,0,0,12,12H228a12,12,0,0,0,12-12V416Z" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "112" y = "240" width = "128" height = "144" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M340,480H268a12,12,0,0,1-12-12V44a12,12,0,0,1,12-12h72a12,12,0,0,1,12,12V468A12,12,0,0,1,340,480Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M369,100.7l30,367.83a12,12,0,0,0,13.45,10.92l72.16-9a12,12,0,0,0,10.47-12.9L465,91.21a12,12,0,0,0-13.2-10.94l-72.13,7.51A12,12,0,0,0,369,100.7Z"
        /> < title > { title } < / title > < / svg >
    }
}
