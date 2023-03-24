#[cfg(feature = "TbGiftCard")]
use leptos::*;
#[cfg(feature = "TbGiftCard")]
///This icon requires the feature `TbGiftCard` to be enabled.
#[component]
pub fn GiftCard(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-gift-card"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 5m0 3a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 16l3 -3l3 3" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M8 13c-.789 0 -2 -.672 -2 -1.5s.711 -1.5 1.5 -1.5c1.128 -.02 2.077 1.17 2.5 3c.423 -1.83 1.372 -3.02 2.5 -3c.789 0 1.5 .672 1.5 1.5s-1.211 1.5 -2 1.5h-4z"
        /> < title > { title } < / title > < / svg >
    }
}
