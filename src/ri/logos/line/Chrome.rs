#[cfg(feature = "RiLogosLineChrome")]
use leptos::*;
#[cfg(feature = "RiLogosLineChrome")]
///This icon requires the feature `RiLogosLineChrome` to be enabled.
#[component]
pub fn Chrome(
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
        "M10.365 19.833l1.93-3.342a4.499 4.499 0 0 1-4.234-2.315L4.794 8.52a8.003 8.003 0 0 0 5.57 11.313zm2.225.146A8 8 0 0 0 19.602 9.5h-3.86A4.48 4.48 0 0 1 16.5 12a4.48 4.48 0 0 1-.642 2.318l-3.268 5.66zm1.553-6.691l.022-.038a2.5 2.5 0 1 0-4.354-.042l.024.042a2.499 2.499 0 0 0 4.308.038zm-8.108-6.62l1.929 3.34A4.5 4.5 0 0 1 12 7.5h6.615A7.992 7.992 0 0 0 12 4a7.98 7.98 0 0 0-5.965 2.669zM12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
