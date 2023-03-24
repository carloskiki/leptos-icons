#[cfg(feature = "RiBusinessFillPrinterCloud")]
use leptos::*;
#[cfg(feature = "RiBusinessFillPrinterCloud")]
///This icon requires the feature `RiBusinessFillPrinterCloud` to be enabled.
#[component]
pub fn PrinterCloud(
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
        "M10.566 17A4.737 4.737 0 0 0 10 19.25c0 1.023.324 1.973.877 2.75H7v-5h3.566zm6.934-4a3.5 3.5 0 0 1 3.5 3.5l-.001.103a2.75 2.75 0 0 1-.581 5.392L20.25 22h-5.5l-.168-.005a2.75 2.75 0 0 1-.579-5.392L14 16.5a3.5 3.5 0 0 1 3.5-3.5zM21 8a1 1 0 0 1 1 1l.001 4.346A5.482 5.482 0 0 0 17.5 11l-.221.004A5.503 5.503 0 0 0 12.207 15H5v5H3a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1h18zM8 10H5v2h3v-2zm9-8a1 1 0 0 1 1 1v3H6V3a1 1 0 0 1 1-1h10z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
