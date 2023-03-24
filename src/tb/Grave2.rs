#[cfg(feature = "TbGrave2")]
use leptos::*;
#[cfg(feature = "TbGrave2")]
///This icon requires the feature `TbGrave2` to be enabled.
#[component]
pub fn Grave2(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-grave-2"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 16.17v-9.17a3 3 0 0 1 3 -3h4a3 3 0 0 1 3 3v9.171" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 7v5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 9h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 21v-2a3 3 0 0 1 3 -3h8a3 3 0 0 1 3 3v2h-14z"
        /> < title > { title } < / title > < / svg >
    }
}
