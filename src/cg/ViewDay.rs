#[cfg(feature = "CgViewDay")]
use leptos::*;
#[cfg(feature = "CgViewDay")]
///This icon requires the feature `CgViewDay` to be enabled.
#[component]
pub fn ViewDay(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M2 8C2 6.34315 3.34315 5 5 5H19C20.6569 5 22 6.34315 22 8V16C22 17.6569 20.6569 19 19 19H5C3.34315 19 2 17.6569 2 16V8ZM13 7H19C19.5523 7 20 7.44771 20 8V11H13V7ZM11 7H5C4.44772 7 4 7.44772 4 8V11H11V7ZM4 13V16C4 16.5523 4.44772 17 5 17H11V13H4ZM13 17H19C19.5523 17 20 16.5523 20 16V13H13V17Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
