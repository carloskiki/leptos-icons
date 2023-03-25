#[cfg(feature = "BiRegularChevronsDown")]
use leptos::*;
#[cfg(feature = "BiRegularChevronsDown")]
///This icon requires the feature `BiRegularChevronsDown` to be enabled.
#[component]
pub fn ChevronsDown(
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
        "m12 15.586-4.293-4.293-1.414 1.414L12 18.414l5.707-5.707-1.414-1.414z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m17.707 7.707-1.414-1.414L12 10.586 7.707 6.293 6.293 7.707 12 13.414z" /> <
        title > { title } < / title > < / svg >
    }
}
