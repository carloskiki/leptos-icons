#[cfg(feature = "RiDeviceFillBaseStation")]
use leptos::*;
#[cfg(feature = "RiDeviceFillBaseStation")]
///This icon requires the feature `RiDeviceFillBaseStation` to be enabled.
#[component]
pub fn BaseStation(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M12 13l6 9H6l6-9zm-1.06-2.44a1.5 1.5 0 1 1 2.12-2.12 1.5 1.5 0 0 1-2.12 2.12zM5.281 2.783l1.415 1.415a7.5 7.5 0 0 0 0 10.606l-1.415 1.415a9.5 9.5 0 0 1 0-13.436zm13.436 0a9.5 9.5 0 0 1 0 13.436l-1.415-1.415a7.5 7.5 0 0 0 0-10.606l1.415-1.415zM8.11 5.611l1.414 1.414a3.5 3.5 0 0 0 0 4.95l-1.414 1.414a5.5 5.5 0 0 1 0-7.778zm7.778 0a5.5 5.5 0 0 1 0 7.778l-1.414-1.414a3.5 3.5 0 0 0 0-4.95l1.414-1.414z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
