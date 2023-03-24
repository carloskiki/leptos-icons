#[cfg(feature = "RiHealthLineInfraredThermometer")]
use leptos::*;
#[cfg(feature = "RiHealthLineInfraredThermometer")]
///This icon requires the feature `RiHealthLineInfraredThermometer` to be enabled.
#[component]
pub fn InfraredThermometer(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M21 2v9h-3.001L18 12c0 2.21-1.79 4-4 4h-1.379l-.613 3.111.911 1.321c.314.455.2 1.078-.255 1.391-.167.115-.365.177-.568.177H3l2.313-10.024L3 11l4-9h14zm-2 2H8.3L5.655 9.95l1.985.837L5.514 20h4.678l-.309-.448L11.96 9H19V4zm-3.001 7h-2.394l-.591 3H14c1.105 0 2-.895 2-2l-.001-1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
