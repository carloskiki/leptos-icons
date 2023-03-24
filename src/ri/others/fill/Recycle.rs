#[cfg(feature = "RiOthersFillRecycle")]
use leptos::*;
#[cfg(feature = "RiOthersFillRecycle")]
///This icon requires the feature `RiOthersFillRecycle` to be enabled.
#[component]
pub fn Recycle(
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
        "M19.562 12.098l1.531 2.652c.967 1.674.393 3.815-1.28 4.781-.533.307-1.136.469-1.75.469H16v2l-5-3.5 5-3.5v2h2.062c.088 0 .174-.023.25-.067.213-.123.301-.378.221-.601l-.038-.082-1.531-2.652 2.598-1.5zM7.737 9.384l.53 6.08-1.73-1-1.032 1.786c-.044.076-.067.162-.067.25 0 .245.177.45.41.492l.09.008H9v3H5.938c-1.933 0-3.5-1.567-3.5-3.5 0-.614.162-1.218.469-1.75l1.031-1.786-1.732-1 5.53-2.58zm6.013-6.415c.532.307.974.749 1.281 1.281l1.03 1.786 1.733-1-.53 6.08-5.532-2.58 1.732-1-1.031-1.786c-.044-.076-.107-.14-.183-.183-.213-.123-.478-.072-.631.11l-.052.073-1.53 2.652-2.599-1.5 1.53-2.652c.967-1.674 3.108-2.248 4.782-1.281z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
