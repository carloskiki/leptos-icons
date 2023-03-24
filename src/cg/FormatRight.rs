#[cfg(feature = "CgFormatRight")]
use leptos::*;
#[cfg(feature = "CgFormatRight")]
///This icon requires the feature `CgFormatRight` to be enabled.
#[component]
pub fn FormatRight(
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
        "M20 5C20.5523 5 21 5.44772 21 6C21 6.55228 20.5523 7 20 7H4C3.44772 7 3 6.55228 3 6C3 5.44772 3.44772 5 4 5H20Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 9C20.5523 9 21 9.44772 21 10C21 10.5523 20.5523 11 20 11H12C11.4477 11 11 10.5523 11 10C11 9.44772 11.4477 9 12 9H20Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 14C21 13.4477 20.5523 13 20 13H4C3.44772 13 3 13.4477 3 14C3 14.5523 3.44772 15 4 15H20C20.5523 15 21 14.5523 21 14Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 17C20.5523 17 21 17.4477 21 18C21 18.5523 20.5523 19 20 19H12C11.4477 19 11 18.5523 11 18C11 17.4477 11.4477 17 12 17H20Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
