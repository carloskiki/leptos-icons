#[cfg(feature = "RiOthersFillPingPong")]
use leptos::*;
#[cfg(feature = "RiOthersFillPingPong")]
///This icon requires the feature `RiOthersFillPingPong` to be enabled.
#[component]
pub fn PingPong(
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
        "M11.5 2a9.5 9.5 0 0 1 9.5 9.5 9.46 9.46 0 0 1-1.003 4.254l2.463 2.464a1 1 0 0 1 0 1.414l-2.828 2.828a1 1 0 0 1-1.414 0l-2.464-2.463A9.46 9.46 0 0 1 11.5 21a9.5 9.5 0 0 1 0-19zm5.303 13.388l-1.414 1.414 3.536 3.535 1.414-1.414-3.536-3.535zm1.864-6.105l-9.384 9.384c.7.216 1.445.333 2.217.333a7.48 7.48 0 0 0 2.74-.516l-.972-.974a1 1 0 0 1 0-1.414l2.828-2.828a1 1 0 0 1 1.414 0l.974.972A7.48 7.48 0 0 0 19 11.5c0-.772-.117-1.516-.333-2.217z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
