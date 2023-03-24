#[cfg(feature = "RiLogosLineDrive")]
use leptos::*;
#[cfg(feature = "RiLogosLineDrive")]
///This icon requires the feature `RiLogosLineDrive` to be enabled.
#[component]
pub fn Drive(
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
        "M9.097 6.15L4.31 14.443l1.755 3.032 4.785-8.29L9.097 6.15zm-1.3 12.324h9.568l1.751-3.034H9.55l-1.752 3.034zm11.314-5.034l-4.786-8.29H10.83l4.787 8.29h3.495zM8.52 3.15h6.96L22 14.444l-3.48 6.03H5.49L2 14.444 8.52 3.15zm3.485 8.036l-1.302 2.254h2.603l-1.301-2.254z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
