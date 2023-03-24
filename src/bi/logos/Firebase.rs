#[cfg(feature = "BiLogosFirebase")]
use leptos::*;
#[cfg(feature = "BiLogosFirebase")]
///This icon requires the feature `BiLogosFirebase` to be enabled.
#[component]
pub fn Firebase(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M5.239 15.063 7.21 2.381a.453.453 0 0 1 .847-.145l2.12 3.979-4.938 8.848zM19.24 18.14 17.363 6.469a.454.454 0 0 0-.766-.246L4.76 18.14l6.55 3.691c.411.23.912.23 1.323 0l6.607-3.691zM13.917 7.955 12.4 5.052a.452.452 0 0 0-.8 0L4.939 16.989l8.978-9.034z"
        /> < title > { title } < / title > < / svg >
    }
}
