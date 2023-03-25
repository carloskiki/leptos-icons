#[cfg(feature = "OcLgDiamond")]
use leptos::*;
#[cfg(feature = "OcLgDiamond")]
///This icon requires the feature `OcLgDiamond` to be enabled.
#[component]
pub fn Diamond(
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
        "M1.527 13.237a1.75 1.75 0 0 1 0-2.474l9.272-9.273a1.75 1.75 0 0 1 2.475 0l9.272 9.273a1.75 1.75 0 0 1 0 2.474l-9.272 9.272a1.75 1.75 0 0 1-2.475 0Zm1.06-1.414a.25.25 0 0 0 0 .354l9.273 9.272a.25.25 0 0 0 .353 0l9.272-9.272a.25.25 0 0 0 0-.354l-9.272-9.272a.25.25 0 0 0-.353 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
