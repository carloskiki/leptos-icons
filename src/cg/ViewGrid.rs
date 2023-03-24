#[cfg(feature = "CgViewGrid")]
use leptos::*;
#[cfg(feature = "CgViewGrid")]
///This icon requires the feature `CgViewGrid` to be enabled.
#[component]
pub fn ViewGrid(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M5 5C3.34315 5 2 6.34315 2 8V16C2 17.6569 3.34315 19 5 19H19C20.6569 19 22 17.6569 22 16V8C22 6.34315 20.6569 5 19 5H5ZM8 7H5C4.44772 7 4 7.44772 4 8V9H8V7ZM10 7V9H14V7H10ZM16 7V9H20V8C20 7.44771 19.5523 7 19 7H16ZM14 11H10V13H14V11ZM16 13V11H20V13H16ZM14 15H10V17H14V15ZM16 17V15H20V16C20 16.5523 19.5523 17 19 17H16ZM8 17V15H4V16C4 16.5523 4.44772 17 5 17H8ZM8 13V11H4V13H8Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
