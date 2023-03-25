#[cfg(feature = "BiRegularMeh")]
use leptos::*;
#[cfg(feature = "BiRegularMeh")]
///This icon requires the feature `BiRegularMeh` to be enabled.
#[component]
pub fn Meh(
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
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "8.5" cy = "10.5" r = "1.5"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "15.493" cy = "10.493" r =
        "1.493" />< path xmlns = "http://www.w3.org/2000/svg" d = "M7.974 15H16v2H7.974z"
        /> < title > { title } < / title > < / svg >
    }
}
