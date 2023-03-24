#[cfg(feature = "TbDropletFilled2")]
use leptos::*;
#[cfg(feature = "TbDropletFilled2")]
///This icon requires the feature `TbDropletFilled2` to be enabled.
#[component]
pub fn DropletFilled2(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-droplet-filled-2" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.8 11a6 6 0 1 0 10.396 0l-5.197 -8l-5.2 8z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 14h12" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7.305 17.695l3.695 -3.695" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.26 19.74l5.74 -5.74l-5.74 5.74z" /> < title
        > { title } < / title > < / svg >
    }
}
