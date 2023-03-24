#[cfg(feature = "RiMapFillBarricade")]
use leptos::*;
#[cfg(feature = "RiMapFillBarricade")]
///This icon requires the feature `RiMapFillBarricade` to be enabled.
#[component]
pub fn Barricade(
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
        "M19.556 19H21v2H3v-2h1.444l.89-4h13.333l.889 4zM17.333 9l.89 4H5.777l.889-4h10.666zm-.444-2H7.11l.715-3.217A1 1 0 0 1 8.802 3h6.396a1 1 0 0 1 .976.783L16.889 7z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
