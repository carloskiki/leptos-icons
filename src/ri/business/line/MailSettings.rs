#[cfg(feature = "RiBusinessLineMailSettings")]
use leptos::*;
#[cfg(feature = "RiBusinessLineMailSettings")]
///This icon requires the feature `RiBusinessLineMailSettings` to be enabled.
#[component]
pub fn MailSettings(
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
        "M20 7.238l-7.928 7.1L4 7.216V19h10v2H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v9h-2V7.238zM19.501 5H4.511l7.55 6.662L19.502 5zM17.05 19.548a3.017 3.017 0 0 1 0-1.096l-1.014-.586 1-1.732 1.014.586c.278-.238.599-.425.95-.55V15h2v1.17c.351.125.672.312.95.55l1.014-.586 1 1.732-1.014.586a3.017 3.017 0 0 1 0 1.096l1.014.586-1 1.732-1.014-.586a2.997 2.997 0 0 1-.95.55V23h-2v-1.17a2.997 2.997 0 0 1-.95-.55l-1.014.586-1-1.732 1.014-.586zM20 20a1 1 0 1 0 0-2 1 1 0 0 0 0 2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
