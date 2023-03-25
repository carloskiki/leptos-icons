#[cfg(feature = "BiRegularChevronsUp")]
use leptos::*;
#[cfg(feature = "BiRegularChevronsUp")]
///This icon requires the feature `BiRegularChevronsUp` to be enabled.
#[component]
pub fn ChevronsUp(
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
        "m6.293 11.293 1.414 1.414L12 8.414l4.293 4.293 1.414-1.414L12 5.586z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m6.293 16.293 1.414 1.414L12 13.414l4.293 4.293 1.414-1.414L12 10.586z" /> <
        title > { title } < / title > < / svg >
    }
}
