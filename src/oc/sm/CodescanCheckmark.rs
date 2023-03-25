#[cfg(feature = "OcSmCodescanCheckmark")]
use leptos::*;
#[cfg(feature = "OcSmCodescanCheckmark")]
///This icon requires the feature `OcSmCodescanCheckmark` to be enabled.
#[component]
pub fn CodescanCheckmark(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M10.28 6.28a.75.75 0 1 0-1.06-1.06L6.25 8.19l-.97-.97a.75.75 0 0 0-1.06 1.06l1.5 1.5a.75.75 0 0 0 1.06 0l3.5-3.5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.5 15a7.5 7.5 0 1 1 5.807-2.754l2.473 2.474a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215l-2.474-2.473A7.472 7.472 0 0 1 7.5 15Zm0-13.5a6 6 0 1 0 4.094 10.386.748.748 0 0 1 .293-.292 6.002 6.002 0 0 0 1.117-6.486A6.002 6.002 0 0 0 7.5 1.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
