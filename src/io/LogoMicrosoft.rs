#[cfg(feature = "IoLogoMicrosoft")]
use leptos::*;
#[cfg(feature = "IoLogoMicrosoft")]
///This icon requires the feature `IoLogoMicrosoft` to be enabled.
#[component]
pub fn LogoMicrosoft(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M31.87,30.58H244.7V243.39H31.87Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M266.89,30.58H479.7V243.39H266.89Z" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M31.87,265.61H244.7v212.8H31.87Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M266.89,265.61H479.7v212.8H266.89Z" /> < title > { title } < / title > < / svg >
    }
}
