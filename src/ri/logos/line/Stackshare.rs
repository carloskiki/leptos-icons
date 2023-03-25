#[cfg(feature = "RiLogosLineStackshare")]
use leptos::*;
#[cfg(feature = "RiLogosLineStackshare")]
///This icon requires the feature `RiLogosLineStackshare` to be enabled.
#[component]
pub fn Stackshare(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M9.536 13H7.329c-.412 1.166-1.523 2-2.829 2-1.657 0-3-1.343-3-3s1.343-3 3-3c1.306 0 2.418.835 2.83 2h2.206L13 5h3.17c.412-1.165 1.524-2 2.83-2 1.657 0 3 1.343 3 3s-1.343 3-3 3c-1.306 0-2.417-.834-2.829-2h-2.017l-2.886 4.999L14.155 17h2.016c.411-1.165 1.523-2 2.829-2 1.657 0 3 1.343 3 3s-1.343 3-3 3c-1.306 0-2.417-.834-2.829-2H13l-3.464-6zM19 17c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1zM4.5 11c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1zM19 5c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
