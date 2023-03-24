#[cfg(feature = "CgChevronDoubleLeftR")]
use leptos::*;
#[cfg(feature = "CgChevronDoubleLeftR")]
///This icon requires the feature `CgChevronDoubleLeftR` to be enabled.
#[component]
pub fn ChevronDoubleLeftR(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.7071 9.1716L11.2929 7.75739L7.05024 12L11.2929 16.2426L12.7071 14.8284L9.87869 12L12.7071 9.1716Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.5355 7.75739L16.9497 9.1716L14.1213 12L16.9497 14.8284L15.5355 16.2426L11.2929 12L15.5355 7.75739Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1 5C1 2.79086 2.79086 1 5 1H19C21.2091 1 23 2.79086 23 5V19C23 21.2091 21.2091 23 19 23H5C2.79086 23 1 21.2091 1 19V5ZM5 3H19C20.1046 3 21 3.89543 21 5V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V5C3 3.89543 3.89543 3 5 3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
