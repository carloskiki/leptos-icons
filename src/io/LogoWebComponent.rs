#[cfg(feature = "IoLogoWebComponent")]
use leptos::*;
#[cfg(feature = "IoLogoWebComponent")]
///This icon requires the feature `IoLogoWebComponent` to be enabled.
#[component]
pub fn LogoWebComponent(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polygon xmlns = "http://www.w3.org/2000/svg"
        points = "179.9 388 179.9 388 103.74 256 179.9 388" style = "fill:none" /><
        polygon xmlns = "http://www.w3.org/2000/svg" points =
        "179.9 388 332.11 388 408.26 256 332.11 124 179.9 124 103.74 256 179.9 388" style
        = "fill:none" />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "103.74 256 179.9 124 179.9 124 103.74 256" style = "fill:none" />< polygon xmlns
        = "http://www.w3.org/2000/svg" points =
        "496 256 376 48 239.74 48 195.9 124 332.11 124 408.26 256 332.11 388 195.9 388 239.74 464 376 464 496 256"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "179.9 388 103.74 256 179.9 124 179.9 124 223.74 48 136 48 16 256 136 464 223.74 464 179.9 388 179.9 388"
        /> < title > { title } < / title > < / svg >
    }
}
