#[cfg(feature = "RiMapFillRocket")]
use leptos::*;
#[cfg(feature = "RiMapFillRocket")]
///This icon requires the feature `RiMapFillRocket` to be enabled.
#[component]
pub fn Rocket(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M5.33 15.929A13.064 13.064 0 0 1 5 13c0-5.088 2.903-9.436 7-11.182C16.097 3.564 19 7.912 19 13c0 1.01-.114 1.991-.33 2.929l2.02 1.796a.5.5 0 0 1 .097.63l-2.458 4.096a.5.5 0 0 1-.782.096l-2.254-2.254a1 1 0 0 0-.707-.293H9.414a1 1 0 0 0-.707.293l-2.254 2.254a.5.5 0 0 1-.782-.096l-2.458-4.095a.5.5 0 0 1 .097-.631l2.02-1.796zM12 13a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
