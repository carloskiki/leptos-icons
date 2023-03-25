#[cfg(feature = "OcLgBeaker")]
use leptos::*;
#[cfg(feature = "OcLgBeaker")]
///This icon requires the feature `OcLgBeaker` to be enabled.
#[component]
pub fn Beaker(
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
        "M8 8.807V3.5h-.563a.75.75 0 0 1 0-1.5h9.125a.75.75 0 0 1 0 1.5H16v5.307l5.125 9.301c.964 1.75-.302 3.892-2.299 3.892H5.174c-1.998 0-3.263-2.142-2.3-3.892ZM4.189 18.832a1.123 1.123 0 0 0 .985 1.668h13.652a1.123 1.123 0 0 0 .985-1.668L17.7 15H6.3ZM14.5 3.5h-5V9a.75.75 0 0 1-.093.362L7.127 13.5h9.746l-2.28-4.138A.75.75 0 0 1 14.5 9Z"
        /> < title > { title } < / title > < / svg >
    }
}
