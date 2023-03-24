#[cfg(feature = "RiMediaLinePhoneCamera")]
use leptos::*;
#[cfg(feature = "RiMediaLinePhoneCamera")]
///This icon requires the feature `RiMediaLinePhoneCamera` to be enabled.
#[component]
pub fn PhoneCamera(
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
        "M14.803 4a5.96 5.96 0 0 0-.72 2H3v12h18v-5.083a5.96 5.96 0 0 0 2-.72V19c0 .553-.44 1.001-1.002 1.001H2.002A1 1 0 0 1 1 19V5c0-.552.44-1 1.002-1h12.8zM20 9a2 2 0 1 0 0-4 2 2 0 0 0 0 4zm0 2a4 4 0 1 1 0-8 4 4 0 0 1 0 8zm-2 2h2v3h-2v-3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
