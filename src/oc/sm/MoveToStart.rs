#[cfg(feature = "OcSmMoveToStart")]
use leptos::*;
#[cfg(feature = "OcSmMoveToStart")]
///This icon requires the feature `OcSmMoveToStart` to be enabled.
#[component]
pub fn MoveToStart(
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
        "M5.22 7.47a.749.749 0 0 0 0 1.06l3.75 3.75a.749.749 0 1 0 1.06-1.06L7.561 8.75h6.689a.75.75 0 0 0 0-1.5H7.561l2.469-2.47a.749.749 0 1 0-1.06-1.06L5.22 7.47ZM3 3.75a.75.75 0 0 0-1.5 0v8.5a.75.75 0 0 0 1.5 0v-8.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
