#[cfg(feature = "CgBattery")]
use leptos::*;
#[cfg(feature = "CgBattery")]
///This icon requires the feature `CgBattery` to be enabled.
#[component]
pub fn Battery(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6 15C5.44772 15 5 14.5523 5 14V10C5 9.44772 5.44772 9 6 9H12V15H6Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M18 6H5C3.34315 6 2 7.34315 2 9V15C2 16.6569 3.34315 18 5 18H18C19.6569 18 21 16.6569 21 15C21.5523 15 22 14.5523 22 14V10C22 9.44772 21.5523 9 21 9C21 7.34315 19.6569 6 18 6ZM18 8H5C4.44772 8 4 8.44772 4 9V15C4 15.5523 4.44772 16 5 16H18C18.5523 16 19 15.5523 19 15V9C19 8.44772 18.5523 8 18 8Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
