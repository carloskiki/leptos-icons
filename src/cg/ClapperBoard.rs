#[cfg(feature = "CgClapperBoard")]
use leptos::*;
#[cfg(feature = "CgClapperBoard")]
///This icon requires the feature `CgClapperBoard` to be enabled.
#[component]
pub fn ClapperBoard(
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
        "M20.1702 3L20.1663 3.00453C21.7458 3.09084 23 4.39896 23 6V18C23 19.6569 21.6569 21 20 21H4C2.34315 21 1 19.6569 1 18V6C1 4.34315 2.34315 3 4 3H20.1702ZM10.4764 5H16.4764L13.089 9H7.08899L10.4764 5ZM5.08899 9L8.47644 5H4C3.44772 5 3 5.44772 3 6V9H5.08899ZM3 11V18C3 18.5523 3.44772 19 4 19H20C20.5523 19 21 18.5523 21 18V11H3ZM21 9V6C21 5.44771 20.5523 5 20 5H18.4764L15.089 9H21Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
