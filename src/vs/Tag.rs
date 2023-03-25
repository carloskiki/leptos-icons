#[cfg(feature = "VsTag")]
use leptos::*;
#[cfg(feature = "VsTag")]
///This icon requires the feature `VsTag` to be enabled.
#[component]
pub fn Tag(
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
        "M13.2 2H8.017l-.353.146L1 8.81v.707L6.183 14.7h.707l2.215-2.215A4.48 4.48 0 0 0 15.65 9c.027-.166.044-.332.051-.5a4.505 4.505 0 0 0-2-3.74V2.5l-.5-.5zm-.5 2.259A4.504 4.504 0 0 0 11.2 4a.5.5 0 1 0 0 1 3.5 3.5 0 0 1 1.5.338v2.138L8.775 11.4a.506.506 0 0 0-.217.217l-2.022 2.022-4.475-4.476L8.224 3H12.7v1.259zm1 1.792a3.5 3.5 0 0 1 1 2.449 3.438 3.438 0 0 1-.051.5 3.487 3.487 0 0 1-4.793 2.735l3.698-3.698.146-.354V6.051z"
        /> < title > { title } < / title > < / svg >
    }
}
