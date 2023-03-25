#[cfg(feature = "IoEllipsisVerticalCircleSharp")]
use leptos::*;
#[cfg(feature = "IoEllipsisVerticalCircleSharp")]
///This icon requires the feature `IoEllipsisVerticalCircleSharp` to be enabled.
#[component]
pub fn EllipsisVerticalCircleSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M464,256c0-114.87-93.13-208-208-208S48,141.13,48,256s93.13,208,208,208S464,370.87,464,256ZM230,166a26,26,0,1,1,26,26A26,26,0,0,1,230,166Zm0,90a26,26,0,1,1,26,26A26,26,0,0,1,230,256Zm0,90a26,26,0,1,1,26,26A26,26,0,0,1,230,346Z"
        /> < title > { title } < / title > < / svg >
    }
}
