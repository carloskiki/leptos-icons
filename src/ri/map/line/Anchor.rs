#[cfg(feature = "RiMapLineAnchor")]
use leptos::*;
#[cfg(feature = "RiMapLineAnchor")]
///This icon requires the feature `RiMapLineAnchor` to be enabled.
#[component]
pub fn Anchor(
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
        "M2.05 11H7v2H4.062A8.004 8.004 0 0 0 11 19.938V9.874A4.002 4.002 0 0 1 12 2a4 4 0 0 1 1 7.874v10.064A8.004 8.004 0 0 0 19.938 13H17v-2h4.95c.033.329.05.663.05 1 0 5.523-4.477 10-10 10S2 17.523 2 12c0-.337.017-.671.05-1zM12 8a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
