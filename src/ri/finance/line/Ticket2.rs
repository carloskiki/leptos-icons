#[cfg(feature = "RiFinanceLineTicket2")]
use leptos::*;
#[cfg(feature = "RiFinanceLineTicket2")]
///This icon requires the feature `RiFinanceLineTicket2` to be enabled.
#[component]
pub fn Ticket2(
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
        "M21 3a1 1 0 0 1 1 1v5.5a2.5 2.5 0 1 0 0 5V20a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1v-5.5a2.5 2.5 0 1 0 0-5V4a1 1 0 0 1 1-1h18zm-1 2H4v2.968l.156.081a4.5 4.5 0 0 1 2.34 3.74L6.5 12a4.499 4.499 0 0 1-2.344 3.95L4 16.032V19h16v-2.969l-.156-.08a4.5 4.5 0 0 1-2.34-3.74L17.5 12c0-1.704.947-3.187 2.344-3.95L20 7.967V5zm-4 4v6H8V9h8z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
