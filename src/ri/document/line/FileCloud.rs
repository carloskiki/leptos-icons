#[cfg(feature = "RiDocumentLineFileCloud")]
use leptos::*;
#[cfg(feature = "RiDocumentLineFileCloud")]
///This icon requires the feature `RiDocumentLineFileCloud` to be enabled.
#[component]
pub fn FileCloud(
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
        "M14.997 2L21 8l.001 4.26a5.471 5.471 0 0 0-2-1.053L19 9h-5V4H5v16h5.06a4.73 4.73 0 0 0 .817 2H3.993a.993.993 0 0 1-.986-.876L3 21.008V2.992c0-.498.387-.927.885-.985L4.002 2h10.995zM17.5 13a3.5 3.5 0 0 1 3.5 3.5l-.001.103a2.75 2.75 0 0 1-.581 5.392L20.25 22h-5.5l-.168-.005a2.75 2.75 0 0 1-.579-5.392L14 16.5a3.5 3.5 0 0 1 3.5-3.5zm0 2a1.5 1.5 0 0 0-1.473 1.215l-.02.14L16 16.5v1.62l-1.444.406a.75.75 0 0 0 .08 1.466l.109.008h5.51a.75.75 0 0 0 .19-1.474l-1.013-.283L19 18.12V16.5l-.007-.144A1.5 1.5 0 0 0 17.5 15z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
