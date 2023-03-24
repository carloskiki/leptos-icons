#[cfg(feature = "SiFoursquarecityguide")]
use leptos::*;
#[cfg(feature = "SiFoursquarecityguide")]
///This icon requires the feature `SiFoursquarecityguide` to be enabled.
#[component]
pub fn Foursquarecityguide(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.727 3.465l-.535 2.799c-.064.303-.445.623-.801.623H11.41c-.562 0-.963.391-.963.945v.614c0 .569.405.96.966.96h4.23c.395 0 .785.436.697.855l-.535 2.76c-.051.24-.314.63-.785.63h-3.457c-.63 0-.818.091-1.239.601-.42.524-4.206 5.069-4.206 5.069-.037.045-.074.029-.074-.015V3.42c0-.359.311-.78.776-.78h10.274c.375 0 .73.356.633.821v.004zm.451 10.98c.145-.578 1.746-8.784 2.281-11.385M18.486 0H5.683C3.918 0 3.4 1.328 3.4 2.164v20.34c0 .94.504 1.291.789 1.405.284.117 1.069.214 1.541-.328 0 0 6.044-7.014 6.146-7.117.165-.157.165-.157.315-.157h3.914c1.65 0 1.906-1.17 2.086-1.86.15-.569 1.754-8.774 2.279-11.385C20.875 1.08 20.365 0 18.49 0h-.004z"
        /> < title > { title } < / title > < / svg >
    }
}
