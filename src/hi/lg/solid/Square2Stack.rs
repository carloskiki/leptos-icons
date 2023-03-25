#[cfg(feature = "HiLgSolidSquare2Stack")]
use leptos::*;
#[cfg(feature = "HiLgSolidSquare2Stack")]
///This icon requires the feature `HiLgSolidSquare2Stack` to be enabled.
#[component]
pub fn Square2Stack(
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
        "M16.5 6C16.5 4.34315 15.1569 3 13.5 3H6C4.34315 3 3 4.34315 3 6V13.5C3 15.1569 4.34315 16.5 6 16.5V10.5C6 8.01472 8.01472 6 10.5 6H16.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 7.5C19.6569 7.5 21 8.84315 21 10.5V18C21 19.6569 19.6569 21 18 21H10.5C8.84315 21 7.5 19.6569 7.5 18V10.5C7.5 8.84315 8.84315 7.5 10.5 7.5H18Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
