#[cfg(feature = "TbHandMove")]
use leptos::*;
#[cfg(feature = "TbHandMove")]
///This icon requires the feature `TbHandMove` to be enabled.
#[component]
pub fn HandMove(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-hand-move"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 13v-8.5a1.5 1.5 0 0 1 3 0v7.5" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M11 11.5v-2a1.5 1.5 0 0 1 3 0v2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 10.5a1.5 1.5 0 0 1 3 0v1.5" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M17 11.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7l-.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.541 5.594a13.487 13.487 0 0 1 2.46 -1.427" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 3.458c1.32 .354 2.558 .902 3.685 1.612" />
        < title > { title } < / title > < / svg >
    }
}
