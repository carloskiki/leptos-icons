#[cfg(feature = "RiBusinessLineCreativeCommons")]
use leptos::*;
#[cfg(feature = "RiBusinessLineCreativeCommons")]
///This icon requires the feature `RiBusinessLineCreativeCommons` to be enabled.
#[component]
pub fn CreativeCommons(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2zm0 2a8 8 0 1 0 0 16 8 8 0 0 0 0-16zM9 8c1.105 0 2.105.448 2.829 1.173l-1.414 1.414a2 2 0 1 0-.001 2.828l1.414 1.413A4.001 4.001 0 0 1 5 12c0-2.208 1.792-4 4-4zm7 0c1.105 0 2.105.448 2.829 1.173l-1.414 1.414a2 2 0 1 0-.001 2.828l1.414 1.413A4.001 4.001 0 0 1 12 12c0-2.208 1.792-4 4-4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
