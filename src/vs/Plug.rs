#[cfg(feature = "VsPlug")]
use leptos::*;
#[cfg(feature = "VsPlug")]
///This icon requires the feature `VsPlug` to be enabled.
#[component]
pub fn Plug(
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
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M7 1H6v3H4.5l-.5.5V8a4 4 0 0 0 3.5 3.969V15h1v-3.031A4 4 0 0 0 12 8V4.5l-.5-.5H10V1H9v3H7V1zm3.121 9.121A3 3 0 0 1 5 8V5h6v3a3 3 0 0 1-.879 2.121z"
        /> < title > { title } < / title > < / svg >
    }
}
