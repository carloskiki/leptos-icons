#[cfg(feature = "BiSolidWatch")]
use leptos::*;
#[cfg(feature = "BiSolidWatch")]
///This icon requires the feature `BiSolidWatch` to be enabled.
#[component]
pub fn Watch(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d = "M13 8h-2v5h5v-2h-3z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19.999 12c0-2.953-1.612-5.53-3.999-6.916V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v2.083C5.613 6.469 4.001 9.047 4.001 12a8.003 8.003 0 0 0 4.136 7H8v2.041a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V19h-.139a8 8 0 0 0 4.138-7zm-8 5.999A6.005 6.005 0 0 1 6.001 12a6.005 6.005 0 0 1 5.998-5.999c3.31 0 6 2.691 6 5.999a6.005 6.005 0 0 1-6 5.999z"
        /> < title > { title } < / title > < / svg >
    }
}
