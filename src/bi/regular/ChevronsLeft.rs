#[cfg(feature = "BiRegularChevronsLeft")]
use leptos::*;
#[cfg(feature = "BiRegularChevronsLeft")]
///This icon requires the feature `BiRegularChevronsLeft` to be enabled.
#[component]
pub fn ChevronsLeft(
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
        xmlns = "http://www.w3.org/2000/svg" d =
        "m12.707 7.707-1.414-1.414L5.586 12l5.707 5.707 1.414-1.414L8.414 12z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M16.293 6.293 10.586 12l5.707 5.707 1.414-1.414L13.414 12l4.293-4.293z" /> <
        title > { title } < / title > < / svg >
    }
}
