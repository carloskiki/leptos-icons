#[cfg(feature = "RiLogosLineSwitch")]
use leptos::*;
#[cfg(feature = "RiLogosLineSwitch")]
///This icon requires the feature `RiLogosLineSwitch` to be enabled.
#[component]
pub fn Switch(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M12 3v18H7.6A4.6 4.6 0 0 1 3 16.4V7.6A4.6 4.6 0 0 1 7.6 3H12zm-2 2H7.6A2.6 2.6 0 0 0 5 7.6v8.8A2.6 2.6 0 0 0 7.6 19H10V5zm-2.5 5a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zM14 3h2.4A4.6 4.6 0 0 1 21 7.6v8.8a4.6 4.6 0 0 1-4.6 4.6H14V3zm3 11.7a1.8 1.8 0 1 0 0-3.6 1.8 1.8 0 0 0 0 3.6z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
