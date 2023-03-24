#[cfg(feature = "TbOld")]
use leptos::*;
#[cfg(feature = "TbOld")]
///This icon requires the feature `TbOld` to be enabled.
#[component]
pub fn Old(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-old" width
        = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M11 21l-1 -4l-2 -3v-6" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M5 14l-1 -3l4 -3l3 2l3 .5" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 4m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 17l-2 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 21v-8.5a1.5 1.5 0 0 1 3 0v.5" /> < title >
        { title } < / title > < / svg >
    }
}
