#[cfg(feature = "CgToggleSquare")]
use leptos::*;
#[cfg(feature = "CgToggleSquare")]
///This icon requires the feature `CgToggleSquare` to be enabled.
#[component]
pub fn ToggleSquare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 9C9.55228 9 10 9.44772 10 10V14C10 14.5523 9.55228 15 9 15H5C4.44772 15 4 14.5523 4 14V10C4 9.44772 4.44772 9 5 9H9Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M24 7C24 5.89543 23.1046 5 22 5H2C0.89543 5 0 5.89543 0 7V17C0 18.1046 0.895432 19 2 19H22C23.1046 19 24 18.1046 24 17V7ZM22 7H2V17H22V7Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
