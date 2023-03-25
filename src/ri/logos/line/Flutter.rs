#[cfg(feature = "RiLogosLineFlutter")]
use leptos::*;
#[cfg(feature = "RiLogosLineFlutter")]
///This icon requires the feature `RiLogosLineFlutter` to be enabled.
#[component]
pub fn Flutter(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M14.597 10.684h2.828l-5.657 5.658 5.657 5.656h-2.828L8.94 16.34l5.657-5.657zm-.194-8.68h2.829L5.918 13.318l-1.414-1.414 9.9-9.9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
