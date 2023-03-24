#[cfg(feature = "RiWeatherFillRainy")]
use leptos::*;
#[cfg(feature = "RiWeatherFillRainy")]
///This icon requires the feature `RiWeatherFillRainy` to be enabled.
#[component]
pub fn Rainy(
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
        "M15.86 18l-3.153-3.153a1 1 0 0 0-1.414 0L8.18 17.96A8.001 8.001 0 1 1 15.98 6.087 6 6 0 1 1 17 18h-1.139zm-5.628.732L12 16.964l1.768 1.768a2.5 2.5 0 1 1-3.536 0z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
