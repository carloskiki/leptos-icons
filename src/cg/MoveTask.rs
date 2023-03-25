#[cfg(feature = "CgMoveTask")]
use leptos::*;
#[cfg(feature = "CgMoveTask")]
///This icon requires the feature `CgMoveTask` to be enabled.
#[component]
pub fn MoveTask(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.9641 7H10.9641V9H18.9641V7Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 8.82864V15.1714L9.9642 12L6 8.82864Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.9641 11H10.9641V13H18.9641V11Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.9641 15H18.9641V17H10.9641V15Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
